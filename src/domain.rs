use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub os: String,
    pub architecture: String,
    pub logical_cpus: usize,
    pub total_memory_bytes: Option<u64>,
    pub runtime_reachable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelArtifact {
    pub id: String,
    pub family: String,
    pub display_name: String,
    pub runtime_model: String,
    pub parameter_label: String,
    pub download_bytes: u64,
    pub estimated_peak_ram_bytes: u64,
    pub context_tokens: u32,
    pub capabilities: Vec<String>,
    pub platforms: Vec<String>,
    pub lifecycle: String,
    pub source_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRecommendation {
    #[serde(flatten)]
    pub artifact: ModelArtifact,
    pub fit: String,
    pub fit_score: u8,
    pub reason: String,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub updated_at_epoch_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub conversation_id: Option<String>,
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
pub struct RuntimeStatus {
    pub kind: String,
    pub endpoint_id: String,
    pub reachable: bool,
    pub installed_models: Vec<String>,
    pub capabilities: RuntimeCapabilities,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeCapabilities {
    pub install: bool,
    pub chat: bool,
    pub chat_completions: bool,
    pub streaming: bool,
    pub structured_output: Vec<String>,
    pub embeddings: bool,
    pub model_details: bool,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}
