# Local LLM Hub Codex Automation Blueprint
> Created: 2026-08-18
> Updated: 2026-09-03
> Purpose: Codex implementation blueprint

## 0. Goals and Deliverables

### Primary Goal
PC, 서버, Android, iOS에서 사용자가 로컬 LLM을 검색·설치·실행하고, 같은 대화 UI에서 RAG와 MCP 도구를 프로젝트별로 연결할 수 있는 로컬 우선(Local-first) 플랫폼을 설계한다. 초기 필수 모델군은 영상에서 확인한 Qwen3, DeepSeek-R1 Distill 7B, Gemma 4, Mistral Small 3, Phi-4 Mini이며, 특정 revision을 코드에 고정하지 않고 모델 카탈로그로 갱신한다.

### Success Definition
- 사용자가 10분 이내에 장치 진단부터 권장 모델 설치, 첫 스트리밍 대화까지 완료한다.
- 모바일에서 지원 판정을 통과한 소형·양자화 모델을 비행기 모드에서도 직접 실행한다.
- 장치 RAM, 저장공간, CPU/GPU/NPU, OS, 모델 포맷을 바탕으로 `권장`, `실행 가능`, `비권장`, `지원 불가`를 근거와 함께 표시한다.
- PDF, Markdown, TXT, 웹페이지, 동기화 폴더를 프로젝트 지식으로 추가하고 답변에 출처를 표시한다.
- MCP 서버와 도구는 프로젝트별로 활성화하며, 민감한 도구 호출 전에 사용자 승인을 받는다.
- 대화 내용, 문서 원문, 임베딩은 기본적으로 로컬에만 저장되고 외부 통신은 명시적 동의 및 허용 목록을 따른다.
- PC·서버·모바일 클라이언트가 동일한 대화·프로젝트 도메인과 OpenAI 호환 API를 사용한다.
- 5개 필수 모델군 모두 PC 또는 서버에서 최소 하나의 검증된 로컬 실행 아티팩트를 가지며, 모바일은 장치 적합성 검사를 통과한 variant만 설치할 수 있다.
- 모델별 chat template, thinking mode, tool calling, vision, context length 차이가 capability로 표현되고 대화·RAG·MCP UI가 이를 존중한다.

### Out of Scope
- 1차 버전에서 모델 학습, 파인튜닝, 분산 학습, 클라우드 추론 서비스를 직접 제공하지 않는다.
- 임의의 Hugging Face 모델을 검증 없이 모바일 실행 포맷으로 자동 변환하지 않는다.
- 모바일에서 대형 서버급 모델의 실행을 보장하지 않는다.
- MCP 서버의 안전성을 플랫폼이 보증하지 않으며, 샌드박스 밖 무제한 실행을 허용하지 않는다.
- 1차 버전에서 음성·영상 생성이나 멀티 사용자 실시간 공동 편집은 다루지 않는다.

## 1. Working Context

### Background
로컬 LLM 생태계는 모델 저장소, 양자화 포맷, 실행 엔진, 하드웨어 가속 방식이 분산되어 있다. 사용자는 모델 이름만으로 자신의 장치에서 실행 가능한지 판단하기 어렵고, RAG 및 MCP를 붙이려면 별도의 벡터 DB, 문서 파서, 서버 설정과 보안 판단이 필요하다. 이 프로젝트는 이 복잡성을 하나의 설치·대화·프로젝트 경험으로 묶는다.

2026-08-21 공식 자료 조사 기준 Gemma 4는 E2B, E4B, 12B, 31B, 26B-A4B 변형을 제공하며, Google LiteRT-LM은 Android, iOS, 데스크톱을 포함한 엣지 실행을 지향한다. Qwen3는 공식 GGUF와 모바일 실행 경로를 제공한다. DeepSeek-R1 Distill Qwen 7B는 Qwen2.5 기반 추론 특화 모델이다. Mistral Small 3.0은 24B Apache 2.0 모델이지만 현재 공식 문서상 retired 상태이므로 호환성 지원 대상으로 유지하되 신규 기본 추천에서는 후속 모델을 함께 안내한다. Phi-4 Mini Instruct는 3.8B급, 128K context, MIT 모델로 자원 제한 환경을 주요 용도로 명시한다. `최신 모델`, `영상 속 호환 모델`, `현재 장치에 적합한 모델`을 별도 축으로 관리한다.

