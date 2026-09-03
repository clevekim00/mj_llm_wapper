use crate::{
    catalog, device,
    domain::{ApiError, ChatRequest, Conversation},
    runtime::{
        ModelRuntime, RuntimeCompletion, RuntimeError, RuntimeErrorCode, RuntimeEvent,
        RuntimeEventStream,
    },
    store::{Store, now_epoch_seconds},
};
use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, Request, StatusCode, header},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response, Sse, sse::Event},
    routing::{delete, get, post},
};
use futures_util::StreamExt;
use rand::RngCore;
use serde_json::{Value, json};
use std::{collections::HashSet, convert::Infallible, sync::Arc};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub runtime: Arc<dyn ModelRuntime>,
    pub store: Arc<Store>,
    pub token: Arc<String>,
}

pub fn generate_token() -> String {
    let mut bytes = [0_u8; 24];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn router(state: AppState) -> Router {
    let protected = Router::new()
        .route("/api/status", get(status))
        .route("/api/models", get(models))
        .route("/api/models/{model}/install", post(install_model))
        .route("/api/models/{model}/details", get(model_details))
        .route("/v1/chat/completions", post(openai_chat_completions))
        .route("/api/chat", post(chat))
        .route("/api/conversations", get(conversations))
        .route("/api/conversations/{id}", delete(delete_conversation))
        .route("/v1/models", get(openai_models))
        .layer(middleware::from_fn_with_state(state.clone(), require_token));

    Router::new()
        .route("/", get(index))
        .route("/app.js", get(app_js))
        .route("/styles.css", get(styles))
        .route("/manifest.webmanifest", get(manifest))
        .merge(protected)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn require_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Response {
    let supplied = headers
        .get("x-local-token")
        .and_then(|value| value.to_str().ok());
    let bearer = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));
    if supplied != Some(state.token.as_str()) && bearer != Some(state.token.as_str()) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                error: "유효한 로컬 세션 토큰이 필요합니다.".into(),
            }),
        )
            .into_response();
    }
    next.run(request).await
}

async fn index(State(state): State<AppState>) -> Html<String> {
    Html(include_str!("../web/index.html").replace("__TOKEN_VALUE__", state.token.as_str()))
}

async fn app_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        include_str!("../web/app.js"),
    )
}

async fn styles() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../web/styles.css"),
    )
}

async fn manifest() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/manifest+json")],
        include_str!("../web/manifest.webmanifest"),
    )
}

async fn status(State(state): State<AppState>) -> Json<Value> {
    let runtime = state.runtime.status().await;
    let profile = device::detect(runtime.reachable);
    Json(json!({ "device": profile, "runtime": runtime }))
}

async fn models(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<ApiError>)> {
    let runtime = state.runtime.status().await;
    let installed: HashSet<_> = runtime.installed_models.iter().cloned().collect();
    let profile = device::detect(runtime.reachable);
    let catalog = catalog::load_catalog().map_err(internal_error)?;
    Ok(Json(
        json!({ "recommended": catalog::recommend(&catalog, &profile, &installed) }),
    ))
}

async fn openai_models(State(state): State<AppState>) -> Json<Value> {
    let runtime = state.runtime.status().await;
    let provider = runtime.kind.clone();
    let capabilities = runtime.capabilities.clone();
    Json(json!({
        "object": "list",
        "data": runtime.installed_models.into_iter().map(|id| json!({
            "id": id,
            "object": "model",
            "owned_by": "local",
            "mj": {"provider": provider, "capabilities": capabilities}
        })).collect::<Vec<_>>()
    }))
}

async fn install_model(
    State(state): State<AppState>,
    Path(model): Path<String>,
) -> Result<
    Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>>,
    (StatusCode, Json<ApiError>),
> {
    let allowed = catalog::load_catalog()
        .map_err(internal_error)?
        .into_iter()
        .any(|artifact| artifact.runtime_model == model);
    if !allowed {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "카탈로그에 없는 모델입니다.".into(),
            }),
        ));
    }
    let stream = state.runtime.install(&model).await.map_err(runtime_error)?;
    Ok(Sse::new(install_events_to_sse(stream)))
}

async fn model_details(
    State(state): State<AppState>,
    Path(model): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    state
        .runtime
        .model_details(&model)
        .await
        .map(Json)
        .map_err(runtime_openai_error)
}

