//! Optional local SentenceTransformers service, separate from the chat runtime.
use crate::runtime::{RuntimeError, RuntimeErrorCode};
use reqwest::{Client, Url};
use serde_json::{Value, json};
use std::{net::IpAddr, time::Duration};

pub const MODEL: &str = "google/embeddinggemma-2";

#[derive(Clone)]
pub struct EmbeddingRuntime {
    client: Client,
    url: String,
    token: String,
}

impl EmbeddingRuntime {
    pub fn new(endpoint: &str, token: &str) -> Result<Self, String> {
        let url = Url::parse(endpoint).map_err(|_| "invalid MJ_EMBEDDING_URL")?;
        let loopback = url.host_str().is_some_and(|host| {
            host.trim_matches(['[', ']'])
                .parse::<IpAddr>()
                .is_ok_and(|address| address.is_loopback())
        });
        if url.scheme() != "http"
            || !loopback
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/"
        {
            return Err("MJ_EMBEDDING_URL must be an HTTP loopback IP origin".into());
        }
        Ok(Self {
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .redirect(reqwest::redirect::Policy::none())
                .no_proxy()
                .build()
                .map_err(|error| error.to_string())?,
            url: endpoint.trim_end_matches('/').into(),
            token: token.into(),
        })
    }

    pub async fn ready(&self) -> bool {
        match self
            .client
            .get(format!("{}/health", self.url))
            .bearer_auth(&self.token)
            .timeout(Duration::from_secs(2))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                response.json::<Value>().await.is_ok_and(|body| {
                    body.get("model").and_then(Value::as_str) == Some(MODEL)
                        && body.get("ready").and_then(Value::as_bool) == Some(true)
                })
            }
            _ => false,
        }
    }

    pub async fn embed(&self, request: &Value) -> Result<Value, RuntimeError> {
        let response = self
            .client
            .post(format!("{}/v1/embeddings", self.url))
            .bearer_auth(&self.token)
            .json(request)
            .send()
            .await
            .map_err(|error| RuntimeError {
                code: if error.is_timeout() {
                    RuntimeErrorCode::RequestTimeout
                } else {
                    RuntimeErrorCode::RuntimeUnavailable
                },
                message: "local embedding service unavailable".into(),
            })?;
        let status = response.status();
        let body: Value = response.json().await.map_err(|_| upstream_error())?;
        if !status.is_success() {
            let code = match status.as_u16() {
                400 => RuntimeErrorCode::InvalidRequest,
                413 => RuntimeErrorCode::ContextLengthExceeded,
                429 => RuntimeErrorCode::RateLimited,
                503 => RuntimeErrorCode::RuntimeUnavailable,
                _ => RuntimeErrorCode::UpstreamError,
            };
            return Err(RuntimeError {
                code,
                message: "embedding service rejected the request".into(),
            });
        }
        let count = request["input"].as_array().map_or(1, Vec::len);
        let dimensions = request
            .get("dimensions")
            .and_then(Value::as_u64)
            .unwrap_or(768) as usize;
        validate_response(&body, count, dimensions)?;
        Ok(body)
    }
}

fn upstream_error() -> RuntimeError {
    RuntimeError {
        code: RuntimeErrorCode::UpstreamError,
        message: "embedding service returned invalid vectors".into(),
    }
}

fn validate_response(body: &Value, count: usize, dimensions: usize) -> Result<(), RuntimeError> {
    let valid = body["model"] == MODEL
        && body["object"] == "list"
        && body["data"].as_array().is_some_and(|rows| {
            rows.len() == count
                && rows.iter().enumerate().all(|(index, row)| {
                    row["index"].as_u64() == Some(index as u64)
                        && row["embedding"].as_array().is_some_and(|vector| {
                            vector.len() == dimensions
                                && vector
                                    .iter()
                                    .all(|v| v.as_f64().is_some_and(f64::is_finite))
                                && (vector
                                    .iter()
                                    .filter_map(Value::as_f64)
                                    .map(|v| v * v)
                                    .sum::<f64>()
                                    - 1.0)
                                    .abs()
                                    < 0.01
                        })
                })
        });
    if valid { Ok(()) } else { Err(upstream_error()) }
}

pub fn model_descriptor() -> Value {
    json!({"id": MODEL, "object": "model", "owned_by": "google",
    "mj": {"provider": "sentence-transformers", "capabilities": {
        "embeddings": true, "chat": false, "chat_completions": false,
        "modalities": ["text"], "dimensions": [128, 256, 512, 768], "context_tokens": 8192
    }}})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prevents_remote_endpoints_and_credentials() {
        for endpoint in [
            "https://example.com",
            "http://localhost:8000",
            "http://127.0.0.1/path",
            "http://secret@127.0.0.1",
            "http://127.0.0.1?token=secret",
        ] {
            assert!(EmbeddingRuntime::new(endpoint, "test").is_err());
        }
        assert!(EmbeddingRuntime::new("http://127.0.0.1:3211", "test").is_ok());
        assert!(EmbeddingRuntime::new("http://[::1]:3211", "test").is_ok());
    }

    #[test]
    fn rejects_mismatched_or_unnormalized_vectors() {
        let mut body = json!({"model": MODEL, "object":"list", "data":[
            {"index":0, "embedding":[1.0, 0.0]}
        ]});
        assert!(validate_response(&body, 1, 2).is_ok());
        assert!(validate_response(&body, 1, 3).is_err());
        assert!(validate_response(&body, 2, 2).is_err());
        body["data"][0]["embedding"] = json!([0.0, 0.0]);
        assert!(validate_response(&body, 1, 2).is_err());
    }
}
