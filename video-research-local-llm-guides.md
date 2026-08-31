# 로컬 LLM 적용 가이드 영상 조사

> 작성일: 2026-08-21  
> 목적: 사용자 제공 영상에서 제품 기획과 기술 설계에 반영할 실용적인 아이디어를 추출한다.  
> 연계 문서: `product-plan-local-llm-hub.md`, `blueprint-local-llm-hub.md`

## 1. 조사 방법과 신뢰도

- 5개 영상의 장면별 음성과 화면을 분석했다.
- `Rb0OVBaKYdQ`는 장면 분석이 두 차례 실패하여 YouTube 한국어 자동 자막으로 보완했다.
- 영상 속 속도, VRAM, 가격, benchmark 수치는 촬영 장치·runtime·quant·prompt에 종속된다.
- 영상에서 언급된 모델명 중 `Qwen 3.8`처럼 공식 명칭과 다를 가능성이 있는 표기는 원문 그대로 기록하되 제품 카탈로그에는 넣지 않는다.
- 제품 반영 시 공식 model card, runtime 문서, pinned artifact와 자체 benchmark를 최종 근거로 사용한다.

### 판단 등급

| 등급 | 의미 |
|---|---|
| 채택 | 여러 영상과 기존 요구사항이 일치하고 제품 가치가 명확함 |
| PoC | 가치가 있지만 플랫폼·보안·성능 실측이 필요함 |
| 참고 | 특정 장비 또는 개인 경험이므로 제품 정책의 직접 근거로 쓰지 않음 |
| 제외 | 보안·정확성·유지보수 위험이 커서 현재 범위에 넣지 않음 |

## 2. 주제별 분류

| 분류 | 관련 영상 | 핵심 결론 | 판단 | 문서 반영 |
|---|---|---|---|---|
| 모델 실사용 평가 | 1, 2, 4, 5 | parameter 수와 benchmark만으로 실제 속도·완주 능력을 예측할 수 없음 | 채택 | Model Lab, task completion 평가 |
| 장치 적합성 | 2, 3, 4, 5 | VRAM/RAM, 전체 weights, KV cache, context, offload 여부가 핵심 | 채택 | Fit Engine v2 |
| 양자화 선택 | 2, 5 | Q4 계열이 흔한 시작점이지만 장치·모델별 측정 필요 | 채택 | quant recommendation 및 품질 경고 |
| 저사양·휴대 실행 | 3 | 외장 저장장치에서 앱·모델·설정을 함께 운영 가능 | PoC | Portable Workspace mode |
| 원격 추론 | 2, 4, 5 | 무거운 모델은 headless 서버에서 실행하고 노트북·모바일은 client로 사용 | 채택 | Trusted Node, remote routing |
| runtime 단계화 | 1, 2, 4, 5 | 쉬운 경로는 Ollama형 UX, 고급 경로는 llama.cpp형 제어 | 채택 | runtime presets와 Advanced mode |
| 에이전트 신뢰 범위 | 1, 2, 4 | 간단한 구현은 가능하지만 복잡한 설계·장기 작업은 분해와 검증 필요 | 채택 | task complexity gate |
| 한국어·도구 호출 | 1, 2 | 자연어 품질과 tool-call 안정성은 별도 평가해야 함 | 채택 | locale/tool benchmark |
| hybrid local/cloud | 4, 5 | 로컬은 반복·민감 작업, cloud는 고난도 검증에 선택적으로 사용 | PoC | explicit opt-in fallback |
| uncensored model | 3 | 안전장치 제거를 제품 기능으로 홍보하는 것은 위험 | 제외 | 별도 추천/프롬프트 제공하지 않음 |
| 실제 프로젝트 A/B 비교 | 6 | 동일 요구·환경에서 산출물, 누락, 수정 비용까지 비교해야 개인 적합성을 알 수 있음 | 채택 | Project Arena, requirement coverage |
| 단계별 모델 라우팅 | 6 | 계획이 자세한 모델과 구현이 빠른 모델이 다를 수 있음 | PoC | planner/implementer/reviewer route |
| 메모리 한계 근접 실행 | 7 | 실행 가능 여부와 안정적으로 쓸 수 있는지는 다르며 swap·headroom을 실시간 확인해야 함 | 채택 | Memory Admission Controller |
| 빠른/깊은 작업 프로필 | 7 | 같은 family의 소형 Dense와 중형 MoE를 작업 깊이에 따라 전환할 가치가 있음 | 채택 | Quick/Deep profile |