async fn openai_chat_completions(
    State(state): State<AppState>,
    Json(mut request): Json<Value>,
) -> Result<Response, (StatusCode, Json<Value>)> {
    validate_openai_request(&request)?;
    let streaming = request
        .get("stream")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let structured_mode = request
        .get("response_format")
        .and_then(|value| value.get("type"))
        .and_then(Value::as_str)
        .map(str::to_owned);
    let requested_model = request
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let timeout_ms = request
        .pointer("/metadata/timeout_ms")
        .and_then(Value::as_u64)
        .unwrap_or(120_000)
        .clamp(100, 600_000);
    request.as_object_mut().map(|body| body.remove("metadata"));
    let started = std::time::Instant::now();
    let completion = tokio::time::timeout(
        std::time::Duration::from_millis(timeout_ms),
        state.runtime.chat_completion(&request),
    )
    .await
    .map_err(|_| {
        runtime_openai_error(RuntimeError {
            code: RuntimeErrorCode::RequestTimeout,
            message: "model runtime request timed out".into(),
        })
    })?
    .map_err(runtime_openai_error)?;
    match (streaming, completion) {
        (true, RuntimeCompletion::Stream(stream)) => Ok(openai_stream_response(stream)),
        (false, RuntimeCompletion::Json(mut body)) => {
            validate_structured_output(&request, &body)?;
            let details = state.runtime.model_details(&requested_model).await.ok();
            enrich_completion(
                &mut body,
                state.runtime.as_ref(),
                &requested_model,
                details.as_ref(),
                started.elapsed().as_millis() as u64,
                structured_mode.as_deref(),
            );
            Ok(Json(body).into_response())
        }
        _ => Err(openai_error(
            StatusCode::BAD_GATEWAY,
            "upstream_error",
            "model runtime returned a response with the wrong streaming mode",
        )),
    }
}

fn validate_openai_request(request: &Value) -> Result<(), (StatusCode, Json<Value>)> {
    let model = request.get("model").and_then(Value::as_str).unwrap_or("");
    let messages = request.get("messages").and_then(Value::as_array);
    if model.is_empty() || messages.is_none_or(Vec::is_empty) {
        return Err(openai_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "model and messages are required",
        ));
    }
    if let Some(profile) = request
        .pointer("/metadata/policy_profile")
        .and_then(Value::as_str)
        && profile != "local_only"
        && profile != "external_allowed"
    {
        return Err(openai_error(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "metadata.policy_profile must be local_only or external_allowed",
        ));
    }
    if let Some(kind) = request
        .pointer("/response_format/type")
        .and_then(Value::as_str)
        && kind != "json_object"
        && kind != "json_schema"
    {
        return Err(openai_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "model_not_capable",
            "response_format type is not supported",
        ));
    }
    Ok(())
}

fn enrich_completion(
    body: &mut Value,
    runtime: &dyn ModelRuntime,
    requested_model: &str,
    details: Option<&Value>,
    latency_ms: u64,
    structured_mode: Option<&str>,
) {
    let resolved_model = body.get("model").cloned().unwrap_or(Value::Null);
    let model_digest = details
        .and_then(|value| value.get("digest"))
        .cloned()
        .unwrap_or(Value::Null);
    let inference_ms = body
        .get("eval_duration")
        .and_then(Value::as_u64)
        .map(|nanoseconds| nanoseconds / 1_000_000);
    if let Some(object) = body.as_object_mut() {
        object.insert(
            "mj".into(),
            json!({
                "provider": runtime.id(),
                "endpoint_id": runtime.endpoint_id(),
                "requested_model": requested_model,
                "resolved_model": resolved_model,
                "model_digest": model_digest,
                "structured_output_mode": structured_mode.map(|_| "native").unwrap_or("unspecified"),
                "latency_ms": latency_ms,
                "queue_latency_ms": Value::Null,
                "inference_latency_ms": inference_ms
            }),
        );
    }
}

fn openai_error(status: StatusCode, code: &str, message: &str) -> (StatusCode, Json<Value>) {
    (
        status,
        Json(json!({"error": {"message": message, "type": "mj_gateway_error", "code": code}})),
    )
}

fn runtime_openai_error(error: RuntimeError) -> (StatusCode, Json<Value>) {
    let status = match error.code {
        RuntimeErrorCode::InvalidRequest => StatusCode::BAD_REQUEST,
        RuntimeErrorCode::ModelNotInstalled => StatusCode::NOT_FOUND,
        RuntimeErrorCode::ModelNotCapable | RuntimeErrorCode::StructuredOutputInvalid => {
            StatusCode::UNPROCESSABLE_ENTITY
        }
        RuntimeErrorCode::ContextLengthExceeded => StatusCode::PAYLOAD_TOO_LARGE,
        RuntimeErrorCode::RequestTimeout => StatusCode::GATEWAY_TIMEOUT,
        RuntimeErrorCode::RequestCancelled => {
            StatusCode::from_u16(499).unwrap_or(StatusCode::BAD_REQUEST)
        }
        RuntimeErrorCode::RateLimited => StatusCode::TOO_MANY_REQUESTS,
        RuntimeErrorCode::RuntimeUnavailable | RuntimeErrorCode::UpstreamError => {
            StatusCode::BAD_GATEWAY
        }
    };
    openai_error(status, error.code.as_str(), &error.message)
}

