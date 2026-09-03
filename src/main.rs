use mj_local_llm_hub::{
    api::{AppState, generate_token, router},
    runtime::OllamaRuntime,
    store::Store,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mj_local_llm_hub=info,tower_http=info".into()),
        )
        .init();

    let port = std::env::var("MJ_HUB_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3210);
    let bind_ip = std::env::var("MJ_HUB_BIND")
        .ok()
        .and_then(|value| value.parse::<IpAddr>().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    if !bind_ip.is_loopback() {
        return Err("MVP는 loopback 바인딩만 허용합니다. Trusted Node 보안 계층 구현 후 LAN을 활성화하세요.".into());
    }
    let runtime_url =
        std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://127.0.0.1:11434".into());
    let data_dir = std::env::var_os("MJ_HUB_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".mj-local-llm-hub"));
    let token = Arc::new(match std::env::var("MJ_HUB_TOKEN") {
        Ok(value) if value.len() >= 32 => value,
        Ok(_) => return Err("MJ_HUB_TOKEN must contain at least 32 characters".into()),
        Err(_) => generate_token(),
    });
    let state = AppState {
        runtime: Arc::new(OllamaRuntime::new(runtime_url)),
        store: Arc::new(Store::open(data_dir.join("state.json")).await?),
        token: token.clone(),
    };
    let address = SocketAddr::new(bind_ip, port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    info!(%address, "Local LLM Hub MVP started");
    info!("Open http://{address} — API token is injected only into the local page");
    axum::serve(listener, router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("Ctrl-C handler") };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}