참고 공식 자료:
- [Gemma releases](https://ai.google.dev/gemma/docs/releases)
- [Gemma 4 overview](https://ai.google.dev/gemma/docs/core)
- [Google LiteRT-LM](https://github.com/google-ai-edge/LiteRT-LM)
- [Qwen official organization](https://github.com/QwenLM)
- [Qwen 3.6 repository](https://github.com/QwenLM/Qwen3.6)
- [Qwen3 local llama.cpp guide](https://github.com/QwenLM/Qwen3/blob/main/docs/source/run_locally/llama.cpp.md)
- [DeepSeek-R1 official repository](https://github.com/deepseek-ai/DeepSeek-R1)
- [Mistral Small 3 announcement](https://mistral.ai/news/mistral-small-3)
- [Mistral Small 3.0 lifecycle status](https://docs.mistral.ai/models/mistral-small-3-0-25-01)
- [Phi-4 Mini Instruct model card](https://huggingface.co/microsoft/Phi-4-mini-instruct)

### Objective
Codex 구현 흐름은 요구사항과 공식 모델 메타데이터를 수집하고, Rust 중심의 코어 아키텍처와 플랫폼별 런타임 어댑터, 모델 적합성 판정, 대화, RAG, MCP, 보안, 검증 계약을 재현 가능한 산출물로 만든다.

### Scope
- Included: 장치 진단, 동적 모델 카탈로그, 다운로드/검증/삭제, 로컬 추론, 스트리밍 채팅, 프로젝트/대화 저장, RAG 수집·검색·출처, MCP 등록·권한·호출, OpenAI 호환 API, 웹/데스크톱/Android/iOS 클라이언트 계약.
- Excluded: 학습·파인튜닝, 미검증 자동 변환, 퍼블릭 클라우드 호스팅, 엔터프라이즈 SSO/RBAC, 모델 라이선스 법률 자문.

### Inputs
| Item | Format | Source | Notes |
|---|---|---|---|
| 제품 요구사항 | md/json | user | 대상 플랫폼, 개인정보, UX 범위 |
| 장치 프로필 | json | system | OS, arch, RAM, 가용 저장공간, CPU/GPU/NPU, 배터리/열 상태 |
| 모델 메타데이터 | json/api | official registry | 5개 필수 family, 모델 ID, revision, 파일, 해시, 라이선스, 포맷, 메모리 요구량 |
| 런타임 capability | json | runtime adapter | 지원 포맷, 가속기, 컨텍스트, tool calling 지원 |
| 사용자 문서 | pdf/md/txt/html | user/file/url | 로컬 저장 기본 |
| MCP 설정 | json | user/registry | 실행 명령 또는 원격 endpoint, 권한 선언 |
| 대화 요청 | json | client | 프로젝트, 모델, 메시지, 첨부, 생성 옵션 |

### Outputs
| Item | Format | Destination | Notes |
|---|---|---|---|
| 모델 추천 결과 | json | local API/UI | 등급, 예상 메모리·속도, 근거, 경고 포함 |
| 설치된 모델 | model files + json | managed local storage | revision, hash, provenance 기록 |
| 스트리밍 대화 | SSE/WebSocket/JSON | client | 토큰, 상태, 인용, tool event |
| RAG 인덱스 | SQLite + vector index | app data | 원문·청크·임베딩 버전 관리 |
| MCP 권한 정책 | json | encrypted app config | 프로젝트/서버/도구별 allow/ask/deny |
| 감사 로그 | jsonl | local app data | 다운로드, 권한 승인, 도구 호출; 비밀 값 제거 |
| 구현 계약 | md/json schema | docs/contracts | 플랫폼 공통 API 및 어댑터 규격 |

### Constraints
- 로컬 우선: 대화, 문서, 임베딩은 기본 외부 전송 금지. 모델 정보 업데이트만 opt-in 네트워크 접근을 허용한다.
- Rust 우선: 새 네이티브 코어와 시스템 로직은 Rust로 작성한다. C/C++ 런타임이 필수인 경우 Rust FFI 어댑터 뒤에 격리하고 메모리 안전 경계를 문서화한다.
- 모바일 직접 실행은 필수이며 Android와 iOS 모두 지원 대상으로 삼는다. 다만 런타임별 지원 격차는 capability 판정으로 노출한다.
- 대형 모델 다운로드는 중단 재개, 해시 검증, 충분한 여유 공간 확인, Wi-Fi/충전 조건 설정이 필요하다.
- 모델 라이선스, gated access, 배포 조건을 설치 전에 표시하고 사용자 동의를 기록한다.
- 모델이 선언한 메모리 요구량만 믿지 않고 최초 실행 micro-benchmark와 OOM 안전 여유를 적용한다.
- 성능은 decode tokens/s 하나로 판단하지 않고 prefill, TTFT, decode, total completion, peak memory, task success를 분리한다.
- coding/agent 평가는 requirement coverage, plan-to-code traceability, first-run success, manual intervention time, energy를 포함한다.
- 모델 weights, KV cache, runtime overhead, OS reserve를 합산하고 dense/MoE의 total parameter와 active parameter를 구분한다.
- load admission과 지속 실행 상태를 분리하고 memory pressure·swap·thermal 임계치를 넘으면 context/variant 조정 또는 안전 unload한다.
- MCP는 로컬 프로세스(stdio)와 원격 HTTP 계열 전송을 구분하며, 모바일에서는 임의 바이너리 실행을 허용하지 않고 원격 또는 앱 내장 MCP만 지원한다.
- 웹페이지 수집은 robots, 인증, 저작권, 네트워크 동의를 준수하며 기본적으로 사용자가 지정한 URL만 처리한다.
- Assumption: 1차 UI는 웹 기반 반응형 UI와 데스크톱 패키징을 먼저 제공하고, Android/iOS 네이티브 앱은 동일 API/도메인 계약을 사용한다.
- Assumption: 서버 모드는 단일 사용자/신뢰 네트워크를 우선하고 다중 사용자 RBAC는 후속 단계다.
- 원격 추론은 pairing된 Trusted Node만 허용하고 TLS, scoped token, revocation, local/VPN 우선 경로를 사용한다.
- Portable Workspace는 PoC 범위이며 host RAM·temporary file·OS log 흔적을 완전히 제거한다고 보장하지 않는다.
- cloud fallback은 기본 비활성화하며 프로젝트별 명시 동의, 전송 preview, redaction, 비용·token 제한이 필요하다.
- `지원`은 family 이름 인식만을 뜻하지 않는다. 공식/승인된 artifact 다운로드, template 적용, 추론, 중지, 대화 복원, RAG, 지원 가능한 MCP tool calling까지 검증되어야 한다.
- Mistral Small 3.0은 retired 모델이므로 카탈로그에 `compatibility` badge와 후속 모델 안내를 표시하고 보안·포맷 호환 패치는 유지하되 기본 자동 추천 우선순위는 낮춘다.
- 영상 속 벤치마크·VRAM·라이선스 문구는 마케팅 참고값으로만 보존하며, 제품 판정은 pinned model card, 실제 artifact license, 장치별 benchmark에 따른다.

### Terms
| Term | Definition |
|---|---|
| Model Catalog | 설치 가능 아티팩트, family, revision, 라이선스, 포맷, runtime compatibility를 담은 서명된 목록 |
| Required Five | Qwen3, DeepSeek-R1 Distill Qwen 7B, Gemma 4, Mistral Small 3, Phi-4 Mini의 초기 호환성 대상 |
| Device Profile | 장치 자원과 가속기 capability를 정규화한 스냅샷 |
| Fit Score | 모델과 장치의 실행 적합성을 메모리, 성능, 기능, 안정성으로 평가한 점수 |
| Runtime Adapter | 공통 추론 계약을 LiteRT-LM, llama.cpp 계열, MLX, 서버 엔진 등에 연결하는 계층 |
| Project | 대화, 기본 모델, 시스템 지침, 지식베이스, MCP 정책의 격리 단위 |
| Local-first | 데이터와 추론을 기본적으로 장치 내부에서 처리하고 외부 통신은 선택적으로 허용하는 정책 |
| ModelRuntime | API 계층과 특정 추론 엔진 사이의 Rust 비동기 trait 계약 |
| RuntimeEvent | 설치 진행률, 토큰, 지표, OpenAI chunk, 완료를 표현하는 런타임 독립 이벤트 |
| Provenance | provider, endpoint, 실제 모델, digest, 지연시간, 구조화 출력 모드를 기록한 실행 근거 |

### Runtime Adapter Architecture

API와 UI는 특정 엔진의 URL, NDJSON, 모델 파일 형식을 직접 알지 않는다. Rust 코어의 `ModelRuntime` trait가 상태 확인, capability discovery, 설치, 대화, OpenAI Chat Completions, 모델 상세 조회를 정의한다. 어댑터는 엔진 고유 응답을 `RuntimeEvent`와 표준 `RuntimeErrorCode`로 변환하고, raw HTTP response를 API 계층에 노출하지 않는다.

| Adapter | Target | Integration mode | Initial status | Responsibility |
|---|---|---|---|---|
| `OllamaRuntime` | PC/server compatibility | local HTTP sidecar | MVP active | 기존 Ollama 설치·채팅·OpenAI API를 공통 계약으로 정규화 |
| `MistralRsRuntime` | PC/server primary | Rust sidecar first, embedded SDK later | planned | Rust-first inference, Metal/CUDA, server batching |
| `LiteRtLmRuntime` | Android/iOS on-device | Kotlin/Swift/C ABI behind Rust facade | planned | 모바일 직접 추론, GPU/NPU 및 앱 lifecycle 연동 |
| `LlamaCppRuntime` | GGUF compatibility fallback | isolated sidecar/FFI | optional | 다른 어댑터가 지원하지 않는 GGUF 아티팩트 수용 |

`AppState`는 `Arc<dyn ModelRuntime>`만 보유한다. 새 어댑터는 다음 conformance suite를 통과해야 등록된다: health/list, install capability, non-stream/stream chat, cancellation, timeout, error normalization, structured output capability, provenance redaction. 런타임 선택은 향후 `RuntimeRegistry`가 플랫폼, 모델 아티팩트, 사용자 policy profile과 capability를 기준으로 수행한다. 자동 cloud fallback은 금지한다.

모델 카탈로그는 장기적으로 단일 `runtime_model` 문자열에서 `runtime_variants[]`로 이전한다. 각 variant는 `runtime`, `artifact_id`, `format`, `revision`, `checksum`, `platforms`, `capabilities`, `stability`를 갖는다. 기존 필드는 마이그레이션 기간 동안 Ollama 호환 alias로 읽는다.

### mj-narmer Integration Contract

`mj_llm_wrapper_request.md`를 Narmer 연동 요구사항의 입력 문서로 사용한다. 책임 경계는 다음과 같다.

- Gateway: 런타임 연결, OpenAI 호환 생성, 구조화 출력 검증, timeout/cancellation, 오류 정규화, provenance, 로컬/외부 실행 정책과 비밀값 마스킹.
- mj-narmer: URL·YouTube evidence 수집, 장소 추출 prompt/schema, evidence-backed claim 검증, place resolution, 사용자 데이터 영속화.
- Gateway는 Narmer의 장소 도메인이나 데이터베이스를 알지 않으며, Narmer는 특정 로컬 런타임 프로토콜을 알지 않는다.

P0 공유 source of truth는 `contracts/openapi.yaml`이다. `POST /v1/chat/completions`는 `stream` 양쪽 모드와 `json_object`, 제한된 MVP `json_schema` 검증을 제공한다. 성공 응답의 `mj`에는 동적 provider, 비밀값이 제거된 endpoint ID, 요청/실행 모델, 가능한 digest, latency와 structured-output mode를 기록한다. 오류는 요청서에 정의된 안정적인 OpenAI error envelope code를 사용한다. `Authorization: Bearer`가 표준이며 `x-local-token`은 이전 UI 호환 기간에만 유지한다.

P1은 `/v1/embeddings`, 다중 `RuntimeRegistry`, signed policy profile, 전체 JSON Schema validator를 포함한다. P2는 vision/audio adapter, load admission, queue, 비용 회계와 Responses API다.

## 2. Workflow Definition

### End-to-End Flow
`[Requirements + Official Metadata] -> [Step 01 Capability Baseline] -> [Step 02 Architecture Contracts] -> [Step 03 Catalog and Fit Engine] -> [Step 04 Model Lifecycle] -> [Step 05 Conversation Runtime] -> [Step 06 RAG Pipeline] -> [Step 07 MCP Security] -> [Step 08 Cross-platform Validation] -> [Implementation-ready Blueprint]`

### LLM vs Code Boundary
| LLM handles | Code handles |
|---|---|
| 요구사항 누락 탐지, 모델 용도 분류, UX 문구 초안, 위험·trade-off 평가, 검색 문맥 구성 | 장치 탐지, 메모리 계산, 해시/서명 검증, 다운로드, DB I/O, 청킹, 임베딩, 벡터 검색, 권한 집행, API/스키마 검증 |
| 모델 설명과 라이선스 요약, 적합성 결과의 자연어 설명 | 적합성 등급 계산 및 hard constraint 판정 |
| RAG 결과로 답변 생성, 인용할 청크 선택 | 문서 파싱, 중복 제거, ACL 필터, top-k 검색, 인용 ID 검증 |
| MCP 도구 선택과 인자 제안 | 도구 schema 검증, 승인 확인, sandbox 실행, timeout/출력 제한, 감사 로그 |
| 작업 복잡도 분류와 planner/implementer/reviewer 후보 제안 | capability score 기반 허용 경로, 최소 context handoff, acceptance test, intervention event 기록 |
| Quick/Balanced/Deep profile과 신규 후보 승격 판단 | memory admission, swap/pressure 감시, fallback 순서, catalog gate를 결정론적으로 집행 |

#### Step 01: Establish Capability Baseline
1) Step Goal:
공식 자료와 실제 대상 장치군을 기준으로 5개 필수 모델군의 모델·런타임·플랫폼 지원 매트릭스를 만든다.

2) Input / Output:
- Input: 제품 요구사항, 공식 모델 카드, 런타임 문서, 표본 장치 프로필.
- Output: versioned capability matrix와 미확정 항목 목록.

3) LLM Decision Area:
문서 간 표현 차이를 정규화하고, `공식 지원`, `실험적`, `미검증`, `불가`를 구분한다.

4) Code Processing Area:
공식 endpoint에서 메타데이터를 수집하고 revision, checksum, 크기, 포맷 필드를 schema로 검증한다.

5) Success Criteria:
5개 필수 모델군별 최소 하나의 PC 또는 서버 실행 경로와 Android/iOS/desktop/server 지원 상태가 근거 URL 및 확인일과 함께 존재한다. 모바일 미지원도 실패가 아니라 검증 근거가 있는 명시적 상태여야 한다.

6) Validation Method:
JSON Schema 검사, 출처 URL 접근 검사, 사람이 모바일 지원 표본을 검토한다.

7) Failure Handling:
공식 정보가 없거나 충돌하면 자동 추정하지 않고 `unverified`로 표시한다. 네트워크 오류는 지수 backoff로 3회 재시도 후 기존 서명 카탈로그를 사용한다.

8) Skills / Scripts:
- Skill: `model-catalog-curator`
- Script: `.agents/skills/model-catalog-curator/scripts/normalize_catalog.rs`

9) Intermediate Artifact Rule:
`output/step01_capability_matrix.json`

#### Step 02: Define Architecture and Contracts
1) Step Goal:
Rust 코어, 런타임 어댑터, 저장소, API, 플랫폼 UI 사이의 안정적인 경계를 정의한다.