fn openai_stream_response(mut stream: RuntimeEventStream) -> Response {
    let body = Body::from_stream(async_stream::stream! {
        while let Some(event) = stream.next().await {
            match event {
                Ok(RuntimeEvent::OpenAiChunk(value)) => {
                    yield Ok::<_, Infallible>(format!("data: {value}\n\n"));
                }
                Ok(RuntimeEvent::Done) => {
                    yield Ok::<_, Infallible>("data: [DONE]\n\n".to_owned());
                    break;
                }
                Ok(_) => {}
                Err(error) => {
                    let envelope = json!({"error": {
                        "message": error.message,
                        "type": "mj_gateway_error",
                        "code": error.code.as_str()
                    }});
                    yield Ok::<_, Infallible>(format!("data: {envelope}\n\n"));
                    yield Ok::<_, Infallible>("data: [DONE]\n\n".to_owned());
                    break;
                }
            }
        }
    });
    let mut response = Response::new(body);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/event-stream"),
    );
    response
}

fn validate_structured_output(
    request: &Value,
    response: &Value,
) -> Result<(), (StatusCode, Json<Value>)> {
    let Some(format_type) = request
        .pointer("/response_format/type")
        .and_then(Value::as_str)
    else {
        return Ok(());
    };
    let content = response
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .ok_or_else(structured_output_error)?;
    let document = serde_json::from_str::<Value>(content).map_err(|_| structured_output_error())?;
    if format_type == "json_schema"
        && let Some(schema) = request.pointer("/response_format/json_schema/schema")
        && !matches_schema(&document, schema)
    {
        return Err(structured_output_error());
    }
    Ok(())
}

fn structured_output_error() -> (StatusCode, Json<Value>) {
    openai_error(
        StatusCode::UNPROCESSABLE_ENTITY,
        "structured_output_invalid",
        "model output is not valid for the requested structured format",
    )
}

// MVP validator for the JSON Schema subset used by Narmer fixtures.
fn matches_schema(value: &Value, schema: &Value) -> bool {
    if let Some(expected_type) = schema.get("type").and_then(Value::as_str) {
        let type_matches = match expected_type {
            "object" => value.is_object(),
            "array" => value.is_array(),
            "string" => value.is_string(),
            "number" => value.is_number(),
            "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
            "boolean" => value.is_boolean(),
            "null" => value.is_null(),
            _ => false,
        };
        if !type_matches {
            return false;
        }
    }
    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        let Some(object) = value.as_object() else {
            return false;
        };
        if required
            .iter()
            .filter_map(Value::as_str)
            .any(|field| !object.contains_key(field))
        {
            return false;
        }
    }
    if let (Some(object), Some(properties)) = (
        value.as_object(),
        schema.get("properties").and_then(Value::as_object),
    ) {
        for (field, field_schema) in properties {
            if let Some(field_value) = object.get(field)
                && !matches_schema(field_value, field_schema)
            {
                return false;
            }
        }
    }
    if let (Some(items), Some(item_schema)) = (value.as_array(), schema.get("items"))
        && items.iter().any(|item| !matches_schema(item, item_schema))
    {
        return false;
    }
    true
}

async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<
    Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>>,
    (StatusCode, Json<ApiError>),
