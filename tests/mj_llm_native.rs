//! Opt-in real SDK/model integration. Never fakes a result when inputs are absent.
#![cfg(all(feature = "native-macos", target_os = "macos"))]
use futures_util::StreamExt;
use mj_local_llm_hub::{
    mj_llm::{CHAT_MODEL, EMBEDDING_MODEL, MjLlmRuntime},
    runtime::{ModelRuntime, RuntimeCompletion, RuntimeEvent},
};
use serde_json::json;
#[tokio::test]
#[ignore = "requires pinned SDK and MJ_LLM_GENERATION_MODEL / MJ_LLM_EMBEDDING_MODEL"]
async fn real_native_chat_history_embeddings_and_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = MjLlmRuntime::new(
        std::env::var_os("MJ_LLM_GENERATION_MODEL")
            .expect("generation artifact")
            .into(),
        std::env::var_os("MJ_LLM_EMBEDDING_MODEL")
            .expect("embedding artifact")
            .into(),
        dir.path().join("cache"),
    )
    .await
    .unwrap();
    let status = runtime.status().await;
    assert_eq!(status.kind, "mj-llm");
    assert!(status.reachable);
    assert_eq!(status.installed_models.len(), 2);
    let request = json!({"model":CHAT_MODEL,"messages":[{"role":"system","content":"Answer briefly."},{"role":"user","content":"Remember the number 7."},{"role":"assistant","content":"I will remember 7."},{"role":"user","content":"What number did I ask you to remember?"}],"stream":false,"max_tokens":32});
    let RuntimeCompletion::Json(body) = runtime.chat_completion(&request).await.unwrap() else {
        panic!("expected JSON")
    };
    let text = body["choices"][0]["message"]["content"].as_str().unwrap();
    assert!(!text.trim().is_empty());
    assert!(text.contains('7'), "history was not recalled: {text}");
    assert_eq!(body["mj"]["delivery"], "buffered");
    let result = runtime
        .embed(&json!({"model":EMBEDDING_MODEL,"input":["고양이","강아지"]}))
        .await
        .unwrap();
    assert_eq!(result["data"].as_array().unwrap().len(), 2);
    assert_eq!(
        result["data"][0]["embedding"].as_array().unwrap().len(),
        768
    );
    assert!(
        result["mj"]["embedding_space_id"]
            .as_str()
            .unwrap()
            .starts_with("sha256:")
    );
    assert!(runtime.chat_completion(&json!({"model":CHAT_MODEL,"messages":[{"role":"user","content":"hi"}],"stream":true})).await.is_err());
    let mut events = runtime
        .chat(
            CHAT_MODEL,
            &[mj_local_llm_hub::domain::ChatMessage {
                role: "user".into(),
                content: "What is 2+2? Answer briefly.".into(),
            }],
        )
        .await
        .unwrap();
    let mut tokens = 0;
    let mut done = 0;
    while let Some(event) = events.next().await {
        match event.unwrap() {
            RuntimeEvent::Token(text) => {
                assert!(!text.is_empty());
                tokens += 1;
            }
            RuntimeEvent::Done => done += 1,
            _ => {}
        }
    }
    assert_eq!((tokens, done), (1, 1));
}
