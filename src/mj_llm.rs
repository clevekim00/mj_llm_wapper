//! Embedded mj-llm adapter. A permit remains owned by the blocking task after an
//! HTTP timeout/disconnect, so another request cannot overlap an uncancellable SDK call.
use crate::{
    domain::{ChatMessage, ModelArtifact, RuntimeCapabilities, RuntimeStatus},
    runtime::{
        ModelRuntime, RuntimeCompletion, RuntimeError, RuntimeErrorCode, RuntimeEvent,
        RuntimeEventStream,
    },
};
use async_trait::async_trait;
use futures_util::StreamExt;
use mj_llm_core::{
    embedding::{EmbeddingSpace, TextTask},
    pin::{Model, VerifiedGenerationModel, VerifiedModel, n0_pin},
};
use mj_llm_runtime_litert::{EmbeddingEngine, generation::GenerationEngine};
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::{io::AsyncWriteExt, sync::Semaphore};

pub const CHAT_MODEL: &str = "mj-llm/qwen3-0.6b";
pub const EMBEDDING_MODEL: &str = "google/embeddinggemma-2";

fn error(code: RuntimeErrorCode, message: impl Into<String>) -> RuntimeError {
    RuntimeError {
        code,
        message: message.into(),
    }
}
fn unavailable() -> RuntimeError {
    error(
        RuntimeErrorCode::RuntimeUnavailable,
        "mj-llm requires a macOS native-macos build and the pinned LiteRT SDK; see docs/mj-llm-runtime.md",
    )
}
fn native_error(e: mj_llm_runtime_litert::RuntimeError) -> RuntimeError {
    use mj_llm_runtime_litert::RuntimeError as E;
    let code = match e {
        E::Unavailable => RuntimeErrorCode::RuntimeUnavailable,
        E::InvalidInput | E::Embedding(_) => RuntimeErrorCode::InvalidRequest,
        _ => RuntimeErrorCode::UpstreamError,
    };
    error(code, format!("mj-llm: {e}"))
}

#[derive(Clone)]
pub struct MjLlmRuntime {
    generation: PathBuf,
    embedding: PathBuf,
    cache: PathBuf,
    generation_ready: Arc<AtomicBool>,
    embedding_ready: Arc<AtomicBool>,
    gate: Arc<Semaphore>,
}
impl MjLlmRuntime {
    pub async fn new(
        generation: PathBuf,
        embedding: PathBuf,
        cache: PathBuf,
    ) -> Result<Self, RuntimeError> {
        let runtime = Self {
            generation,
            embedding,
            cache,
            generation_ready: Arc::new(AtomicBool::new(false)),
            embedding_ready: Arc::new(AtomicBool::new(false)),
            gate: Arc::new(Semaphore::new(1)),
        };
        let copy = runtime.clone();
        tokio::task::spawn_blocking(move || {
            copy.generation_ready.store(
                VerifiedGenerationModel::open(&copy.generation).is_ok(),
                Ordering::Release,
            );
            copy.embedding_ready.store(
                VerifiedModel::open(&copy.embedding).is_ok(),
                Ordering::Release,
            );
        })
        .await
        .map_err(|_| {
            error(
                RuntimeErrorCode::UpstreamError,
                "model verification task failed",
            )
        })?;
        Ok(runtime)
    }
    pub async fn from_env() -> Result<Self, RuntimeError> {
        let root = std::env::var_os("MJ_HUB_DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| ".mj-local-llm-hub".into());
        let models = std::env::var_os("MJ_LLM_MODEL_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("models"));
        Self::new(
            std::env::var_os("MJ_LLM_GENERATION_MODEL")
                .map(PathBuf::from)
                .unwrap_or_else(|| models.join(n0_pin().generation_model.filename)),
            std::env::var_os("MJ_LLM_EMBEDDING_MODEL")
                .map(PathBuf::from)
                .unwrap_or_else(|| models.join(n0_pin().model.filename)),
            root.join("native-cache"),
        )
        .await
    }
    fn available(&self) -> Result<(), RuntimeError> {
        if cfg!(all(feature = "native-macos", target_os = "macos")) {
            Ok(())
        } else {
            Err(unavailable())
        }
    }
    fn permit(&self) -> Result<tokio::sync::OwnedSemaphorePermit, RuntimeError> {
        self.gate.clone().try_acquire_owned().map_err(|_| {
            error(
                RuntimeErrorCode::RateLimited,
                "mj-llm is busy; retry after the current native operation finishes",
            )
        })
    }
    async fn generate(
        &self,
        model: &str,
        messages: &[ChatMessage],
        max_tokens: u32,
    ) -> Result<String, RuntimeError> {
        validate_messages(model, messages, max_tokens)?;
        self.available()?;
        if !self.generation_ready.load(Ordering::Acquire) {
            return Err(error(
                RuntimeErrorCode::ModelNotInstalled,
                "Install mj-llm/qwen3-0.6b or configure MJ_LLM_GENERATION_MODEL with the pinned artifact",
            ));
        }
        let permit = self.permit()?;
        let path = self.generation.clone();
        let cache = self.cache.join("generation");
        let messages: Vec<Value> = messages
            .iter()
            .map(|m| json!({"role":m.role,"content":m.content}))
            .collect();
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            let model = VerifiedGenerationModel::open(path).map_err(|_| {
                error(
                    RuntimeErrorCode::ModelNotInstalled,
                    "generation model is missing or its checksum changed",
                )
            })?;
            let mut engine = GenerationEngine::load(&model, &cache).map_err(native_error)?;
            let response = engine
                .generate_messages(&messages, max_tokens)
                .map_err(native_error)?;
            response_text(&response)
        })
        .await
        .map_err(|_| {
            error(
                RuntimeErrorCode::UpstreamError,
                "native generation task failed",
            )
        })?
    }
}