> {
    if request.messages.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "메시지가 필요합니다.".into(),
            }),
        ));
    }
    let mut runtime_stream = state
        .runtime
        .chat(&request.model, &request.messages)
        .await
        .map_err(runtime_error)?;
    let store = state.store.clone();
    let conversation_id = request
        .conversation_id
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let title = request
        .messages
        .iter()
        .find(|message| message.role == "user")
        .map(|message| message.content.chars().take(42).collect())
        .unwrap_or_else(|| "새 대화".into());
    let model = request.model;
    let messages = request.messages;

    let stream = async_stream::stream! {
        let mut assistant = String::new();
        yield Ok(Event::default().event("meta").data(json!({"conversation_id": conversation_id}).to_string()));
        while let Some(event) = runtime_stream.next().await {
            match event {
                Ok(RuntimeEvent::Token(content)) => {
                    assistant.push_str(&content);
                    yield Ok(Event::default().event("token").data(json!({"content": content}).to_string()));
                }
                Ok(RuntimeEvent::Metrics(value)) => {
                    yield Ok(Event::default().event("metrics").data(value.to_string()));
                }
                Ok(RuntimeEvent::Done) => break,
                Ok(_) => {}
                Err(error) => {
                    yield Ok(Event::default().event("error").data(json!({"error": error.code.as_str()}).to_string()));
                    break;
                }
            }
        }
        let mut saved_messages = messages;
        saved_messages.push(crate::domain::ChatMessage { role: "assistant".into(), content: assistant });
        let conversation = Conversation { id: conversation_id.clone(), title, model, messages: saved_messages, updated_at_epoch_seconds: now_epoch_seconds() };
        if let Err(error) = store.save_conversation(conversation).await {
            yield Ok(Event::default().event("error").data(json!({"error": format!("대화 저장 실패: {error}")}).to_string()));
        }
        yield Ok(Event::default().event("done").data(json!({"conversation_id": conversation_id}).to_string()));
    };
    Ok(Sse::new(stream))
}

async fn conversations(State(state): State<AppState>) -> Json<Vec<Conversation>> {
    Json(state.store.conversations().await)
}

async fn delete_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ApiError>)> {
    match state
        .store
        .delete_conversation(&id)
        .await
        .map_err(internal_error)?
    {
        true => Ok(StatusCode::NO_CONTENT),
        false => Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "대화를 찾지 못했습니다.".into(),
            }),
        )),
    }
}

fn install_events_to_sse(
    mut stream: RuntimeEventStream,
) -> impl futures_util::Stream<Item = Result<Event, Infallible>> {
    async_stream::stream! {
        while let Some(item) = stream.next().await {
            match item {
                Ok(RuntimeEvent::Progress(value)) => {
                    yield Ok(Event::default().event("progress").data(value.to_string()));
                }
                Ok(RuntimeEvent::Done) => {
                    yield Ok(Event::default().event("done").data("{}"));
                    break;
                }
                Ok(_) => {}
                Err(error) => {
                    yield Ok(Event::default().event("error").data(json!({"error": error.code.as_str()}).to_string()));
                    break;
                }
            }
        }
    }
}

fn internal_error(error: impl std::fmt::Display) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: error.to_string(),
        }),
    )
}

fn runtime_error(error: RuntimeError) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::BAD_GATEWAY,
        Json(ApiError {
            error: format!("로컬 런타임 연결 실패: {}", error.code.as_str()),
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_required_openai_fields() {
        assert!(
            validate_openai_request(
                &json!({"model":"gemma4", "messages":[{"role":"user","content":"hi"}]})
            )
            .is_ok()
        );
        let error = validate_openai_request(&json!({"model":"", "messages":[]})).unwrap_err();
        assert_eq!(error.0, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.1.0.pointer("/error/code").and_then(Value::as_str),
            Some("invalid_request")
        );
    }

    #[test]
    fn adds_gateway_provenance() {
        let mut body = json!({"model":"gemma4", "choices":[]});
        let runtime = crate::runtime::OllamaRuntime::new("http://127.0.0.1:11434".into());
        enrich_completion(
            &mut body,
            &runtime,
            "gemma4",
            Some(&json!({"digest":"sha256:test"})),
            42,
            Some("json_object"),
        );
        assert_eq!(
            body.pointer("/mj/provider").and_then(Value::as_str),
            Some("ollama")
        );
        assert_eq!(
            body.pointer("/mj/structured_output_mode")
                .and_then(Value::as_str),
            Some("native")
        );
        assert_eq!(
            body.pointer("/mj/latency_ms").and_then(Value::as_u64),
            Some(42)
        );
        assert_eq!(
            body.pointer("/mj/model_digest").and_then(Value::as_str),
            Some("sha256:test")
        );
    }

    #[test]
    fn validates_narmer_structured_output() {
        let request = json!({
            "response_format": {
                "type": "json_schema",
                "json_schema": {
                    "schema": {
                        "type": "object",
                        "required": ["places"],
                        "properties": {"places": {"type": "array"}}
                    }
                }
            }
        });
        let valid = json!({"choices":[{"message":{"content":"{\"places\":[]}"}}]});
        let invalid = json!({"choices":[{"message":{"content":"not-json"}}]});
        assert!(validate_structured_output(&request, &valid).is_ok());
        let error = validate_structured_output(&request, &invalid).unwrap_err();
        assert_eq!(
            error.1.0.pointer("/error/code").and_then(Value::as_str),
            Some("structured_output_invalid")
        );
    }
}