2) Input / Output:
- Input: capability matrix, 개인정보·플랫폼 제약.
- Output: 컴포넌트 책임, API schema, event/state 계약, threat boundary.

3) LLM Decision Area:
변경 가능성이 큰 런타임 세부 구현과 장기 유지할 도메인 계약의 경계를 판단한다.

4) Code Processing Area:
`ModelRuntime` object-safe 비동기 trait, 정규화된 `RuntimeEvent`/`RuntimeErrorCode`, `RuntimeRegistry`, OpenAPI/JSON Schema, DB migration 규칙을 정적 검사한다. Ollama NDJSON과 OpenAI SSE decoding은 `OllamaRuntime` 내부에서 끝내고 API에는 공통 이벤트만 전달한다.

5) Success Criteria:
클라이언트가 특정 엔진을 알지 않고도 모델 설치, 대화, RAG, MCP를 호출할 수 있고 어댑터 교체가 DB/API 변경을 강제하지 않는다. mj-narmer가 `base_url=/v1`과 Bearer token만으로 비스트리밍 장소 추출을 요청하고 표준 provenance/error를 받을 수 있다.

6) Validation Method:
`contracts/openapi.yaml` 정적 검사, mock adapter conformance test, Narmer JSON fixture, dependency direction 검사, architecture decision record의 human review.

