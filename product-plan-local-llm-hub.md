# Local LLM Hub 제품 기획서

> 업데이트: 2026-08-21  
> 기준 영상: [구독료 0원, 내 PC에서 돌아가는 LLM 5개](https://www.youtube.com/shorts/4fmdeK2Hf6I)  
> 연계 설계: `blueprint-local-llm-hub.md`
> 적용 가이드 조사: `video-research-local-llm-guides.md`

## 1. 제품 한 줄 정의

내 장치 사양을 분석해 적합한 로컬 LLM을 추천·설치하고, ChatGPT처럼 대화하면서 문서 RAG와 MCP 도구를 프로젝트별로 연결하는 PC·서버·모바일용 로컬 AI 허브다.

## 2. 해결할 문제

- 모델마다 크기, 양자화, 실행 엔진, chat template이 달라 설치가 어렵다.
- 영상이나 커뮤니티에서 “실행된다”는 표현만으로 내 장치에서 안정적으로 작동할지 판단하기 어렵다.
- RAG와 MCP를 붙이려면 별도 프로그램과 보안 설정이 필요하다.
- 최신 모델과 오래됐지만 널리 쓰이는 모델을 프로젝트마다 일관된 API로 쓰기 어렵다.
- 모바일은 메모리, 발열, 배터리, 앱스토어 정책 때문에 PC와 같은 설치 전략을 쓸 수 없다.

## 3. 목표 사용자

### 1차 사용자

- 구독형 AI 대신 로컬 모델을 쓰려는 개인 개발자
- 민감한 문서를 외부로 보내지 않으려는 연구자와 소규모 팀
- 기존 프로젝트에 OpenAI 호환 API로 로컬 모델을 연결하려는 개발자
- Android/iOS에서 오프라인 AI 기능을 구현하려는 앱 개발자

### 핵심 사용 시나리오

1. 앱 설치 후 장치 진단을 실행한다.
2. 목적을 `일반 대화`, `코딩`, `추론`, `문서 분석`, `도구 사용` 중에서 선택한다.
3. 앱이 실행 가능한 모델과 예상 RAM·저장공간·속도를 비교한다.
4. 라이선스와 다운로드 크기를 확인하고 모델을 설치한다.
5. ChatGPT 형태로 대화하며 필요하면 프로젝트 문서와 MCP 도구를 연결한다.
6. 같은 프로젝트를 PC·서버·모바일에서 각 장치에 맞는 모델로 연다.
7. 장치에서 직접 측정한 TTFT, 생성 속도, peak memory와 작업 완주율을 비교한다.
8. 노트북·모바일에서 더 강한 내 PC/서버를 `Trusted Node`로 연결해 원격 추론한다.
9. 복잡한 agent 작업은 계획을 작은 단계로 나누고 실행·테스트 결과를 확인한다.

## 4. 초기 필수 지원 모델 5개

| 모델군 | 기준 variant | 제품 내 역할 | 우선 플랫폼 | 주요 capability | 제품 정책 |
|---|---|---|---|---|---|
| Qwen3 | 0.6B/1.7B/4B/8B/14B 중 검증 variant | 한국어·다국어·코딩·일반 대화 | 모바일/PC/서버 | thinking 전환, tool use, 공식 GGUF | 소형은 모바일 우선, 14B는 PC GPU 우선 |
| DeepSeek-R1 | Distill-Qwen-7B | 수학·논리·단계적 추론 | 고사양 모바일 조건부/PC/서버 | reasoning output | thinking content 표시·숨김 정책 필요 |
| Gemma 4 | E2B/E4B 및 검증 variant | 모바일 일반 대화·멀티모달 | 모바일/PC/서버 | vision 등 variant별 modality | LiteRT-LM 우선, iOS/Android capability 별도 검증 |
| Mistral Small 3 | 24B Instruct | RAG·agent·다국어 호환 | PC GPU/서버 | function calling, 32K context | retired 호환 모델로 표시, 모바일 설치 차단 |
| Phi-4 Mini | 3.8B Instruct | 경량 추론·CPU fallback | 모바일/PC/서버 | 128K 선언 context, function calling | 실제 context는 장치 메모리에 맞게 제한 |

### 지원의 정의

모델 목록에 이름만 보이는 것은 지원으로 보지 않는다. 각 모델은 다음 조건을 만족해야 `Verified`가 된다.

- 공식 또는 승인된 배포처와 revision이 고정되어 있다.
- 파일 크기, checksum, 라이선스, provenance를 확인할 수 있다.
- 적합한 runtime과 chat template이 등록되어 있다.
- 설치, 첫 토큰, 스트리밍, 중지, 재시작 후 대화 복원이 동작한다.
- RAG prompt 및 citation이 깨지지 않는다.
- tool calling을 지원하는 variant만 MCP 자동 호출을 활성화한다.
- 대상 플랫폼의 실제 장치 benchmark를 통과한다.

## 5. 모델별 사용자 경험

### Qwen3

- 기본 추천: 한국어 대화와 코딩을 함께 원하는 사용자.
- `빠른 응답`과 `깊이 생각하기` 모드를 명확히 분리한다.
- 모바일은 소형 양자화 variant만 표시한다.
- 14B 이상은 VRAM/RAM 실측 후 PC·서버에 추천한다.

### DeepSeek-R1 Distill 7B

- 기본 추천: 수학, 논리, 복잡한 문제 풀이.
- 추론 중간 출력은 접을 수 있는 별도 영역에 표시한다.
- 긴 reasoning으로 context와 배터리를 과도하게 쓰지 않도록 token budget을 제공한다.
- 일반 대화에는 지연과 verbosity 경고를 표시한다.

### Gemma 4

- 기본 추천: 모바일과 이미지 입력이 중요한 사용자.
- 텍스트/이미지 capability는 variant 및 runtime이 실제 지원할 때만 UI에 노출한다.
- Android와 iOS의 가속기 지원 상태를 별도 badge로 표시한다.

### Mistral Small 3

- 기본 추천: 충분한 RAM/VRAM을 가진 PC·서버의 RAG 및 agent 호환.
- 공식 lifecycle상 retired임을 표시하고 신규 프로젝트에는 후속 Mistral 모델도 제안한다.
- 기존 프로젝트 재현을 위해 pinned revision 설치와 보안 패치를 유지한다.

### Phi-4 Mini

- 기본 추천: GPU가 없거나 자원이 제한된 장치의 추론과 function calling.
- 모델의 최대 context 선언과 실제 장치에서 안전한 context를 구분해 표시한다.
- CPU-only 모드는 예상 속도와 배터리 영향을 설치 전에 안내한다.

## 6. 핵심 기능

### A. 모델 센터

- 5개 필수 모델군 필터 및 비교
- `최신`, `호환성`, `검증됨`, `실험적`, `지원 불가` badge
- 양자화별 다운로드 크기와 예상 peak RAM
- 중단 가능한 다운로드, 재개, checksum 검증, 안전한 삭제
- family별 작은 variant 및 후속 모델 추천

### B. 장치 진단과 자동 추천

- OS, architecture, RAM, 저장공간, CPU/GPU/NPU 탐지
- 30초 이내 micro-benchmark
- 목적별 Fit Score와 추천 이유
- weights, KV cache, runtime overhead, OS reserve를 분리한 메모리 예산
- dense/MoE의 total/active parameter 및 GPU full residency·partial offload 구분
- OOM 안전 여유, 발열, 배터리, sustained throttling 상태 반영
- 모델 설치 전 `권장/가능/비권장/불가` 판정
- 설치 전 `Estimated`, 첫 실행 후 `Measured on this device` 결과를 구분

### C. 대화

- 멀티턴 스트리밍 채팅
- 대화 저장, 검색, 이름 변경, 삭제
- 메시지 수정 후 대화 분기
- 대화 중 모델 전환
- thinking 영역, citation, MCP 호출 상태의 통합 event UI
- 프로젝트별 system instruction 및 생성 옵션

### D. 로컬 RAG

- PDF, Markdown, TXT, 웹페이지, 폴더 동기화
- 로컬 embedding과 hybrid retrieval
- 답변별 문서명·페이지·청크 citation
- 프로젝트별 인덱스 격리
- 변경된 파일만 증분 색인

### E. MCP

- 서버 등록 및 tool discovery
- 프로젝트별 `항상 허용/매번 확인/차단`
- 파일·네트워크·비밀 값 범위 제한
- 모바일에서는 임의 로컬 프로세스 실행 금지
- 도구 호출 내역과 승인 기록을 로컬 감사 로그로 보관

### F. 개발자 API

- OpenAI 호환 chat completions의 필수 subset
- 모델 설치·상태·추천을 위한 native management API
- SSE 또는 WebSocket streaming
- capability discovery endpoint

### G. Model Lab

- 동일 prompt·temperature·seed·quant·context 조건의 비교 실행
- cold/warm start, prefill, TTFT, decode, total completion time 측정
- peak RAM/VRAM, context/KV-cache 사용량, 열·배터리·실패율 기록
- 한국어, reasoning, coding, multi-turn, tool calling, RAG citation 평가셋
- coding 결과를 sandbox에서 build/test/run하고 task completion으로 판정
- community benchmark와 해당 장치 실측 결과를 시각적으로 분리

### H. 실행 모드와 원격 노드

- `On-device`: 소형 모델의 완전 로컬·offline 실행
- `Portable Workspace`: 외장 저장소에 암호화된 설정·대화·모델을 휴대하는 PoC
- `Trusted Node`: headless PC/server를 안전하게 pairing해 노트북·모바일에서 사용
- `Hybrid Optional`: 명시적으로 동의한 프로젝트만 고난도 작업을 cloud로 전달
- runtime은 `Simple` preset과 context, GPU offload, batch 등을 조절하는 `Advanced` mode 제공
- agent는 `Assist`, `Supervised`, `Autonomous Candidate` 신뢰 수준으로 표시

## 7. 플랫폼 정책

| 플랫폼 | 실행 방식 | 1차 검증 대상 |
|---|---|---|
| Android | 앱 내장 runtime, CPU/GPU/NPU capability | Qwen3 소형, Gemma 4 E2B/E4B, Phi-4 Mini, DeepSeek 7B 조건부 |
| iOS | 앱 내장 runtime, CPU/Metal capability | Qwen3 소형, Gemma 4 검증 variant, Phi-4 Mini, DeepSeek 7B 조건부 |
| macOS | Rust core + GGUF/LiteRT/MLX adapter | 5개 모델군 모두 |
| Windows | Rust core + GGUF/LiteRT adapter | 5개 모델군 모두 |
| Linux/Server | Rust core + GGUF/server adapter | 5개 모델군 모두 |

모바일 직접 실행은 제품 필수 조건이지만 모든 모델을 모바일에서 실행한다는 의미는 아니다. 각 플랫폼에서 안전하게 실행 가능한 variant를 제공하고, 부적합한 모델은 이유와 대안을 보여준다.

## 8. 정보 구조와 주요 화면

1. **온보딩**: 개인정보 원칙 → 장치 진단 → 목적 선택 → 첫 모델 추천
2. **모델 센터**: 모델군 비교 → variant 상세 → 라이선스 동의 → 설치 진행
3. **채팅**: 대화 목록 → 메시지 → thinking/citation/tool event → 모델 전환
4. **프로젝트**: 기본 모델 → 지식베이스 → MCP → 데이터 저장 정책
5. **장치**: hardware profile → Model Lab → context memory budget → 설치 모델 → 저장공간
6. **노드**: headless node 발견 → QR/코드 pairing → 권한 → latency → model routing
7. **개발자 설정**: local endpoint → API key → 호환 API 예제 → 로그

## 9. 성공 지표

- 첫 설치 후 첫 응답까지 중앙값 10분 이하
- 추천 모델의 첫 실행 성공률 95% 이상
- verified artifact의 checksum 실패 실행률 0%
- 모바일 지원 variant의 OOM 없는 20회 연속 대화 성공률 99% 이상
- RAG citation 유효성 100%, 한국어 retrieval 평가 목표치 별도 baseline 대비 개선
- 승인 없는 위험 MCP 호출 실행률 0%
- crash-free session 99.5% 이상
- benchmark 재실행 시 decode 속도 편차 15% 이내 또는 환경 변화 원인 표시
- Trusted Node의 승인 없는 접속 성공률 0%, 연결 중 평문 대화 전송률 0%
- coding 평가에서 text 완료와 실제 build/test 성공률을 별도 공개

## 10. 출시 단계

### Phase 0 — 기술 검증

- 5개 모델의 공식 artifact와 라이선스 manifest 작성
- PC 3 OS 및 Android/iOS 표본 장치 benchmark
- 모델별 chat template와 streaming event 검증
- TTFT/prefill/decode/peak-memory benchmark harness와 task-completion 평가셋

### Phase 1 — Local Chat MVP

- 장치 진단, 모델 센터, 설치, 스트리밍 채팅
- 5개 모델 PC/서버 지원
- 모바일 3개 우선 family의 verified variant 제공
- Simple/Advanced runtime preset과 context memory calculator 제공

### Phase 2 — RAG

- 파일/URL/폴더 수집
- 로컬 hybrid retrieval와 citation

### Phase 3 — MCP

- registry, permission gateway, sandbox, audit
- 모델별 tool-calling capability 적용

### Phase 4 — 생태계 확장

- OpenAI 호환 API 안정화
- 추가 Qwen/Gemma/Mistral/Phi/DeepSeek revision 자동 갱신
- 모바일 조건부 모델의 verified 범위 확대
- Trusted Node, Portable Workspace PoC, 프로젝트별 opt-in hybrid fallback 검증

## 11. 공식 근거와 주의사항

- [Qwen3 공식 로컬 실행 문서](https://github.com/QwenLM/Qwen3/blob/main/docs/source/run_locally/llama.cpp.md)는 공식 GGUF와 llama.cpp 경로를 안내한다.
- [DeepSeek-R1 공식 저장소](https://github.com/deepseek-ai/DeepSeek-R1)는 Distill-Qwen-7B와 모델별 기반 라이선스를 설명한다.
- [Gemma 공식 릴리스](https://ai.google.dev/gemma/docs/releases)와 [LiteRT-LM](https://github.com/google-ai-edge/LiteRT-LM)을 모바일 실행 검증의 기준으로 사용한다.
- [Mistral Small 3 발표](https://mistral.ai/news/mistral-small-3)는 24B와 Apache 2.0을 명시하지만, [현재 모델 문서](https://docs.mistral.ai/models/mistral-small-3-0-25-01)는 retired 상태와 후속 모델 사용을 안내한다.
- [Phi-4 Mini 공식 모델 카드](https://huggingface.co/microsoft/Phi-4-mini-instruct)는 3.8B급, 128K context, MIT 라이선스와 자원 제한 환경 용도를 명시한다.
- 영상의 “무료”, benchmark, 특정 GPU 실행 주장은 사용자의 실제 장치 및 정확한 artifact에 대한 보증이 아니다. 설치 시 개별 라이선스와 실측 결과를 다시 표시한다.