fn validate_messages(
    model: &str,
    messages: &[ChatMessage],
    max_tokens: u32,
) -> Result<(), RuntimeError> {
    if model != CHAT_MODEL {
        return Err(error(
            RuntimeErrorCode::ModelNotInstalled,
            "Select mj-llm/qwen3-0.6b; Ollama tags are not native artifacts",
        ));
    }
    if messages.is_empty() || messages.len() > 32 || !(1..=32).contains(&max_tokens) {
        return Err(error(
            RuntimeErrorCode::InvalidRequest,
            "Require 1–32 messages and max_tokens in 1–32",
        ));
    }
    if messages.iter().map(|m| m.content.len()).sum::<usize>() > 1024 {
        return Err(error(
            RuntimeErrorCode::ContextLengthExceeded,
            "mj-llm preview allows 1024 UTF-8 bytes across messages and a 512-token native context; start a new conversation",
        ));
    }
    for (i, m) in messages.iter().enumerate() {
        if m.content.trim().is_empty()
            || !matches!(m.role.as_str(), "system" | "user" | "assistant")
            || (m.role == "system" && i != 0)
        {
            return Err(error(
                RuntimeErrorCode::InvalidRequest,
                "Only nonempty text system (first), user and assistant messages are supported",
            ));
        }
    }
    if messages.last().is_none_or(|m| m.role != "user") {
        return Err(error(
            RuntimeErrorCode::InvalidRequest,
            "Last message must be user",
        ));
    }
    Ok(())
}
fn response_text(response: &Value) -> Result<String, RuntimeError> {
    let parts = response["content"].as_array().ok_or_else(|| {
        error(
            RuntimeErrorCode::UpstreamError,
            "native response has no content",
        )
    })?;
    let mut text = String::new();
    for part in parts {
        if part["type"] != "text" {
            return Err(error(
                RuntimeErrorCode::UpstreamError,
                "unexpected native response type",
            ));
        }
        text.push_str(
            part["text"]
                .as_str()
                .ok_or_else(|| error(RuntimeErrorCode::UpstreamError, "invalid native text"))?,
        );
    }
    if text.trim().is_empty() {
        return Err(error(
            RuntimeErrorCode::UpstreamError,
            "empty native response",
        ));
    }
    Ok(text)
}
fn pinned(model: &str) -> Result<Model, RuntimeError> {
    match model {
        CHAT_MODEL => Ok(n0_pin().generation_model),
        EMBEDDING_MODEL => Ok(n0_pin().model),
        _ => Err(error(
            RuntimeErrorCode::InvalidRequest,
            "Unknown mj-llm model",
        )),
    }
}