7) Failure Handling:
플랫폼별 capability가 공통 계약을 충족하지 못하면 optional capability로 격리하고 UI에서 기능을 숨긴다. 핵심 대화 계약 불충족 시 해당 어댑터 채택을 중단한다.

8) Skills / Scripts:
- Skill: `local-llm-architecture`
- Script: `scripts/validate_contracts.rs`

9) Intermediate Artifact Rule:
`output/step02_architecture_contracts.md`

#### Step 03: Design Catalog and Device Fit Engine
1) Step Goal:
장치에 맞는 모델 variant와 runtime을 안전하게 추천하고 설치 불가 원인을 설명한다.

2) Input / Output:
- Input: Device Profile, Model Catalog, runtime capability, 사용자 목적.
- Output: ranked recommendation JSON과 예상 자원·품질·기능 trade-off.

3) LLM Decision Area:
`일반 대화`, `코딩`, `추론`, `긴 문서`, `도구 사용`, `멀티모달` 목적의 가중치와 모델 family별 설명 문구를 구성한다.

4) Code Processing Area:
hard filter(arch/format/runtime/license/storage), 예상 peak RAM(`weights + KV cache + runtime overhead + OS reserve + safety margin`), GPU residency/offload, disk reserve, thermal tier, benchmark score를 결정론적으로 계산한다.

결과에는 `ARTIFACT_FITS`, `SESSION_STARTABLE`, `SUSTAINED_SAFE`를 별도 boolean과 근거로 기록한다.

5) Success Criteria:
같은 입력은 같은 추천을 만들고, 지원 불가 모델은 설치 버튼이 비활성화되며 구체적 이유와 같은 family의 작은 variant 또는 대안 모델이 표시된다.

개발 workload에서는 planning, implementation, review 점수를 분리해 단일 모델 고정과 단계별 routing 후보를 모두 반환한다.

6) Validation Method:
golden device fixtures, 경계값/property tests, dense/MoE 및 context/KV-cache fixtures, 실제 저·중·고사양 장치 benchmark 비교.

7) Failure Handling:
장치 정보가 누락되면 보수적 CPU/RAM 기준을 사용하고 `측정 필요`로 표시한다. 최초 실행 OOM, swap 급증 또는 과열 시 context 축소 → 작은 quant/variant → unload 순으로 대응하고 자동 재실행은 한 번만 허용한다.

8) Skills / Scripts:
- Skill: `device-model-advisor`
- Script: `.agents/skills/device-model-advisor/scripts/score_fit.rs`

9) Intermediate Artifact Rule:
`output/step03_model_fit_report.json`

#### Step 04: Design Secure Model Lifecycle
1) Step Goal:
모델 탐색, 라이선스 동의, resumable download, 검증, 설치, 최적화, rollback, 삭제를 정의한다.

2) Input / Output:
- Input: 선택된 catalog artifact, 저장 위치, 네트워크/배터리 정책.
- Output: 검증된 model installation record와 runtime-ready artifact.

3) LLM Decision Area:
라이선스 핵심 조건과 실패 원인을 사용자가 이해할 수 있는 언어로 요약한다.

4) Code Processing Area:
URL allowlist, TLS, byte-range download, 임시 파일, 크기/hash/signature 검증, atomic rename, 캐시 참조계수, 삭제 전 사용 여부 검사를 수행한다.

5) Success Criteria:
부분 다운로드나 위조 파일이 실행 경로에 진입하지 않고, 설치 중 앱 종료 후 재개 가능하며, 동일 revision을 중복 저장하지 않는다.

6) Validation Method:
corrupt/truncated artifact 테스트, 디스크 부족 fault injection, checksum fixture, 설치/삭제 state-machine 테스트.

7) Failure Handling:
해시 불일치 시 파일을 격리하고 재시도 1회 후 중단한다. 공간 부족 시 필요한 추가 용량을 표시한다. gated model 인증 실패 시 토큰 입력을 요청하되 토큰을 로그에 남기지 않는다.

8) Skills / Scripts:
- Skill: `model-lifecycle-manager`
- Script: `.agents/skills/model-lifecycle-manager/scripts/verify_artifact.rs`

9) Intermediate Artifact Rule:
`output/step04_installation_manifest.json`

#### Step 05: Build Conversation Runtime Contract
1) Step Goal:
현재 대화와 같은 멀티턴 스트리밍 채팅을 프로젝트 및 모델 전환과 함께 제공한다.

2) Input / Output:
- Input: project, conversation history, selected model, system policy, attachments, generation options.
- Output: token stream, final assistant message, usage/latency metrics, citations, tool events.

3) LLM Decision Area:
답변 생성, 대화 문맥 축약, RAG 사용 판단, MCP 도구 호출 제안, 모델 전환 시 맥락 유지 전략을 담당한다.

4) Code Processing Area:
chat template 적용, tokenizer/context budget 계산, cancellation, 요청별 timeout, backpressure, persistence, retry idempotency, SSE/WebSocket 변환을 수행한다. 비스트리밍 structured output은 JSON parse 후 Narmer가 사용하는 JSON Schema subset을 검증하고 실패를 `structured_output_invalid`로 반환한다.

