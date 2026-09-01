# MJ Local LLM Hub MVP

장치에 맞는 로컬 모델을 추천하고 설치한 뒤 같은 화면에서 스트리밍 대화를 나누는 Rust 기반 MVP다. 현재 실행 어댑터는 로컬 [Ollama](https://ollama.com/)이며 서버는 loopback 주소만 허용한다.

## 실행

요구사항:

- Rust 1.85 이상
- 로컬에서 실행 중인 Ollama
- 모델 다운로드를 위한 저장공간과 네트워크

```bash
ollama serve
cargo run
```

브라우저에서 `http://127.0.0.1:3210`을 연다. 다른 포트는 `MJ_HUB_PORT`, 다른 로컬 Ollama 주소는 `OLLAMA_HOST`로 지정할 수 있다.

## 구현된 MVP 범위

- OS, architecture, CPU, RAM 및 런타임 연결 진단
- Qwen3, DeepSeek-R1, Gemma 4, Mistral Small 3, Phi-4 Mini catalog
- 메모리 안전 여유를 반영한 deterministic 추천
- Ollama의 재개 가능한 pull stream을 이용한 모델 설치 진행률
- 멀티턴 스트리밍 대화와 JSON atomic persistence
- 최근 대화 복원과 삭제 API
- 설치된 모델을 반환하는 OpenAI 형식 `/v1/models` subset
- responsive PWA UI, loopback-only bind, per-launch local token

## 아직 구현하지 않은 출시 게이트

- Android/iOS 네이티브 온디바이스 inference adapter
- 모델 artifact 자체 checksum/signature 검증과 독립 model store
- OpenAI `/v1/chat/completions`, Responses, Anthropic adapter
- RAG, MCP, Trusted Node
- load/unload/refcount/LRU memory admission

따라서 이 저장소는 Phase 1의 수직 MVP이며 전체 제품 출시판은 아니다.

## 검증

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```