#[async_trait]
impl ModelRuntime for MjLlmRuntime {
    fn id(&self) -> &'static str {
        "mj-llm"
    }
    fn endpoint_id(&self) -> String {
        "embedded://litert-cpu".into()
    }
    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            install: true,
            chat: true,
            chat_completions: true,
            streaming: false,
            structured_output: vec![],
            embeddings: true,
            model_details: true,
        }
    }
    fn catalog(&self) -> Result<Vec<ModelArtifact>, serde_json::Error> {
        Ok([
            (
                CHAT_MODEL,
                "Qwen3 0.6B · mj-llm",
                "0.6B",
                vec!["chat".into()],
                2 * 1024_u64.pow(3),
            ),
            (
                EMBEDDING_MODEL,
                "EmbeddingGemma 2 · mj-llm",
                "440M",
                vec!["embeddings".into()],
                1024_u64.pow(3),
            ),
        ]
        .into_iter()
        .map(|(id, name, size, capabilities, ram)| {
            let model = pinned(id).expect("fixed model ID");
            ModelArtifact {
                id: id.into(),
                family: "mj-llm".into(),
                display_name: name.into(),
                runtime_model: id.into(),
                parameter_label: size.into(),
                download_bytes: model.bytes,
                estimated_peak_ram_bytes: ram,
                context_tokens: 512,
                capabilities,
                platforms: vec!["macos".into()],
                lifecycle: "experimental".into(),
                source_url: format!(
                    "https://huggingface.co/{}/tree/{}",
                    model.repository, model.revision
                ),
            }
        })
        .collect())
    }
    async fn status(&self) -> RuntimeStatus {
        let mut models = vec![];
        if self.available().is_ok() {
            if self.generation_ready.load(Ordering::Acquire) {
                models.push(CHAT_MODEL.into());
            }
            if self.embedding_ready.load(Ordering::Acquire) {
                models.push(EMBEDDING_MODEL.into());
            }
        }
        RuntimeStatus {
            kind: self.id().into(),
            endpoint_id: self.endpoint_id(),
            reachable: self.available().is_ok(),
            installed_models: models,
            capabilities: self.capabilities(),
        }
    }
    async fn install(&self, model: &str) -> Result<RuntimeEventStream, RuntimeError> {
        self.available()?;
        let pin = pinned(model)?;
        let permit = self.permit()?;
        let (path, ready) = if model == CHAT_MODEL {
            (&self.generation, &self.generation_ready)
        } else {
            (&self.embedding, &self.embedding_ready)
        };
        let path = path.clone();
        let ready = ready.clone();
        // The background task owns the admission permit and temporary file even if the
        // caller disconnects. Only a fully verified artifact is published.
        let id = model.to_owned();
        let task = tokio::spawn(async move {
            let _permit = permit;
            let parent = path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(std::path::Path::new("."));
            tokio::fs::create_dir_all(parent).await.map_err(|_| {
                error(
                    RuntimeErrorCode::UpstreamError,
                    "cannot create model directory",
                )
            })?;
            let temporary = tempfile::NamedTempFile::new_in(parent).map_err(|_| {
                error(
                    RuntimeErrorCode::UpstreamError,
                    "cannot create model download",
                )
            })?;
            let mut file = tokio::fs::File::from_std(temporary.reopen().map_err(|_| {
                error(
                    RuntimeErrorCode::UpstreamError,
                    "cannot write model download",
                )
            })?);
            let url = format!(
                "https://huggingface.co/{}/resolve/{}/{}",
                pin.repository, pin.revision, pin.filename
            );
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(3600))
                .build()
                .map_err(|_| error(RuntimeErrorCode::UpstreamError, "download client failed"))?;
            let response = client
                .get(url)
                .send()
                .await
                .and_then(reqwest::Response::error_for_status)
                .map_err(|_| {
                    error(
                        RuntimeErrorCode::UpstreamError,
                        "model download failed; check network and upstream access",
                    )
                })?;
            let mut chunks = response.bytes_stream();
            let mut bytes = 0u64;
            while let Some(chunk) = chunks.next().await {
                let chunk = chunk.map_err(|_| {
                    error(
                        RuntimeErrorCode::UpstreamError,
                        "model download interrupted",
                    )
                })?;
                bytes += chunk.len() as u64;
                if bytes > pin.bytes {
                    return Err(error(
                        RuntimeErrorCode::UpstreamError,
                        "model download exceeds pinned size",
                    ));
                }
                file.write_all(&chunk).await.map_err(|_| {
                    error(
                        RuntimeErrorCode::UpstreamError,
                        "model write failed; check disk space",
                    )
                })?;
            }
            file.sync_all()
                .await
                .map_err(|_| error(RuntimeErrorCode::UpstreamError, "model sync failed"))?;
            drop(file);
            tokio::task::spawn_blocking(move || {
                // Move the permit into verification: cancellation cannot release admission early.
                let _permit = _permit;
                if id == CHAT_MODEL {
                    VerifiedGenerationModel::open(temporary.path()).map(|_| ())
                } else {
                    VerifiedModel::open(temporary.path()).map(|_| ())
                }
                .map_err(|_| {
                    error(
                        RuntimeErrorCode::UpstreamError,
                        "downloaded artifact failed size/SHA-256 verification",
                    )
                })?;
                temporary.persist(&path).map_err(|_| {
                    error(
                        RuntimeErrorCode::UpstreamError,
                        "cannot publish verified model",
                    )
                })?;
                ready.store(true, Ordering::Release);
                Ok::<_, RuntimeError>(())
            })
            .await
            .map_err(|_| {
                error(
                    RuntimeErrorCode::UpstreamError,
                    "model verification task failed",
                )
            })?
        });
        Ok(Box::pin(async_stream::stream! {
            yield Ok(RuntimeEvent::Progress(json!({"status":"downloading and verifying"})));
            match task.await {Ok(Ok(()))=>{yield Ok(RuntimeEvent::Progress(json!({"status":"verified"})));yield Ok(RuntimeEvent::Done);},Ok(Err(e))=>yield Err(e),Err(_)=>yield Err(error(RuntimeErrorCode::UpstreamError,"install task failed"))}
        }))
    }
    async fn chat(
        &self,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<RuntimeEventStream, RuntimeError> {
        validate_messages(model, messages, 32)?;
        self.available()?;
        let runtime = self.clone();
        let model = model.to_owned();
        let messages = messages.to_vec();
        Ok(Box::pin(async_stream::stream! {
            match runtime.generate(&model,&messages,32).await {Ok(text)=>{yield Ok(RuntimeEvent::Token(text));yield Ok(RuntimeEvent::Metrics(json!({"provider":"mj-llm","delivery":"buffered","max_output_tokens":32})));yield Ok(RuntimeEvent::Done);},Err(e)=>yield Err(e)}
        }))
    }
    async fn chat_completion(&self, request: &Value) -> Result<RuntimeCompletion, RuntimeError> {
        let (model, messages, max_tokens) = parse_completion(request)?;
        let text = self.generate(&model, &messages, max_tokens).await?;
        Ok(RuntimeCompletion::Json(
            json!({"id":format!("chatcmpl-{}",uuid::Uuid::new_v4()),"object":"chat.completion","created":crate::store::now_epoch_seconds(),"model":CHAT_MODEL,"choices":[{"index":0,"message":{"role":"assistant","content":text},"finish_reason":null}],"mj":{"delivery":"buffered","max_output_tokens":max_tokens,"finish_reason_available":false,"usage_available":false}}),
        ))
    }
    async fn model_details(&self, model: &str) -> Result<Value, RuntimeError> {
        let pin = pinned(model)?;
        Ok(
            json!({"model":model,"provider":"mj-llm","backend":"cpu","context_tokens":512,"max_output_tokens":if model==CHAT_MODEL{Some(32)}else{None},"artifact_revision":pin.revision,"artifact_sha256":pin.sha256,"streaming":false,"structured_output":false,"status":"experimental"}),
        )
    }
    async fn embed(&self, request: &Value) -> Result<Value, RuntimeError> {
        let (inputs, task) = parse_embeddings(request)?;
        self.available()?;
        if !self.embedding_ready.load(Ordering::Acquire) {
            return Err(error(
                RuntimeErrorCode::ModelNotInstalled,
                "Install google/embeddinggemma-2 or configure MJ_LLM_EMBEDDING_MODEL",
            ));
        }
        let permit = self.permit()?;
        let path = self.embedding.clone();
        let cache = self.cache.join("embedding");
        tokio::task::spawn_blocking(move || {let _permit=permit;let model=VerifiedModel::open(path).map_err(|_|error(RuntimeErrorCode::ModelNotInstalled,"embedding artifact missing or changed"))?;let mut engine=EmbeddingEngine::load(&model,&cache).map_err(native_error)?;let mut data=vec![];for (index,input) in inputs.iter().enumerate(){let vector=engine.embed_text(task,input).map_err(native_error)?;data.push(json!({"object":"embedding","index":index,"embedding":vector.values()}));}Ok(json!({"object":"list","model":EMBEDDING_MODEL,"data":data,"mj":{"provider":"mj-llm","embedding_space_id":EmbeddingSpace::n0().id(),"profile":n0_pin().profile.id,"task":match task {TextTask::Query=>"search_query",TextTask::Document=>"document"},"dimensions":768,"normalized":true,"modalities":["text"],"usage_available":false}}))}).await.map_err(|_|error(RuntimeErrorCode::UpstreamError,"native embedding task failed"))?
    }
}

