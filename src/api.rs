use crate::{
    catalog, device,
    domain::{ApiError, ChatRequest, Conversation},
    runtime::OllamaRuntime,
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
    pub runtime: OllamaRuntime,
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
    if supplied != Some(state.token.as_str()) {
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
    Json(
        json!({ "object": "list", "data": runtime.installed_models.into_iter().map(|id| json!({ "id": id, "object": "model", "owned_by": "local" })).collect::<Vec<_>>() }),
    )
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
    let response = state.runtime.pull(&model).await.map_err(runtime_error)?;
    Ok(Sse::new(ndjson_to_sse(response, "progress")))
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
    let response = state
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
        let mut upstream = response.bytes_stream();
        let mut buffer = String::new();
        let mut assistant = String::new();
        yield Ok(Event::default().event("meta").data(json!({"conversation_id": conversation_id}).to_string()));
        while let Some(chunk) = upstream.next().await {
            match chunk {
                Ok(bytes) => {
                    buffer.push_str(&String::from_utf8_lossy(&bytes));
                    while let Some(position) = buffer.find('\n') {
                        let line = buffer[..position].trim().to_string();
                        buffer.drain(..=position);
                        if line.is_empty() { continue; }
                        match serde_json::from_str::<Value>(&line) {
                            Ok(value) => {
                                if let Some(content) = value.pointer("/message/content").and_then(Value::as_str) {
                                    assistant.push_str(content);
                                    yield Ok(Event::default().event("token").data(json!({"content": content}).to_string()));
                                }
                                if value.get("done").and_then(Value::as_bool) == Some(true) {
                                    yield Ok(Event::default().event("metrics").data(value.to_string()));
                                }
                            }
                            Err(error) => yield Ok(Event::default().event("error").data(json!({"error": error.to_string()}).to_string())),
                        }
                    }
                }
                Err(error) => yield Ok(Event::default().event("error").data(json!({"error": error.to_string()}).to_string())),
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

fn ndjson_to_sse(
    response: reqwest::Response,
    event_name: &'static str,
) -> impl futures_util::Stream<Item = Result<Event, Infallible>> {
    async_stream::stream! {
        let mut upstream = response.bytes_stream();
        let mut buffer = String::new();
        while let Some(chunk) = upstream.next().await {
            match chunk {
                Ok(bytes) => {
                    buffer.push_str(&String::from_utf8_lossy(&bytes));
                    while let Some(position) = buffer.find('\n') {
                        let line = buffer[..position].trim().to_string();
                        buffer.drain(..=position);
                        if !line.is_empty() { yield Ok(Event::default().event(event_name).data(line)); }
                    }
                }
                Err(error) => yield Ok(Event::default().event("error").data(json!({"error": error.to_string()}).to_string())),
            }
        }
        if !buffer.trim().is_empty() { yield Ok(Event::default().event(event_name).data(buffer)); }
        yield Ok(Event::default().event("done").data("{}"));
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

fn runtime_error(error: reqwest::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::BAD_GATEWAY,
        Json(ApiError {
            error: format!("로컬 런타임 연결 실패: {error}"),
        }),
    )
}
