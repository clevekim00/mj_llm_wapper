use crate::domain::{ChatMessage, RuntimeStatus};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

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
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').into(),
        }
    }

    pub async fn status(&self) -> RuntimeStatus {
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
                    kind: "ollama",
                    base_url: self.base_url.clone(),
                    reachable: true,
                    installed_models: models,
                }
            }
            _ => RuntimeStatus {
                kind: "ollama",
                base_url: self.base_url.clone(),
                reachable: false,
                installed_models: vec![],
            },
        }
    }

    pub async fn pull(&self, model: &str) -> Result<Response, reqwest::Error> {
        self.client
            .post(format!("{}/api/pull", self.base_url))
            .json(&PullRequest {
                model,
                stream: true,
            })
            .send()
            .await?
            .error_for_status()
    }

    pub async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(format!("{}/api/chat", self.base_url))
            .json(&RuntimeChatRequest {
                model,
                messages,
                stream: true,
            })
            .send()
            .await?
            .error_for_status()
    }
}