fn parse_completion(request: &Value) -> Result<(String, Vec<ChatMessage>, u32), RuntimeError> {
    let body = request.as_object().ok_or_else(|| {
        error(
            RuntimeErrorCode::InvalidRequest,
            "request must be an object",
        )
    })?;
    for key in body.keys() {
        if !["model", "messages", "stream", "max_tokens"].contains(&key.as_str()) {
            return Err(error(
                RuntimeErrorCode::ModelNotCapable,
                format!("mj-llm preview does not support {key}"),
            ));
        }
    }
    if request.get("stream").is_some_and(|s| s != &json!(false)) {
        return Err(error(
            RuntimeErrorCode::ModelNotCapable,
            "mj-llm native token streaming is not implemented; set stream=false",
        ));
    }
    let model = request["model"].as_str().unwrap_or("").to_owned();
    let raw = request["messages"].as_array().ok_or_else(|| {
        error(
            RuntimeErrorCode::InvalidRequest,
            "messages must be an array",
        )
    })?;
    if raw.iter().any(|m| {
        m.as_object()
            .is_none_or(|o| o.keys().any(|k| k != "role" && k != "content"))
    }) {
        return Err(error(
            RuntimeErrorCode::ModelNotCapable,
            "only role and text content are supported",
        ));
    }
    let messages: Vec<ChatMessage> =
        serde_json::from_value(request["messages"].clone()).map_err(|_| {
            error(
                RuntimeErrorCode::InvalidRequest,
                "messages require string role and content",
            )
        })?;
    let max = match request.get("max_tokens") {
        None => 32,
        Some(v) => v
            .as_u64()
            .filter(|n| (1..=32).contains(n))
            .ok_or_else(|| error(RuntimeErrorCode::InvalidRequest, "max_tokens must be 1–32"))?
            as u32,
    };
    validate_messages(&model, &messages, max)?;
    Ok((model, messages, max))
}
fn parse_embeddings(request: &Value) -> Result<(Vec<String>, TextTask), RuntimeError> {
    if request["model"] != EMBEDDING_MODEL {
        return Err(error(
            RuntimeErrorCode::InvalidRequest,
            "model must be google/embeddinggemma-2",
        ));
    }
    let body = request
        .as_object()
        .ok_or_else(|| error(RuntimeErrorCode::InvalidRequest, "expected object"))?;
    if body
        .keys()
        .any(|k| !["model", "input", "dimensions", "encoding_format", "mj"].contains(&k.as_str()))
    {
        return Err(error(
            RuntimeErrorCode::ModelNotCapable,
            "native embeddings support only model/input/dimensions/encoding_format/mj.task",
        ));
    }
    if request
        .get("dimensions")
        .is_some_and(|v| v.as_u64() != Some(768))
        || request.get("encoding_format").is_some_and(|v| v != "float")
    {
        return Err(error(
            RuntimeErrorCode::ModelNotCapable,
            "native preview embeddings require 768 dimensions and float encoding",
        ));
    }
    let options = match request.get("mj") {
        None => serde_json::Map::new(),
        Some(value) => value
            .as_object()
            .cloned()
            .ok_or_else(|| error(RuntimeErrorCode::InvalidRequest, "mj must be an object"))?,
    };
    if options.keys().any(|key| key != "task") {
        return Err(error(
            RuntimeErrorCode::ModelNotCapable,
            "native preview supports mj.task only; titles are not supported",
        ));
    }
    let task = match options.get("task").and_then(Value::as_str) {
        None if !options.contains_key("task") => TextTask::Document,
        Some("search_query") => TextTask::Query,
        Some("document") => TextTask::Document,
        _ => {
            return Err(error(
                RuntimeErrorCode::ModelNotCapable,
                "native mj.task supports document or search_query",
            ));
        }
    };
    let inputs = if let Some(s) = request["input"].as_str() {
        vec![s.to_owned()]
    } else {
        serde_json::from_value::<Vec<String>>(request["input"].clone()).map_err(|_| {
            error(
                RuntimeErrorCode::InvalidRequest,
                "input must be text or a text array",
            )
        })?
    };
    if inputs.is_empty()
        || inputs.len() > 8
        || inputs.iter().any(|s| s.trim().is_empty() || s.len() > 8192)
    {
        return Err(error(
            RuntimeErrorCode::InvalidRequest,
            "require 1–8 nonempty inputs, at most 8192 UTF-8 bytes each; native token overflow is rejected",
        ));
    }
    Ok((inputs, task))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> Value {
        json!({"model":CHAT_MODEL,"messages":[{"role":"user","content":"2 + 2?"}]})
    }
    #[test]
    fn completion_rejects_silent_option_loss_and_keeps_roles() {
        let mut r = request();
        for (key, value) in [
            ("stream", json!(true)),
            ("tools", json!([])),
            ("temperature", json!(0)),
            ("response_format", json!({"type":"json_object"})),
            ("max_tokens", json!(33)),
        ] {
            r[key] = value;
            assert!(parse_completion(&r).is_err(), "{key}");
            r.as_object_mut().unwrap().remove(key);
        }
        r["messages"] = json!([{"role":"system","content":"Be brief."},{"role":"user","content":"Remember 7."},{"role":"assistant","content":"OK."},{"role":"user","content":"Which number?"}]);
        let (_, messages, _) = parse_completion(&r).unwrap();
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[2].role, "assistant");
        assert_eq!(messages[1].content, "Remember 7.");
        r["messages"][0]["name"] = json!("ignored");
        assert!(parse_completion(&r).is_err());
    }
    #[test]
    fn context_and_modality_are_not_silently_truncated() {
        let mut r = request();
        r["messages"][0]["content"] = json!("가".repeat(342));
        assert!(matches!(
            parse_completion(&r).unwrap_err().code,
            RuntimeErrorCode::ContextLengthExceeded
        ));
        r["messages"][0]["content"] =
            json!([{"type":"image_url","image_url":{"url":"https://example.com/a.jpg"}}]);
        assert!(parse_completion(&r).is_err());
        r = request();
        r["model"] = json!("qwen3:0.6b");
        assert!(parse_completion(&r).is_err());
    }
    #[test]
    fn native_embeddings_have_an_explicit_profile() {
        let mut r =
            json!({"model":EMBEDDING_MODEL,"input":["한글","test"],"mj":{"task":"document"}});
        assert_eq!(parse_embeddings(&r).unwrap().0.len(), 2);
        for (k, v) in [
            ("dimensions", json!(512)),
            ("encoding_format", json!("base64")),
            ("mj", json!({"task":"unknown"})),
            ("input", json!([])),
        ] {
            let saved = r.clone();
            r[k] = v;
            assert!(parse_embeddings(&r).is_err());
            r = saved;
        }
        assert!(response_text(&json!({"content":[]})).is_err());
        assert_eq!(
            response_text(&json!({"content":[{"type":"text","text":"한글"}]})).unwrap(),
            "한글"
        );
    }
    #[tokio::test]
    async fn missing_models_and_busy_admission_are_explicit() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = MjLlmRuntime::new(
            dir.path().join("g"),
            dir.path().join("e"),
            dir.path().join("cache"),
        )
        .await
        .unwrap();
        assert!(runtime.status().await.installed_models.is_empty());
        let permit = runtime.permit().unwrap();
        assert!(matches!(
            runtime.permit().unwrap_err().code,
            RuntimeErrorCode::RateLimited
        ));
        drop(permit);
        assert!(runtime.permit().is_ok());
        let catalog = runtime.catalog().unwrap();
        assert_eq!(catalog.len(), 2);
        assert!(catalog.iter().all(|m| !m.runtime_model.contains(':')));
        #[cfg(not(feature = "native-macos"))]
        {
            assert!(!runtime.status().await.reachable);
            assert!(matches!(
                runtime
                    .chat_completion(&request())
                    .await
                    .err()
                    .unwrap()
                    .code,
                RuntimeErrorCode::RuntimeUnavailable
            ));
        }
    }
    #[tokio::test]
    async fn cancelled_waiter_does_not_release_inflight_permit() {
        let gate = Arc::new(Semaphore::new(1));
        let permit = gate.clone().try_acquire_owned().unwrap();
        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let worker = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            let _ = started_tx.send(());
            done_rx.recv().unwrap();
        });
        started_rx.await.unwrap();
        worker.abort();
        assert!(gate.clone().try_acquire_owned().is_err());
        done_tx.send(()).unwrap();
        worker.await.unwrap();
        assert!(gate.try_acquire_owned().is_ok());
    }
}
