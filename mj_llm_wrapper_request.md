# `mj_llm_wrapper` 확장 요청서

## 목적

`place-intelligence-service`(`mj-narmer`)가 로컬 open-weight 모델과 사용자가 선택한 외부 모델을 동일한 계약으로 호출할 수 있도록 `mj_llm_wapper`를 공용 Model Gateway로 확장한다.

현재 구현은 Ollama 모델 추천·설치, 스트리밍 채팅, 대화 저장, `/v1/models` 일부를 제공한다. Narmer가 요구하는 백그라운드 정보 추출 작업에 바로 사용하려면 아래 기능이 추가되어야 한다.

> 저장소의 현재 실제 폴더명과 crate 이름은 각각 `mj_llm_wapper`, `mj-local-llm-hub`이다. 외부 제품명과 API 명칭에서는 `mj_llm_wrapper` 또는 별도의 안정된 이름을 사용하고, 폴더명 변경은 호환성 영향을 검토한 후 별도 작업으로 진행한다.

## 권장 책임 경계

### `mj_llm_wrapper`가 담당

- Ollama 등 모델 런타임 연결, 상태 확인, 모델 목록·설치
- 통일된 생성/임베딩 요청 및 응답 계약
- 구조화 출력(JSON Schema) 지원과 기본 검증
- timeout, 취소, 제한된 재시도, 오류 정규화
- 모델·provider·latency·token usage 등 실행 메타데이터 반환
- 비밀값 참조와 로그 마스킹
- 로컬 전용/외부 허용 같은 실행 정책 적용

### `mj-narmer`가 담당

- URL 수집과 YouTube/페이지 evidence 생성
- 장소 추출용 prompt와 도메인 JSON Schema
- evidence-backed claim 검증, place resolution 및 중복 처리
- source URL, 수집 시각, 추출 방식, 모델 버전의 영속화
- 사용자 소유권, private note 분리, 검색·Slack 응답

래퍼가 장소 도메인 규칙이나 Narmer 데이터베이스를 직접 알도록 만들지 않는다.

## P0: Narmer 연동에 필요한 최소 기능

### 1. OpenAI-compatible Chat Completions

- `POST /v1/chat/completions`
- `stream: false`와 `stream: true` 모두 지원
- 현재 Ollama `/api/chat`을 내부 adapter로 사용
- 최소 요청 필드: `model`, `messages`, `temperature`, `stream`, `response_format`
- 최소 응답 필드: `id`, `object`, `created`, `model`, `choices`, `usage`
- `/v1/models`는 유지하되 설치 여부와 capability를 확장 메타데이터로 제공

Narmer의 현재 `OpenAICompatibleAnalyzer`가 `{base_url}/chat/completions`를 호출하므로 설정의 `base_url`을 `http://127.0.0.1:<port>/v1`로 지정하면 호환되어야 한다.

### 2. 구조화 출력

- `response_format: {"type":"json_object"}` 지원
- 가능하면 `response_format: {"type":"json_schema", ...}` 지원
- 런타임이 native structured output을 지원하면 전달하고, 미지원이면 명시적인 degraded mode로 처리
- JSON parse/schema validation 실패를 성공 응답으로 위장하지 말고 표준 오류로 반환
- 응답에 `structured_output_mode: native | prompt_fallback | unsupported`를 기록

### 3. 작업용 비스트리밍 호출

백그라운드 worker는 UI 채팅 SSE가 아니라 완료형 응답이 필요하다.

- 비스트리밍 호출이 전체 응답을 모아 단일 JSON으로 반환
- client disconnect/cancellation을 upstream Ollama 요청에 전파
- 요청별 timeout 지원
- 모델 출력이 비어 있거나 잘린 경우 구분 가능한 오류 코드 반환

### 4. 실행 메타데이터와 provenance

각 성공 응답에 다음 값을 제공한다.

- provider/runtime 종류와 endpoint 식별자(비밀값 제외)
- 요청 모델명과 실제 실행 모델명
- 모델 digest/version(런타임에서 획득 가능한 경우)
- prompt/completion/total token 또는 runtime이 제공하는 동등 지표
- queue 및 inference latency
- structured output mode

Narmer는 이 값을 AI-derived place claim의 provenance에 저장한다.

### 5. 표준 오류 계약

OpenAI 형식의 error envelope를 사용하고 안정적인 `code`를 제공한다.

- `runtime_unavailable`
- `model_not_installed`
- `model_not_capable`
- `invalid_request`
- `structured_output_invalid`
- `context_length_exceeded`
- `request_timeout`
- `request_cancelled`
- `rate_limited`
- `upstream_error`

내부 URL, 토큰, prompt 전문이나 private content를 오류 로그에 남기지 않는다.

### 6. 보안과 로컬 서비스 인증