5) Success Criteria:
대화 생성·이름 변경·검색·삭제, 메시지 수정 후 분기, 모델/로컬·Trusted Node routing 전환, 응답 중지가 가능하고 앱 재시작 후 복원된다. OpenAI 비스트리밍 응답에는 provider, sanitized endpoint, requested/resolved model, 가능한 digest, latency, structured output mode가 포함된다.

6) Validation Method:
API contract tests, 긴 대화 context-budget tests, stream cancellation tests, golden chat-template tests, node disconnect/reconnect 및 routing failover tests.

7) Failure Handling:
컨텍스트 초과 시 최근 메시지와 고정 메시지를 보존해 요약을 제안한다. runtime crash는 세션을 복구하고 미완료 메시지를 명시한다. 자동 재생성은 중복 부작용이 없는 경우 1회로 제한한다.

8) Skills / Scripts:
- Skill: `conversation-runtime`
- Script: `scripts/validate_chat_events.rs`

9) Intermediate Artifact Rule:
`output/step05_conversation_contract.json`

#### Step 06: Build Local RAG Pipeline
1) Step Goal:
PDF, Markdown, TXT, 웹페이지, 동기화 폴더를 로컬 지식베이스로 만들고 근거 있는 답변을 제공한다.

2) Input / Output:
- Input: 문서/URL/폴더, parser policy, embedding model, query.
- Output: versioned chunks, vector index, retrieval results, source citations.

3) LLM Decision Area:
질의 재작성, 검색 결과 관련성 보조 판정, 충분한 근거 여부, 인용 기반 답변을 담당한다.

4) Code Processing Area:
파일 type 감지, parsing, content hash, incremental sync, chunking, embedding batch, vector/keyword hybrid search, metadata/프로젝트 ACL filtering을 수행한다.

5) Success Criteria:
변경된 문서만 재색인하고, 답변의 모든 citation이 존재하는 source/chunk로 역참조되며, 프로젝트 간 문서가 섞이지 않는다.

6) Validation Method:
parser fixture, retrieval benchmark, citation referential-integrity 검사, 프로젝트 격리 테스트, 한국어 질의 평가셋.

7) Failure Handling:
암호화/손상 PDF는 사용자에게 알리고 건너뛴다. embedding 모델 변경 시 별도 index version을 만들고 완료 후 atomic switch한다. 웹 수집 실패는 원본 캐시가 있을 때만 stale 표시로 사용한다.

8) Skills / Scripts:
- Skill: `local-rag-pipeline`
- Script: `.agents/skills/local-rag-pipeline/scripts/validate_index.rs`

9) Intermediate Artifact Rule:
`output/step06_rag_evaluation.json`

#### Step 07: Build MCP Registry and Permission Gateway
1) Step Goal:
MCP 서버 등록·연결·도구 발견·승인·실행을 프로젝트별 최소 권한으로 제공한다.

2) Input / Output:
- Input: MCP manifest/endpoint, tool schemas, project policy, proposed tool call.
- Output: allow/ask/deny decision, sanitized result, audit event.

3) LLM Decision Area:
사용자 의도에 맞는 도구 선택과 인자를 제안하고, 승인이 필요한 행위를 자연어로 설명한다.

4) Code Processing Area:
schema validation, executable/host allowlist, secret injection, filesystem/network scope, timeout, output limit, consent token, audit/redaction을 강제한다.

5) Success Criteria:
승인되지 않은 도구는 실행되지 않고, 권한 범위는 프로젝트별로 격리되며, 모바일은 임의 로컬 프로세스를 시작하지 않는다.

6) Validation Method:
malicious MCP fixtures, command/path traversal tests, SSRF tests, approval replay tests, secret redaction 검사.

7) Failure Handling:
schema 불일치나 권한 위반은 즉시 deny하고 기록한다. timeout은 프로세스/요청을 종료한다. 동일 서버가 3회 연속 실패하면 circuit-open 상태로 전환하고 사용자 재활성화를 요구한다.

8) Skills / Scripts:
- Skill: `mcp-permission-auditor`
- Script: `.agents/skills/mcp-permission-auditor/scripts/validate_policy.rs`

9) Intermediate Artifact Rule:
`output/step07_mcp_threat_report.json`

#### Step 08: Validate Cross-platform Release Readiness
1) Step Goal:
대표 장치와 실패 조건에서 설치·대화·RAG·MCP 전체 흐름의 출시 준비도를 판정한다.

2) Input / Output:
- Input: 이전 단계 계약/산출물, 장치 matrix, test corpus, performance budget.
- Output: release gate report, known limitations, rollback criteria.

3) LLM Decision Area:
정성적 대화/RAG 품질을 평가하고 잔여 위험의 출시 허용 여부를 제안한다.

4) Code Processing Area:
unit/integration/E2E, cold/warm prefill·TTFT·decode·peak-memory benchmark, sustained thermal/energy test, offline test, network deny test, 격리된 Project Arena의 code build/test/run·requirement coverage·manual intervention 측정, migration/rollback, SBOM/license scan을 실행한다.

5) Success Criteria:
Android/iOS 각 2개 tier, desktop 3 OS, Linux server에서 필수 시나리오가 통과하고 P0/P1 결함이 없으며 estimated/measured 성능과 알려진 제한이 UI와 문서에 표시된다.

6) Validation Method:
CI matrix, physical-device test report, schema validator, human acceptance test, security checklist.

7) Failure Handling:
P0/P1 또는 데이터 유출 가능성은 release abort다. 특정 어댑터만 실패하면 feature flag로 비활성화하고 이전 catalog/runtime으로 rollback한다. 성능 미달은 추천 등급 하향 후 재검증한다.

8) Skills / Scripts:
- Skill: `local-llm-release-auditor`
- Script: `scripts/release_gate.rs`

9) Intermediate Artifact Rule:
`output/step08_release_gate.json`

### State Model
| State | Entry Condition | Exit Condition | Next State |
|---|---|---|---|
| `COLLECTING_REQUIREMENTS` | 플랫폼, 개인정보, 모델, 대화/RAG/MCP 요구를 수집 중 | 필수 요구와 가정이 기록됨 | `PLANNING` |
| `PLANNING` | 아키텍처와 단계별 계약을 설계 중 | 실행/검증 계획과 schema가 준비됨 | `RUNNING_SCRIPT` or `VALIDATING` |
| `RUNNING_SCRIPT` | 수집, 정규화, scoring, 검증 스크립트 실행 중 | 성공 또는 복구 불가 실패가 확인됨 | `VALIDATING` or `FAILED` |
| `VALIDATING` | 산출물 구조, 보안, 품질, 플랫폼 지원을 확인 중 | 판정 결과가 확정됨 | `DONE` or `NEEDS_USER_INPUT` or `FAILED` |
| `NEEDS_USER_INPUT` | 라이선스, 네트워크, 권한, 위험 수용 등 사람 판단 필요 | 사용자가 명시적으로 선택함 | `PLANNING` or `DONE` |
| `DONE` | 구현 가능한 설계와 검증 결과가 승인됨 | Terminal | none |
| `FAILED` | 무결성, 권한, 핵심 플랫폼 지원 등 복구 불가 조건 발생 | Terminal | none |

