use crate::domain::{ChatMessage, RuntimeCapabilities, RuntimeStatus};
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{pin::Pin, time::Duration};

pub type RuntimeEventStream =
    Pin<Box<dyn Stream<Item = Result<RuntimeEvent, RuntimeError>> + Send>>;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum RuntimeEvent {
    Progress(Value),
    Token(String),
    Metrics(Value),
    OpenAiChunk(Value),
    Done,
}

pub enum RuntimeCompletion {
    Json(Value),
    Stream(RuntimeEventStream),
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeErrorCode {
    RuntimeUnavailable,
    ModelNotInstalled,
    ModelNotCapable,
    InvalidRequest,
    StructuredOutputInvalid,
    ContextLengthExceeded,
    RequestTimeout,
    RequestCancelled,
    RateLimited,
    UpstreamError,
}

impl RuntimeErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeUnavailable => "runtime_unavailable",
            Self::ModelNotInstalled => "model_not_installed",
            Self::ModelNotCapable => "model_not_capable",
            Self::InvalidRequest => "invalid_request",
            Self::StructuredOutputInvalid => "structured_output_invalid",
            Self::ContextLengthExceeded => "context_length_exceeded",
            Self::RequestTimeout => "request_timeout",
            Self::RequestCancelled => "request_cancelled",
            Self::RateLimited => "rate_limited",
            Self::UpstreamError => "upstream_error",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub code: RuntimeErrorCode,
    pub message: String,
}

impl RuntimeError {
    fn from_reqwest(error: reqwest::Error) -> Self {
        let code = if error.is_timeout() {
            RuntimeErrorCode::RequestTimeout
        } else if error.is_connect() {
            RuntimeErrorCode::RuntimeUnavailable
        } else {
            error
                .status()
                .map(http_error_code)
                .unwrap_or(RuntimeErrorCode::UpstreamError)
        };
        Self {
            code,
            message: "model runtime request failed".into(),
        }
    }

    fn invalid_stream() -> Self {
        Self {
            code: RuntimeErrorCode::UpstreamError,
            message: "model runtime returned an invalid stream frame".into(),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for RuntimeError {}

fn http_error_code(status: StatusCode) -> RuntimeErrorCode {
    match status {
        StatusCode::NOT_FOUND => RuntimeErrorCode::ModelNotInstalled,
        StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => {
            RuntimeErrorCode::RequestTimeout
        }
        StatusCode::TOO_MANY_REQUESTS => RuntimeErrorCode::RateLimited,
        StatusCode::BAD_REQUEST => RuntimeErrorCode::InvalidRequest,
        _ => RuntimeErrorCode::UpstreamError,
    }
}

#[async_trait]
pub trait ModelRuntime: Send + Sync {
    fn id(&self) -> &'static str;
    fn endpoint_id(&self) -> String;
    fn capabilities(&self) -> RuntimeCapabilities;
    async fn status(&self) -> RuntimeStatus;
    async fn install(&self, model: &str) -> Result<RuntimeEventStream, RuntimeError>;
    async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<RuntimeEventStream, RuntimeError>;
    async fn chat_completion(&self, request: &Value) -> Result<RuntimeCompletion, RuntimeError>;
    async fn model_details(&self, model: &str) -> Result<Value, RuntimeError>;
}

#[derive(Clone)]
pub struct OllamaRuntime {
    client: Client,
    pub base_url: String,
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<TagModel>,
}

#[derive(Deserialize)]
struct TagModel {
    name: String,
}

#[derive(Serialize)]
struct PullRequest<'a> {
    model: &'a str,
    stream: bool,
}

#[derive(Serialize)]
struct RuntimeChatRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    stream: bool,
}

impl OllamaRuntime {
    pub fn new(base_url: String) -> Self {
        Self::with_timeout(base_url, 120)
    }

    pub fn with_timeout(base_url: String, timeout_seconds: u64) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(timeout_seconds))
                .build()
                .expect("valid reqwest client"),
            base_url: base_url.trim_end_matches('/').into(),
        }
    }

    async fn checked(&self, request: reqwest::RequestBuilder) -> Result<Response, RuntimeError> {
        request
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(RuntimeError::from_reqwest)
    }
}