- loopback-only 기본값 유지
- 현재 per-launch token 인증을 OpenAI endpoint에도 일관되게 적용
- `Authorization: Bearer <local-token>`을 우선 지원하고, 과도기에는 `x-local-token`도 허용 가능
- access token, 외부 provider key, 사용자 prompt/private note는 로그에서 마스킹
- 외부 provider를 추가할 때 key 원문 대신 macOS Keychain reference를 설정에 저장

## P1: 곧 필요한 확장

### Provider adapter 인터페이스

현재 `OllamaRuntime` 구체 타입에 결합된 상태를 trait로 분리한다.

예시 책임:

- `status()` / `list_models()`
- `chat_completion()` / streaming variant
- `embedding()`
- `pull_model()`은 설치 가능한 로컬 provider에서만 capability로 노출
- capability discovery: text, vision, tool use, JSON schema, embeddings, context size

초기 구현은 검증된 Ollama adapter 하나만 활성화한다. 추후 OpenAI-compatible remote adapter 또는 구독형 모델 연동을 별도 adapter로 추가하되 allowlist와 명시적 사용자 설정을 요구한다.

### Embeddings API

- `POST /v1/embeddings`
- 모델별 vector dimension과 normalized 여부 제공
- Narmer MVP는 SQLite FTS를 계속 사용할 수 있으므로 P0를 막지는 않지만, 의미 검색 도입 전에 필요하다.

### 실행 정책

요청별 정책 필드 또는 서버 측 profile을 제공한다.

- `local_only`
- `external_allowed`
- 허용 provider/model allowlist
- timeout과 최대 출력 토큰

자동 cloud fallback은 기본 비활성화한다. 외부 전송이 발생하면 호출 전에 이미 승인된 profile이어야 하며 응답 metadata로 실제 provider를 반환한다.

## P2: 후속 고려 사항

- vision 입력(페이지 스크린샷/비디오 key frame)
- audio transcription adapter
- model load/unload, refcount, LRU admission
- 동시 실행 queue와 장치 자원 제한
- 비용 한도 및 cloud usage accounting
- Responses API 호환성

YouTube 전체 영상 다운로드나 사이트 수집은 래퍼 책임이 아니다. Narmer collector가 공식 API와 허용된 공개 metadata를 이용해 evidence를 만들고, 래퍼에는 필요한 텍스트·허용된 미디어 입력만 전달한다.

## 제안 API 예시

```json
POST /v1/chat/completions
Authorization: Bearer <local-session-token>
{
  "model": "gemma3:12b",
  "messages": [
    {"role": "system", "content": "Extract only evidence-backed places."},
    {"role": "user", "content": "{...source evidence...}"}
  ],
  "temperature": 0,
  "stream": false,
  "response_format": {
    "type": "json_schema",
    "json_schema": {
      "name": "place_extraction",
      "strict": true,
      "schema": {"type": "object"}
    }
  },
  "metadata": {
    "purpose": "place_extraction",
    "policy_profile": "local_only"
  }
}
```

응답의 provider 확장 메타데이터 예시:

```json
{
  "model": "gemma3:12b",
  "choices": [{"message": {"role": "assistant", "content": "{\"places\":[]}"}}],
  "usage": {"prompt_tokens": 100, "completion_tokens": 12, "total_tokens": 112},
  "mj": {
    "provider": "ollama",
    "resolved_model": "gemma3:12b",
    "model_digest": "sha256:...",
    "structured_output_mode": "native",
    "latency_ms": 840
  }
}
```

## 완료 조건

- 기존 모델 추천·설치·채팅 기능과 테스트가 유지된다.
- OpenAI-compatible 비스트리밍/스트리밍 contract test가 추가된다.
- `json_object`, `json_schema`, malformed structured output 테스트가 추가된다.
- runtime unavailable, missing model, timeout, cancellation의 오류 contract test가 추가된다.
- authorization 누락/오류 및 비밀값 로그 마스킹 테스트가 추가된다.
- Narmer fixture를 사용한 통합 테스트에서 `OpenAICompatibleAnalyzer`가 래퍼를 통해 장소 JSON을 받고 검증을 통과한다.
- OpenAPI 또는 JSON Schema 계약 파일을 저장소의 공유 source of truth로 추가한다.
- README에 지원 provider/capability 표와 안정성 수준을 명시한다.

## 권장 구현 순서

1. `ModelRuntime` trait와 오류/응답 domain type 도입
2. 기존 `OllamaRuntime`을 trait adapter로 이동
3. `/v1/chat/completions` 비스트리밍 구현
4. structured output와 provenance metadata 구현
5. streaming 및 cancellation 보강
6. OpenAPI/JSON Schema와 contract/integration tests 추가
7. `/v1/embeddings`와 실행 정책 추가

## 현재 기준선

- 검토 시점: 2026-09-01
- 현재 테스트: `cargo test` 성공, 2 passed / 0 failed
- 현재 OpenAI 호환 범위: `/v1/models` subset만 구현
- 현재 runtime: Ollama 단일 구체 adapter