## 3. Implementation Spec

### Recommended Folder Structure
```text
/project-root
  AGENTS.md
  Cargo.toml
  /apps
    /desktop-web              # responsive web UI + desktop shell
    /android                  # native mobile host and UI
    /ios                      # native mobile host and UI
  /crates
    /domain                   # Project, Conversation, Model, KnowledgeBase
    /device-profile           # hardware/capability detection
    /model-catalog            # signed catalog, license, provenance
    /model-fit                # deterministic recommendation engine
    /memory-admission        # load/sustained safety and pressure guard
    /benchmark-core          # TTFT, throughput, memory, task completion
    /project-arena           # paired project runs and acceptance evidence
    /capability-router       # planner/implementer/reviewer model routing
    /model-store              # resumable download and verified storage
    /inference-core           # runtime-neutral session/stream contracts
    /runtime-litert           # LiteRT-LM FFI adapter
    /runtime-gguf             # GGUF/llama.cpp-family FFI adapter
    /runtime-mlx              # Apple Silicon optimized adapter
    /runtime-mobile           # MNN/ExecuTorch-style mobile adapter boundary
    /runtime-server           # optional server runtime adapter
    /rag-core                 # ingestion, index, retrieval, citations
    /mcp-gateway              # transports, policy, approval, sandbox
    /persistence              # SQLite migrations and encrypted secrets
    /local-api                # OpenAI-compatible + native management API
    /trusted-node             # secure pairing, routing, revocation
    /portable-workspace       # external-storage PoC and host trace policy
  /contracts
    openapi.yaml
    model-catalog.schema.json
    device-profile.schema.json
    chat-events.schema.json
    mcp-policy.schema.json
  /.agents
    /skills
      /model-catalog-curator
      /device-model-advisor
      /model-lifecycle-manager
      /conversation-runtime
      /local-rag-pipeline
      /mcp-permission-auditor
      /local-llm-release-auditor
  /output
  /scripts
  /docs
    /adr
    /security
    /model-support
```

### AGENTS.md Responsibilities
- 기본 구현 주체는 단일 Codex agent이며, 요구사항 → 계약 → 구현 → 검증 순서를 강제한다.
- 새 네이티브 코드는 Rust 우선이며 불가피한 C/C++ 의존성은 FFI 어댑터 내부에만 둔다.
- 모델 지원을 코드에 하드코딩하지 않고 versioned catalog와 runtime capability로 추가한다.
- 개인정보 외부 전송, 모델 라이선스 동의, MCP 위험 권한은 자동 승인하지 않고 사용자 확인 상태로 전환한다.
- 각 도메인 작업은 해당 `.agents/skills/<skill-name>/` 스킬로 라우팅하며 중간 산출물은 `output/stepNN_<name>.<ext>`로 저장한다.
- release 전 offline, artifact integrity, project isolation, MCP permission, mobile physical-device 검증을 필수화한다.

### Custom Agent Definitions
| Name | Path | Role | Required Fields |
|---|---|---|---|
| none | none | 초기에는 단일 Codex agent + skills/scripts로 충분하며 custom subagent 조정 비용을 만들지 않는다. | none |

### Skill and Script Inventory
| Name | Type | Role | Trigger Condition |
|---|---|---|---|
| `model-catalog-curator` | skill | 공식 메타데이터를 provenance 포함 catalog로 정규화 | 모델 버전 추가/갱신 시 |
| `device-model-advisor` | skill | 장치 profile과 목적에 맞는 variant 추천 | 설치 전 또는 장치 상태 변경 시 |
| `model-lifecycle-manager` | skill | 안전한 다운로드·검증·설치·삭제 흐름 | 모델 상태 변경 시 |
| `conversation-runtime` | skill | 채팅 계약, template, context 정책 검토 | 대화 기능/모델 어댑터 변경 시 |
| `local-rag-pipeline` | skill | 수집·인덱싱·검색·인용 품질 관리 | 지식 소스/embedding 변경 시 |
| `mcp-permission-auditor` | skill | MCP 설정과 최소 권한 정책 검토 | MCP 등록/권한/transport 변경 시 |
| `local-llm-release-auditor` | skill | cross-platform release gate | release 후보 생성 시 |
| `normalize_catalog.rs` | script | catalog 필드 정규화 및 schema 검증 | catalog 수집 후 |
| `score_fit.rs` | script | deterministic hard filter 및 Fit Score | 모델 추천 요청 시 |
| `validate_contracts.rs` | script | API/schema/dependency 계약 검사 | 계약 또는 adapter 변경 시 |
| `release_gate.rs` | script | 필수 검증 결과 집계 | release 전 |

### Skill Creation Rules

> 이 설계서에 정의된 모든 스킬은 구현 시 반드시 `skill-creator` 스킬(`/skill-creator`)을 사용하여 생성할 것.
> 직접 SKILL.md를 수동 작성하지 말 것 — 규격 불일치 및 트리거 실패의 원인이 됨.

skill-creator가 보장하는 규격:
1. SKILL.md frontmatter (`name`, `description`) 필수 필드 준수
2. `description`의 트리거 정확도 최적화 (eval 기반 optimization loop)
3. 스킬 저장 위치 `.agents/skills/<skill-name>/` 규격 준수
4. 폴더 구조 (`SKILL.md` + `scripts/` + `references/`) 규격 준수
5. Progressive disclosure: SKILL.md 본문 500줄 이내, 대용량 참조는 `references/`로 분리
6. 테스트 프롬프트 실행 및 품질 검증 완료

### Core Artifacts
| Path | Format | Producer | Purpose |
|---|---|---|---|
| `output/step01_capability_matrix.json` | json | Step 01 | 모델·런타임·플랫폼 공식 지원 근거 |
| `output/step02_architecture_contracts.md` | md | Step 02 | 컴포넌트, API, 보안 경계 |
| `output/step03_model_fit_report.json` | json | Step 03 | 장치별 추천 결과와 설명 |
| `output/step04_installation_manifest.json` | json | Step 04 | 설치 artifact 무결성과 provenance |
| `output/step05_conversation_contract.json` | json | Step 05 | 채팅 event/state 계약 |
| `output/step06_rag_evaluation.json` | json | Step 06 | 검색 및 인용 품질 결과 |
| `output/step07_mcp_threat_report.json` | json | Step 07 | MCP 정책 및 공격 fixture 결과 |
| `output/step08_release_gate.json` | json | Step 08 | 플랫폼별 출시 판정 |
| `output/step08_project_arena_report.json` | json | Step 08 | paired run, acceptance coverage, 개입·에너지 비용 |