## 3. 영상별 정리

### 영상 1 — Mac Mini 32GB 모델 비교

- 링크: [맥미니 32gb 램으로 로컬 LLM 종류별로 돌려봤습니다](https://www.youtube.com/watch?v=DgMLiIkDvcM)
- 분류: 모델 비교, 코드 실행 검증, prompt adherence
- 환경: Mac Mini 32GB, Ollama 기반 실험

주요 관찰:

- `0:19–0:38`: 수학, Python 이해, 문서 이해처럼 같은 prompt set으로 모델을 비교한다.
- `1:23–3:20`: Qwen3 8B/14B/30B의 속도가 크기에 비례하지 않는다. 특정 14B가 더 느린 사례가 나온다.
- `3:20–4:42`: coding 특화 모델은 일반 수학·요약에서 약할 수 있고, DeepSeek는 정답과 별개로 출력 형식 준수 문제가 나타난다.
- `5:41–11:46`: 생성된 Todo 코드를 실제 실행해 CRUD, persistence, ID 충돌을 검증한다.

반영:

- 채택: 모델 평가는 `정답률 + prompt adherence + latency + 실행 검증 + task completion`으로 구성한다.
- 채택: coding 모델도 일반 대화 추천과 분리된 workload profile을 가진다.
- 채택: 코드 생성 평가는 단순 text judge가 아니라 sandbox에서 build/test/run한다.
- 참고: 영상의 모델별 순위와 초 단위 결과는 단일 장치 측정이므로 제품 기본 점수로 사용하지 않는다.

### 영상 2 — 하드웨어 선정, LM Studio 서버, Hermes 연동

- 링크: [이거 모르고 장비 사면 돈 날립니다 | 로컬 LLM 현실 + 구축 가이드](https://www.youtube.com/watch?v=Rb0OVBaKYdQ)
- 분류: 하드웨어 구매, quant/context, headless inference, remote client, agent 평가
- 분석 보완: 장면 분석 실패로 YouTube 자동 자막 사용

주요 관찰:

- `2:07–3:10`: 장비 → 모델 → 실사용 검증 순서로 선택하며 parameter보다 VRAM 적재와 목적 적합성을 강조한다.
- `4:13–6:31`: VRAM, 시스템 RAM, memory bandwidth, CPU 순으로 중요도를 설명하고 agent 수준을 단발 호출·자율 완주·장기 multi-turn으로 나눈다.
- `12:10–12:56`: quant는 크기와 품질의 trade-off이며 작은 모델의 적절한 quant가 큰 모델의 과도한 저정밀 quant보다 나을 수 있다고 본다.
- `15:39–16:27`: 자동 추천은 후보 축소용이고 속도 추정치는 반드시 실측해야 한다.
- `18:14–20:50`: MoE도 전체 weights를 메모리에 올려야 하며 KV cache와 context가 남는 메모리를 소비한다.
- `20:52–23:56`: 속도를 prefill, TTFT, decode로 분리하고 coding, multi-turn, tool call, output quality를 함께 평가한다.
- `26:05–33:17`: headless Linux에서 runtime·model·context를 설정하고 OpenAI 호환 local API를 제공한다.
- `33:19–41:44`: 원격 PC와 모바일이 서버 모델을 사용하는 thin-client 패턴을 보여준다.

반영:

- 채택: hardware advisor에 workload와 예산, VRAM residency, memory bandwidth, KV cache 계산을 추가한다.
- 채택: 추천 결과에 `estimated`와 `measured`를 구분한다.
- 채택: 성능을 TTFT, prefill t/s, decode t/s, total completion time으로 표시한다.
- 채택: headless inference node, secure pairing, remote client routing을 제품 핵심 모드로 추가한다.
- 채택: agent capability를 `assist`, `supervised`, `autonomous-candidate` 단계로 표시한다.
- 참고: 특정 GPU 가격·비교·추천은 시점 의존적이므로 제품 구매 추천에는 실시간 데이터가 없으면 사용하지 않는다.

### 영상 3 — 저사양 노트북과 USB 휴대형 AI

- 링크: [저사양 노트북 USB에 '평생 무료' AI 설치하는 법](https://www.youtube.com/watch?v=izY5qKA2p8g)
- 분류: portable deployment, offline, 저사양 fallback
- 환경: 8GB RAM Ryzen 5 노트북, 32GB 이상 USB 제안

주요 관찰:

- `0:13–1:40`: 앱·모델을 이동식 저장장치에 넣어 오프라인·개인정보 보호 용도로 사용한다.
- `1:43–3:42`: 저장장치 성능과 파일시스템, 긴 설치 시간, batch installer를 사용한다.
- `4:18–5:21`: hardware 기반 자동 추천 후에도 실제 메모리 부족으로 첫 모델 실행이 실패한다.
- `5:24–7:05`: 작은 text model로 교체하고 네트워크를 끊어 offline 실행을 확인한다.

반영:

- PoC: `Portable Workspace`는 앱 설정·대화·모델을 외장 저장소에 담되 OS별 runtime은 별도 패키지로 둔다.
- 채택: 첫 모델 실행 실패 시 자동으로 더 작은 quant/model을 추천한다.
- 채택: 설치 전 저장장치 속도, 공간, filesystem, 모델 lock 상태를 진단한다.
- 채택: offline verification test를 온보딩에서 실행할 수 있게 한다.
- 제외: 출처 불명의 batch installer와 `uncensored` system prompt를 제품이 제공하지 않는다.
- 주의: USB에 저장해도 host의 RAM, 임시 파일, OS 로그에 데이터가 남을 수 있으므로 “USB에만 남는다”고 보장하지 않는다.

### 영상 4 — 로컬 coding agent의 한계와 작업 분해

- 링크: [Qwen 3.8 Locally: How Much Can You Actually Trust It?](https://www.youtube.com/watch?v=dTutfoSVMq4)
- 분류: coding agent, task complexity, remote inference, human verification
- 명칭 주의: 영상의 `Qwen 3.8` 표기는 공식 catalog에서 별도 검증 전 사용하지 않는다.

주요 관찰:

- `0:38–1:26`: 로컬 모델을 cloud 완전 대체가 아니라 routine/private task용 보조 도구로 정의하고 데스크톱 추론을 노트북에서 원격 사용한다.
- `2:07–2:33`: 재사용 가능한 prompt repository와 단순 agent client를 쓴다.
- `2:33–5:44`: 단순·중간 프런트엔드 작업은 one-shot으로 완주한다.
- `5:44–7:47`: frontend/backend/database가 있는 복잡한 시스템 계획은 외부 검토에서 치명적 누락이 발견된다.
- `7:53–9:31`: 수정된 계획을 작은 작업으로 나누자 비주류 언어의 동작 가능한 prototype을 완성한다.

반영:

- 채택: 요청 복잡도를 평가해 one-shot, plan-first, mandatory-review 흐름으로 라우팅한다.
- 채택: 복잡한 아키텍처 계획은 자동 실행 전에 human 또는 별도 evaluator gate를 통과한다.
- 채택: agent가 생성한 코드는 build/test/run 결과를 완료 조건으로 사용한다.
- PoC: 사용자가 허용한 경우에만 cloud evaluator를 사용하며 전달할 파일과 내용을 미리 표시한다.

### 영상 5 — 64GB Mac, Qwen 27B, quant/context, local-cloud 분업

- 링크: [64GB RAM으로 돌리는 로컬 LLM으로 저렴하게 코딩 AI 쓸 수 있다고!?](https://www.youtube.com/watch?v=fJN0UJSTM1o)
- 분류: quant 추천, context memory, runtime 단계화, remote desktop
- 환경: 64GB Apple Silicon을 중심으로 한 설명

주요 관찰:

- `1:31–2:52`: 장치 메모리 안에 모델, OS, 개발 도구, KV cache가 함께 들어가야 한다.
- `2:55–3:50`: token/s와 대화 체감 속도를 연결하지만 cloud 완전 대체보다는 보조 도구로 제안한다.
- `4:44–6:05`: 빠른 시작은 Ollama, 세밀한 context·GPU offload·multimodal 제어는 llama.cpp로 나눈다.
- `6:57–7:30`: laptop의 발열·팬 소음을 피하기 위해 전용 데스크톱에 모델을 두고 원격 접속한다.
- `8:14–8:41`: routine coding/review는 local, 어려운 문제는 cloud로 분리한다.

반영:

- 채택: `Simple` runtime preset과 `Advanced` tuning mode를 제공한다.
- 채택: context slider는 KV cache 예상량과 다른 앱에 남길 메모리를 즉시 보여준다.
- 채택: sustained workload benchmark에 온도·throttling·배터리·소음을 포함한다.
- PoC: cloud fallback은 기본 off이고 프로젝트별 opt-in, redaction preview, 비용 제한을 갖는다.
- 참고: 영상의 tokens/s와 비용 절감 비율은 해당 환경의 사례로만 저장한다.

### 영상 6 — Qwen 3.6과 Gemma 4의 동일 앱 구현 비교

- 링크: [Qwen 3.6 vs Gemma 4: I Built the Same App With Both Locally](https://www.youtube.com/watch?v=Um8Px55mINc)
- 분류: 실제 프로젝트 A/B 평가, coding agent, 계획-구현 일치도, 에너지 사용
- 환경: 데스크톱 GPU에서 모델을 실행하고 네트워크를 통해 agent client를 사용
- 과제: Tauri 기반 Markdown viewer/editor를 동일 요구사항으로 구현

주요 관찰:

- `0:15–0:44`: 추상 benchmark의 객관적 1등보다 자신의 task와 hardware에 맞는 모델을 찾는 것을 목표로 한다.
- `1:27–2:26`: 동일 기술 스택과 환경에서 Qwen 3.6 27B와 Gemma 4 31B dense 모델을 비교한다.
- `3:08–3:40`, `6:32–7:10`: 두 모델 모두 phase와 작은 task로 계획하지만 Qwen 쪽 계획이 더 상세하다.
- `4:04–6:13`: Qwen은 구현 중 자기 오류를 찾았지만 완료까지 오래 걸리고 수동 port/API 수정이 필요했으며 일부 toolbar가 작동하지 않았다.
- `7:33–9:26`: Gemma는 약 두 배 빠르게 끝났고 파일 구조가 더 좋았지만 filesystem plugin 설정이 빠졌고 계획한 일부 UI 기능을 생략했다.
- `7:33–7:46`: 장시간 로컬 추론의 전력 사용 증가가 언급된다.
- `9:26–10:35`: 어느 모델도 절대 승자가 아니며 상세한 계획과 빠른 구현·구조화 사이에 trade-off가 있다.

반영:

- 채택: Model Lab에 동일 실제 프로젝트를 격리된 workspace에서 실행하는 `Project Arena`를 추가한다.
- 채택: 결과 평가는 생성 시간뿐 아니라 requirement coverage, plan-to-code traceability, 첫 실행 성공, 수동 수정 횟수·시간, test pass, 저장소 구조를 포함한다.
- 채택: “완료했다”는 모델의 선언 대신 요구사항별 acceptance test로 누락을 검출한다.
- 채택: 생성 중 model self-correction과 사용자 개입을 event로 기록해 실질 자동화 비용을 계산한다.
- 채택: 장시간 benchmark에는 소비 전력·에너지와 thermal throttling을 포함한다.
- PoC: 프로젝트 단계별로 planner, implementer, reviewer 모델을 다르게 선택하는 capability router를 검증한다.
- 참고: 영상의 46분, 약 2배 속도 등은 해당 hardware·prompt의 사례이며 제품 기본 성능으로 사용하지 않는다.

### 영상 7 — Ornith 1.5 9B/35B의 24GB MacBook 실행

- 링크: [Ornith 1.5 로컬 실행 영상](https://www.youtube.com/watch?v=fP9hBrweli4&list=PLX56MOPpu7GeKI8UneJsfXZfLwj0SV3Ko)
- 분류: 신규 모델 first run, Dense/MoE 비교, unified memory 한계, 속도·품질 trade-off
- 환경: MacBook M4 Pro, 24GB unified memory
- 공식 참고: [Ornith 1.5 9B model card](https://huggingface.co/ornith-ai/Ornith-1.5-9B)

주요 관찰:

- `0:08–0:34`: 9B Dense 아티팩트는 약 6.5GB로 소개되며 짧은 로컬 질의에 빠른 응답을 보인다.
- `0:34–0:57`: 약 23GB인 35B MoE를 24GB 통합 메모리에 올리면서 다른 프로세스를 위한 여유가 거의 사라진다.
- `0:57–1:10`: 영상 환경에서 35B 모델은 약 5–8 tokens/s로 생성되지만 단일 first-run 사례다.
- `1:10–1:40`: 복잡한 reasoning에서는 35B가 더 일관되고 세밀하다는 주관적 관찰이 있다.
- `1:40–2:07`: “메모리에 들어간다”는 사실을 실행 가능성의 근거로 보지만 장시간 안정성·swap·앱 병행 사용은 검증하지 않는다.

반영:

- 채택: 모델 load 직전 정적 계산뿐 아니라 load 중·대화 중 memory pressure, swap, 앱 headroom을 감시한다.
- 채택: `Fits`와 `Safe for sustained use`를 분리하며 OS·다른 앱에 남길 최소 메모리를 사용자가 설정할 수 있게 한다.
- 채택: 임계치 초과 시 context 축소, 작은 quant/variant 전환, 모델 unload 순으로 안전하게 대응한다.
- 채택: 같은 family 내 `Quick` 소형 모델과 `Deep` 대형/MoE 모델을 대화별로 전환하는 profile을 제공한다.
- PoC: Ornith 1.5는 `Experimental/Fast-track candidate`로 catalog에 등록해 provenance, license, runtime, chat/tool template, 반복 benchmark를 통과하면 승격한다.
- 참고: 5–8 tokens/s, 무소음, 24GB 적재 가능성은 영상 장치의 단기 관찰로만 저장한다.
- 제외: 9B가 특정 대형 모델보다 우수하다는 vendor benchmark만으로 기본 추천하거나 모바일 지원을 선언하지 않는다.

## 4. 공통으로 도출된 제품 요구사항

### Model Lab

- 같은 prompt suite, seed, temperature, quant, context로 모델을 비교한다.
- cold/warm start, prefill, TTFT, decode, peak RAM/VRAM, energy, thermal을 분리 측정한다.
- 한국어, 일반 추론, coding, 긴 문서, multi-turn, tool calling, RAG citation을 평가한다.
- coding 결과는 sandbox에서 실제 build/test/run한다.
- 실제 프로젝트 A/B 실행 시 요구사항을 acceptance test와 연결하고 plan 대비 구현 누락을 계산한다.
- 사람의 수정 횟수·시간, model self-correction, retry, 최초 실행 성공 여부를 기록한다.
- 세 번 이상 반복하고 평균뿐 아니라 편차와 실패율을 기록한다.
- 영상·community 결과는 `Community Evidence`, 자체 결과는 `Measured on this device`로 분리한다.

### Device Fit Engine v2

- weights와 KV cache, runtime overhead, OS reserve, 동시 실행 앱을 함께 계산한다.
- dense/MoE의 total parameter와 active parameter를 분리한다.
- GPU full residency, partial offload, CPU-only 상태를 구분하고 성능 절벽을 경고한다.
- 추천은 설치 전 추정치이며 첫 실행 benchmark 후 재보정한다.
- `artifact fits`, `session starts`, `safe sustained use`를 별도 상태로 관리하고 swap·memory pressure를 지속 감시한다.

### Execution Modes

| Mode | 목적 | 기본 정책 |
|---|---|---|
| On-device | 완전 offline, 낮은 latency | 장치에 맞는 small model만 |
| Portable Workspace | 외장 저장장치에 앱 데이터와 모델 휴대 | 저장장치 진단, host 흔적 경고, 암호화 |
| Trusted Node | PC/server가 추론, 모바일/노트북이 client | pairing, TLS, access token, local/VPN 우선 |
| Hybrid Optional | local routine + cloud high complexity | 기본 off, 명시 동의, 전송 preview와 비용 제한 |

### Session Profiles

| Profile | 선택 기준 | 기본 동작 |
|---|---|---|
| Quick | 짧은 질의, 낮은 지연, multitasking | 작은 Dense/quant, 짧은 context, 넉넉한 OS reserve |
| Balanced | 일반 대화·코딩 | fit score 1위의 검증 variant |
| Deep | 복잡한 reasoning, 품질 우선 | 큰 Dense/MoE, 낮은 동시성, 강화된 memory/thermal guard |

### Agent Trust Levels

| Level | 허용 범위 | 필수 검증 |
|---|---|---|
| Assist | 답변·초안·단발 도구 제안 | 사용자 확인 |
| Supervised | 작은 작업 여러 단계 실행 | 매 단계 또는 위험 작업 승인, build/test |
| Autonomous Candidate | 제한된 workspace에서 반복 실행·수정 | capability benchmark, sandbox, budget, rollback |

### Capability Router

- 모델별로 `planning`, `implementation`, `review`, `tool-use`, `Korean`, `RAG` 점수를 별도로 유지한다.
- 프로젝트 전체에 한 모델을 고정하는 방식과 단계별 최적 모델을 사용하는 방식을 모두 지원한다.
- 다른 모델로 전달할 때는 전체 대화를 넘기지 않고 승인된 plan, relevant files, test result만 전달한다.
- 자동 라우팅은 동일 장치의 verified benchmark가 있을 때만 허용하고 그 외에는 사용자 선택을 요청한다.

## 5. 반영 우선순위

### MVP에 포함

- Model Lab의 최소 benchmark
- Project Arena의 requirement coverage 및 manual-intervention 측정
- runtime memory-pressure guard와 Quick/Balanced/Deep profile
- TTFT/decode/peak memory 표시
- KV-cache/context aware 추천
- runtime Simple/Advanced preset
- 첫 실행 실패 시 smaller-model fallback
- task complexity 및 agent trust badge

### Phase 2–3

- headless Trusted Node와 secure pairing
- 모바일/노트북 원격 model routing
- code sandbox execution benchmark
- Portable Workspace PoC

### 후속 검토

- 프로젝트별 opt-in cloud fallback
- 실제 전력·온도·소음 측정 지원
- planner/implementer/reviewer 단계별 model routing
- Ornith 1.5 fast-track candidate의 반복 안정성 및 모바일 artifact 검증
- 사용자 benchmark의 익명 공유; 개인정보 및 재현성 정책이 먼저 필요

## 6. 제외하거나 과장하지 않을 내용

- 단일 영상의 모델 순위, GPU 구매 추천, tokens/s를 일반화하지 않는다.
- `평생 무료`, `완전한 데이터 무유출`, `cloud 모델 대체`를 보장하지 않는다.
- USB 실행이 host에 흔적을 전혀 남기지 않는다고 주장하지 않는다.
- unofficial/community artifact를 공식 모델처럼 표시하지 않는다.
- 안전장치 해제나 `uncensored` prompt를 기본 제품 기능으로 제공하지 않는다.