#[async_trait]
impl ModelRuntime for OllamaRuntime {
    fn id(&self) -> &'static str {
        "ollama"
    }

    fn endpoint_id(&self) -> String {
        reqwest::Url::parse(&self.base_url)
            .ok()
            .map(|url| {
                let host = url.host_str().unwrap_or("local");
                match url.port() {
                    Some(port) => format!("{}://{host}:{port}", url.scheme()),
                    None => format!("{}://{host}", url.scheme()),
                }
            })
            .unwrap_or_else(|| "local-runtime".into())
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            install: true,
            chat: true,
            chat_completions: true,
            streaming: true,
            structured_output: vec!["json_object".into(), "json_schema".into()],
            embeddings: false,
            model_details: true,
        }
    }

    async fn status(&self) -> RuntimeStatus {
        let result = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await;
        match result {
            Ok(response) if response.status().is_success() => {
                let models = response
                    .json::<TagsResponse>()
                    .await
                    .map(|body| body.models.into_iter().map(|model| model.name).collect())
                    .unwrap_or_default();
                RuntimeStatus {
                    kind: self.id().into(),
                    endpoint_id: self.endpoint_id(),
                    reachable: true,
                    installed_models: models,
                    capabilities: self.capabilities(),
                }
            }
            _ => RuntimeStatus {
                kind: self.id().into(),
                endpoint_id: self.endpoint_id(),
                reachable: false,
                installed_models: vec![],
                capabilities: self.capabilities(),
            },
        }
    }

    async fn install(&self, model: &str) -> Result<RuntimeEventStream, RuntimeError> {
        let response = self
            .checked(
                self.client
                    .post(format!("{}/api/pull", self.base_url))
                    .json(&PullRequest {
                        model,
                        stream: true,
                    }),
            )
            .await?;
        Ok(ollama_ndjson_events(response, OllamaStreamKind::Install))
    }

    async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<RuntimeEventStream, RuntimeError> {
        let response = self
            .checked(
                self.client
                    .post(format!("{}/api/chat", self.base_url))
                    .json(&RuntimeChatRequest {
                        model,
                        messages,
                        stream: true,
                    }),
            )
            .await?;
        Ok(ollama_ndjson_events(response, OllamaStreamKind::Chat))
    }

    async fn chat_completion(&self, request: &Value) -> Result<RuntimeCompletion, RuntimeError> {
        let streaming = request
            .get("stream")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let response = self
            .checked(
                self.client
                    .post(format!("{}/v1/chat/completions", self.base_url))
                    .json(request),
            )
            .await?;
        if streaming {
            Ok(RuntimeCompletion::Stream(openai_sse_events(response)))
        } else {
            response
                .json::<Value>()
                .await
                .map(RuntimeCompletion::Json)
                .map_err(RuntimeError::from_reqwest)
        }
    }

    async fn model_details(&self, model: &str) -> Result<Value, RuntimeError> {
        self.checked(
            self.client
                .post(format!("{}/api/show", self.base_url))
                .json(&json!({"model": model})),
        )
        .await?
        .json()
        .await
        .map_err(RuntimeError::from_reqwest)
    }
}

#[derive(Clone, Copy)]
enum OllamaStreamKind {
    Install,
    Chat,
}

fn ollama_ndjson_events(response: Response, kind: OllamaStreamKind) -> RuntimeEventStream {
    Box::pin(async_stream::stream! {
        let mut upstream = response.bytes_stream();
        let mut buffer = String::new();
        while let Some(chunk) = upstream.next().await {
            let bytes = chunk.map_err(RuntimeError::from_reqwest)?;
            buffer.push_str(&String::from_utf8_lossy(&bytes));
            while let Some(position) = buffer.find('\n') {
                let line = buffer[..position].trim().to_owned();
                buffer.drain(..=position);
                if line.is_empty() { continue; }
                let value = serde_json::from_str::<Value>(&line)
                    .map_err(|_| RuntimeError::invalid_stream())?;
                match kind {
                    OllamaStreamKind::Install => yield Ok(RuntimeEvent::Progress(value)),
                    OllamaStreamKind::Chat => {
                        if let Some(content) = value.pointer("/message/content").and_then(Value::as_str)
                            && !content.is_empty()
                        {
                            yield Ok(RuntimeEvent::Token(content.into()));
                        }
                        if value.get("done").and_then(Value::as_bool) == Some(true) {
                            yield Ok(RuntimeEvent::Metrics(value));
                        }
                    }
                }
            }
        }
        if !buffer.trim().is_empty() {
            let value = serde_json::from_str::<Value>(buffer.trim())
                .map_err(|_| RuntimeError::invalid_stream())?;
            match kind {
                OllamaStreamKind::Install => yield Ok(RuntimeEvent::Progress(value)),
                OllamaStreamKind::Chat => {
                    if let Some(content) = value.pointer("/message/content").and_then(Value::as_str)
                        && !content.is_empty()
                    {
                        yield Ok(RuntimeEvent::Token(content.into()));
                    }
                    if value.get("done").and_then(Value::as_bool) == Some(true) {
                        yield Ok(RuntimeEvent::Metrics(value));
                    }
                }
            }
        }
        yield Ok(RuntimeEvent::Done);
    })
}

fn openai_sse_events(response: Response) -> RuntimeEventStream {
    Box::pin(async_stream::stream! {
        let mut upstream = response.bytes_stream();
        let mut buffer = String::new();
        while let Some(chunk) = upstream.next().await {
            let bytes = chunk.map_err(RuntimeError::from_reqwest)?;
            buffer.push_str(&String::from_utf8_lossy(&bytes));
            while let Some(position) = buffer.find('\n') {
                let line = buffer[..position].trim().to_owned();
                buffer.drain(..=position);
                if let Some(data) = line.strip_prefix("data:").map(str::trim) {
                    if data == "[DONE]" {
                        yield Ok(RuntimeEvent::Done);
                    } else if !data.is_empty() {
                        let value = serde_json::from_str::<Value>(data)
                            .map_err(|_| RuntimeError::invalid_stream())?;
                        yield Ok(RuntimeEvent::OpenAiChunk(value));
                    }
                }
            }
        }
        if !buffer.trim().is_empty() {
            let data = buffer.trim().strip_prefix("data:").map(str::trim).unwrap_or("");
            if data == "[DONE]" {
                yield Ok(RuntimeEvent::Done);
            } else if !data.is_empty() {
                let value = serde_json::from_str::<Value>(data)
                    .map_err(|_| RuntimeError::invalid_stream())?;
                yield Ok(RuntimeEvent::OpenAiChunk(value));
                yield Ok(RuntimeEvent::Done);
            }
        }
    })
}