### Product Architecture Decisions
| Decision | Choice | Rationale |
|---|---|---|
| Core language | Rust workspace | PC/server/mobile 공유 로직, 메모리 안전성, FFI 경계 관리 |
| Runtime strategy | capability-driven adapters | Gemma/Qwen 버전과 플랫폼 지원 변화에 대응 |
| Mobile execution | on-device mandatory | 사용자의 명시 요구; offline 및 privacy 보장 |
| Primary mobile path | LiteRT-LM adapter + GGUF adapter validation | Gemma edge 가속과 Qwen/범용 생태계를 모두 수용하되 지원은 실측 기반 |
| Persistence | SQLite + encrypted secret store | 단일 장치 로컬 우선, migration 및 검색 용이 |
| RAG | local hybrid retrieval | 벡터 단독 실패를 보완하고 문서를 외부 전송하지 않음 |
| MCP | permission gateway | 모델이 도구를 직접 실행하지 못하도록 decision과 execution 분리 |
| Model updates | signed remote catalog, cached offline | 최신 계열 지원과 공급망 안전성의 균형 |
| API | native management API + OpenAI-compatible inference subset | 자체 기능과 기존 프로젝트 연동을 동시에 지원 |
| Performance evidence | estimated + device-measured | 영상·community 수치를 일반화하지 않고 실제 장치 결과로 추천 재보정 |
| Session profiles | Quick/Balanced/Deep | 같은 family의 속도·품질·headroom trade-off를 사용자 의도로 선택 |
| New model onboarding | experimental fast-track | 출시 직후 모델을 기본 추천과 분리해 빠르게 검증하되 공급망·안정성 gate 유지 |
| Remote inference | paired Trusted Node | 고사양 PC/server를 모바일·노트북에서 안전하게 공유 |
| Agent execution | complexity and trust gates | one-shot 과신을 막고 plan-first, review, build/test/run을 강제 |
| Model comparison | paired Project Arena | 동일 요구·환경에서 결과물, 누락, 수정 비용을 비교 |
| Task routing | verified capability router | 계획·구현·검토 강점이 다른 모델을 단계별 선택 |
| Portable mode | encrypted external workspace PoC | 저사양·이동 환경을 지원하되 host 무흔적을 과장하지 않음 |

### Required Model Support Matrix
| Family | Baseline artifact | Runtime candidates | Desktop/Server | Mobile | Special contract |
|---|---|---|---|---|---|
| Qwen3 | official GGUF, pinned quant/revision | GGUF, MLX, validated mobile adapter | Required | Small variants required | thinking/non-thinking mode, tool schema |
| DeepSeek-R1 | Distill-Qwen-7B pinned artifact | GGUF, server adapter | Required | Conditional after physical-device gate | reasoning stream separation, token budget |
| Gemma 4 | official deployable E2B/E4B artifact | LiteRT-LM, GGUF where verified | Required | Required for a verified small variant | modality and accelerator capability |
| Mistral Small 3 | 24B Instruct pinned artifact | GGUF, server adapter | Required | Explicitly unsupported in v1 unless gate passes | retired lifecycle badge, successor hint |
| Phi-4 Mini | 3.8B Instruct pinned artifact | GGUF, LiteRT-LM where verified | Required | Required for a verified quant | safe context cap, function calling capability |

각 cell은 `supported`, `experimental`, `blocked`, `retired-compatible`, `unverified` 중 하나와 검증 날짜를 가져야 한다. 모델 family 지원과 개별 artifact 지원을 분리하고, community quant는 checksum과 재현 benchmark가 없으면 `Verified`로 승격하지 않는다.

### Phased Delivery
| Phase | Deliverable | Exit Gate |
|---|---|---|
| 0. Feasibility | 5개 필수 모델 PC/서버 benchmark와 Qwen3/Gemma 4/Phi-4 Mini/DeepSeek 7B 모바일 후보 검증 | RAM, 속도, 열, 안정성 및 명시적 blocked 상태 확보 |
| 1. Local Chat MVP | 장치 진단, 5-family catalog, 설치, 스트리밍 대화, 저장/검색/삭제 | 5개 PC/서버 E2E, 모바일 필수 variant, artifact integrity 통과 |
| 2. RAG | 파일/URL/폴더 수집, hybrid retrieval, citation, Model Lab 확장 | 한국어 retrieval 및 task-completion 평가와 project isolation 통과 |
| 3. MCP | 등록, tool discovery, 승인, sandbox/audit, agent trust levels | 공격 fixture, 권한 replay, supervised completion 테스트 통과 |
| 4. Ecosystem | OpenAI 호환 API, Trusted Node, Portable Workspace PoC, 추가 adapter | pairing 보안, contract suite, cross-platform release gate 통과 |

### MVP Implementation Slice (2026-09-01)

이번 구현은 Phase 1의 위험을 가장 빨리 검증하는 단일 수직 흐름이다. 전체 출시판과 구분하며 Android/iOS 네이티브 추론을 완료한 것으로 표시하지 않는다.

```text
Responsive PWA
  -> loopback Rust HTTP/SSE API
    -> Device Profiler + Deterministic Fit Engine
    -> Embedded Five-family Catalog
    -> Ollama Runtime Adapter
    -> Atomic JSON Conversation Store
```

| 영역 | MVP 구현 | 출시판으로 가기 위한 남은 게이트 |
|---|---|---|
| 장치 진단 | OS, arch, logical CPU, total RAM, runtime reachability | GPU/NPU, disk, thermal, battery, swap 측정 |
| 모델 catalog | Qwen3, DeepSeek-R1, Gemma 4, Mistral Small 3, Phi-4 Mini의 검증 runtime tag | signed remote catalog, revision/checksum/license manifest |
| 추천 | peak RAM/total RAM hard threshold와 이유 | micro-benchmark, KV/context, sustained thermal 보정 |
| 설치 | Ollama pull NDJSON을 SSE progress로 전달 | 독립 resumable store, checksum/signature, quarantine |
| 대화 | 멀티턴 token SSE, stop 시 upstream 종료, atomic JSON 저장 | context compaction, branching/search, latency metrics |
| API | native status/models/install/chat/conversations, `/v1/models` subset | OpenAI chat/responses, Anthropic, conformance suite |
| 보안 | `127.0.0.1` 강제, 실행별 token, catalog allowlist, CORS 미허용 | encrypted secret store, rate/body limits, Trusted Node pairing |
| 모바일 | responsive installable PWA와 동일 domain contract | Android/iOS 앱 내장 Rust runtime adapter와 physical-device gate |
| RAG/MCP | 미구현 | Phase 2/3에서 persistent RAG와 permission gateway 구현 |

