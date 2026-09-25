# MJ Local LLM Hub MVP

장치에 맞는 로컬 모델을 추천하고 설치한 뒤 같은 화면에서 스트리밍 대화를 나누는 Rust 기반 MVP다. API는 특정 엔진과 분리된 `ModelRuntime` 어댑터를 사용한다. 현재 활성 어댑터는 로컬 [Ollama](https://ollama.com/)이며 서버는 loopback 주소만 허용한다.

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
서비스 간 통합 테스트에서는 32자 이상의 `MJ_HUB_TOKEN`을 지정해 고정 Bearer token을 사용할 수 있다. 지정하지 않으면 기존처럼 실행마다 임의 token이 생성된다.

## 구현된 MVP 범위

- OS, architecture, CPU, RAM 및 런타임 연결 진단
- Qwen3, DeepSeek-R1, Gemma 4, Mistral Small 3, Phi-4 Mini 및 OpenAI gpt-oss catalog
- 메모리 안전 여유를 반영한 deterministic 추천
- Ollama의 재개 가능한 pull stream을 이용한 모델 설치 진행률
- 멀티턴 스트리밍 대화와 JSON atomic persistence
- 최근 대화 복원과 삭제 API
- 설치된 모델을 반환하는 OpenAI 형식 `/v1/models` subset
- OpenAI-compatible `/v1/chat/completions` proxy (stream/non-stream)
- Bearer/local token 인증, 표준 gateway 오류와 비스트리밍 provenance metadata
- Ollama 모델 capability 조회 `/api/models/{model}/details`
- object-safe `ModelRuntime` trait와 설치·토큰·지표·OpenAI chunk 공통 이벤트
- 런타임 독립 오류 코드 및 동적 provider/endpoint/model provenance
- mj-narmer용 `json_object` 및 기본 `json_schema` 구조화 출력 검증
- mj-narmer 공유 API 계약 `contracts/openapi.yaml`
- responsive PWA UI, loopback-only bind, per-launch local token

## 런타임 어댑터

`AppState`는 `Arc<dyn ModelRuntime>`만 참조한다. Ollama의 `/api/pull`, `/api/chat` NDJSON과 `/v1/chat/completions` SSE 해석은 `OllamaRuntime` 안에서 수행되며 API 계층에는 정규화된 이벤트만 전달된다.

| Provider | 대상 | 상태 | 설치 | Chat | Structured output | Embeddings |
|---|---|---|---:|---:|---:|---:|
| Ollama | PC/server | MVP active | 지원 | 지원 | `json_object`, 기본 `json_schema` | 미구현 |
| mistral.rs | PC/server | planned | 예정 | 예정 | capability 검증 예정 | 예정 |
| LiteRT-LM | Android/iOS | planned | 예정 | 예정 | 모델별 판정 예정 | 예정 |
| llama.cpp | GGUF fallback | optional | 예정 | 예정 | 모델별 판정 예정 | 예정 |

### OpenAI gpt-oss

OpenAI의 Apache 2.0 오픈 웨이트 추론 모델 두 종류를 Ollama 어댑터에서 지원한다.

| 모델 | Ollama tag | 공식 권장 메모리 | 프로젝트 동작 |
|---|---|---:|---|
| gpt-oss-20b | `gpt-oss:20b` | 16GB 이상 VRAM 또는 통합 메모리 | 모델 센터에서 장치 적합성 확인 후 설치·대화 |
| gpt-oss-120b | `gpt-oss:120b` | 60GB 이상 VRAM 또는 통합 메모리 | 고메모리 워크스테이션·서버에서 설치·대화 |

두 모델은 텍스트 전용이며 128K context, reasoning effort, function calling과 structured output을 지원한다. Ollama가 Harmony prompt format을 적용하므로 별도 prompt template 없이 기존 `OllamaRuntime`의 설치·채팅·OpenAI-compatible API 경로를 재사용한다. 장치 추천기는 운영체제와 context 여유를 남기기 위해 공식 최소 실행 메모리보다 보수적으로 판정한다. gpt-oss는 OpenAI API나 ChatGPT에서 제공되는 모델이 아니라 사용자가 직접 내려받아 로컬 런타임에서 실행하는 모델이다.

- [OpenAI gpt-oss 소개](https://openai.com/index/introducing-gpt-oss/)
- [OpenAI 공식 Ollama 실행 가이드](https://developers.openai.com/cookbook/articles/gpt-oss/run-locally-ollama)
- [OpenAI gpt-oss 모델 카드](https://openai.com/index/gpt-oss-model-card/)

새 어댑터는 `status`, `capabilities`, `install`, `chat`, `chat_completion`, `model_details`를 구현하고 모든 엔진 고유 오류를 안정적인 gateway 오류 코드로 변환해야 한다. 상세 설계는 [`blueprint-local-llm-hub.md`](blueprint-local-llm-hub.md), 외부 계약은 [`contracts/openapi.yaml`](contracts/openapi.yaml)에 있다.

## mj-narmer 연동

Narmer의 OpenAI-compatible client에는 base URL을 `http://127.0.0.1:3210/v1`로 설정하고 `MJ_HUB_TOKEN`을 Bearer token으로 전달한다.

```bash
curl http://127.0.0.1:3210/v1/chat/completions \
  -H 'Authorization: Bearer <MJ_HUB_TOKEN>' \
  -H 'Content-Type: application/json' \
  -d '{
    "model": "gemma4:e2b-it-qat",
    "messages": [{"role": "user", "content": "Return {\"places\":[]}"}],
    "stream": false,
    "response_format": {
      "type": "json_schema",
      "json_schema": {
        "name": "place_extraction",
        "strict": true,
        "schema": {
          "type": "object",
          "required": ["places"],
          "properties": {"places": {"type": "array"}}
        }
      }
    },
    "metadata": {"purpose": "place_extraction", "policy_profile": "local_only", "timeout_ms": 120000}
  }'
```

비스트리밍 성공 응답의 `mj` 필드에는 provider, credential/path를 제거한 endpoint ID, 요청/실행 모델, 가능한 digest, latency와 structured-output mode가 포함된다. `metadata`는 gateway 정책에만 사용하고 upstream 모델에는 전달하지 않는다. 현재 JSON Schema 검증은 Narmer MVP에 필요한 `type`, `required`, `properties`, `items` subset이며 전체 표준 validator는 후속 작업이다.

표준 오류 코드는 `runtime_unavailable`, `model_not_installed`, `model_not_capable`, `invalid_request`, `structured_output_invalid`, `context_length_exceeded`, `request_timeout`, `request_cancelled`, `rate_limited`, `upstream_error`다.

## 아직 구현하지 않은 출시 게이트

- Android/iOS 네이티브 온디바이스 inference adapter
- 모델 artifact 자체 checksum/signature 검증과 독립 model store
- Responses 및 Anthropic adapter
- `/v1/embeddings`와 전체 JSON Schema validator
- 다중 runtime registry와 자동 장치별 adapter 선택
- RAG, MCP, Trusted Node
- load/unload/refcount/LRU memory admission
- macOS Keychain 기반 장기 client credential 발급

따라서 이 저장소는 Phase 1의 수직 MVP이며 전체 제품 출시판은 아니다.

## 검증

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```