MVP runtime은 교체 가능한 경계를 검증하기 위해 Ollama HTTP API를 사용한다. 이는 최종 runtime 독점 결정이 아니며, `runtime.rs` 밖의 catalog·추천·대화 저장·UI가 Ollama의 응답 형식을 직접 알지 못하게 한다. 모델 tag는 2026-09-01 확인한 Ollama library 값이며 source URL을 catalog 각 항목에 기록한다.

### MVP API Contract

| Method | Path | Purpose | Security |
|---|---|---|---|
| `GET` | `/api/status` | 장치와 runtime 상태 | `X-Local-Token` |
| `GET` | `/api/models` | catalog 및 장치별 추천 | `X-Local-Token` |
| `POST` | `/api/models/{model}/install` | allowlisted 모델 pull SSE | `X-Local-Token` |
| `POST` | `/api/chat` | 로컬 runtime chat SSE와 대화 저장 | `X-Local-Token` |
| `GET` | `/api/conversations` | 최근 대화 복원 | `X-Local-Token` |
| `DELETE` | `/api/conversations/{id}` | 대화 삭제 | `X-Local-Token` |
| `GET` | `/v1/models` | 설치 모델의 OpenAI 형식 subset | `X-Local-Token` |

SSE event는 설치에서 `progress`, `done`, `error`, 채팅에서 `meta`, `token`, `metrics`, `done`, `error`를 사용한다. UI는 runtime 고유 NDJSON을 직접 소비하지 않고 Rust API가 정규화한 event만 처리한다.

### MVP Acceptance Criteria

- Ollama가 중지된 상태에서도 앱이 시작되고 진단 화면에 명확한 복구 지침을 표시한다.
- 4 GB fixture에서 Mistral Small 3가 `blocked`이고 Qwen3 소형이 최상위가 되는 deterministic test를 통과한다.
- catalog 외 model ID를 설치 endpoint가 거절한다.
- token 없는 관리·대화 API 요청은 `401`이며 non-loopback bind는 프로세스 시작 전에 거절한다.
- upstream pull/chat 오류는 `502`로 변환되고 UI가 실패 상태에서 재시도 가능하다.
- assistant 응답 완료 후 대화가 임시 파일 쓰기와 rename으로 저장되고 재시작 후 복원된다.
- `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`를 통과한다.
- 모바일 PWA는 화면 대응 완료로만 판정하며, 네이티브 오프라인 inference가 통과하기 전 제품 전체의 모바일 성공 조건은 미달 상태로 유지한다.

### Key Risks and Mitigations
| Risk | Impact | Mitigation |
|---|---|---|
| 최신 모델의 모바일 runtime 지원 지연 | 설치는 가능하나 실행 실패 | catalog에서 모델 family와 deployable artifact를 분리하고 `unverified` 차단 |
| 모바일 OOM·발열·배터리 소모 | 앱 종료, UX 악화 | safety margin, 짧은 micro-benchmark, thermal throttling, 작은 variant fallback |
| C/C++ runtime 의존성 | Rust-only 목표와 충돌 | Rust FFI wrapper, 최소 unsafe surface, sanitizer 및 ABI test |
| 공급망 변조 | 악성 모델/바이너리 실행 | signed catalog, pinned revision, checksum, origin allowlist, SBOM |
| RAG 환각 또는 잘못된 인용 | 신뢰 저하 | citation integrity, 근거 부족 응답, retrieval evaluation |
| MCP prompt injection | 데이터/시스템 피해 | tool result 비신뢰 처리, allow/ask/deny, sandbox, secret scoping, human approval |
| 모바일 앱스토어 제한 | 임의 코드/대형 파일 배포 제약 | 원격 다운로드 정책 준수, 모바일 임의 프로세스 실행 금지, 앱 내장 runtime만 사용 |
| 5개 모델의 서로 다른 chat/tool 계약 | 출력 파싱 오류와 MCP 오작동 | family별 template conformance suite와 capability-gated UI |
| Mistral Small 3 retired 상태 | 신규 사용자에게 구형 모델 추천 | 호환성 badge, 기본 추천 하향, 후속 모델 migration 안내 |
| community/video benchmark 일반화 | 잘못된 모델·장비 추천 | 출처 badge, 재현 조건 저장, 해당 장치 micro-benchmark로 재보정 |
| 원격 노드 노출 | 대화·문서 유출 또는 무단 추론 | mutual pairing, TLS, scoped token, revoke, local/VPN 기본 |
| USB/portable 무흔적 오해 | host 임시 데이터 노출 | 명시적 trace warning, workspace 암호화, clean-exit 검사, 무흔적 보장 금지 |
| local agent 과신 | 잘못된 설계·코드 자동 실행 | complexity classifier, plan-first, human/evaluator gate, sandbox build/test/run |
| 모델의 허위 완료 선언 | 기능 누락을 성공으로 처리 | requirement-to-test mapping, plan/code diff, acceptance test를 완료 기준으로 사용 |
| 단계별 모델 handoff의 문맥 유출·손실 | 개인정보 노출 또는 구현 불일치 | 최소 context package, 프로젝트 정책 필터, handoff schema와 checksum |
| 모델이 적재되지만 OS 여유가 없음 | swap, 앱 종료, 장기 성능 붕괴 | admission reserve, live pressure guard, context/quant fallback, safe unload |
| 출시 직후 모델의 과대평가 | 불안정 artifact를 기본 추천 | Experimental 격리, pinned provenance, 반복 실측, 승격 checklist |

## 4. Validation Checklist

- [ ] Every workflow step has all 9 required fields
- [ ] Intermediate artifacts use the `output/stepNN_<name>.<ext>` rule
- [ ] LLM vs code responsibilities are separated clearly
- [ ] Human review points are explicit where needed
- [ ] Codex skill paths use `.agents/skills/...`
- [ ] Codex custom subagents use `.codex/agents/*.toml` or explicitly document none
- [ ] Skill additions or updates mention `skill-creator`
- [ ] Local-first network and data boundaries are explicit
- [ ] Mobile on-device inference has feasibility and physical-device gates
- [ ] Model versions are catalog-driven rather than hardcoded
- [ ] MCP permission and RAG citation security checks are explicit
