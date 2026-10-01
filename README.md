<p align="center"><img src="docs/assets/mj-local-llm-hub-logo.png" width="144" height="144" alt="MJ Local LLM Hub 아이콘"></p>
<h1 align="center">MJ Local LLM Hub</h1>
<p align="center"><strong>내 장치에 맞는 오픈 웨이트 LLM을 찾고, 설치하고, 바로 대화하세요.</strong></p>
<p align="center">로컬 AI를 PC · 서버 · 모바일에서 더 쉽게.</p>
<p align="center">Rust · Ollama adapter · 한국어 / English / 日本語 / Esperanto · MIT</p>

<p align="center"><a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=ko">한국어 탭으로 읽기</a> · <a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=en">Read in English</a> · <a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=jp">日本語で読む</a> · <a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=es">Legi Esperante</a></p>

문서 사이트에서는 같은 페이지의 네 언어 탭으로 내용을 바꿔 읽을 수 있습니다. GitHub README에서는 이동 링크로 제공합니다. [전체 문서](https://clevekim00.github.io/mj_llm_wapper/) · [그림으로 보는 쉬운 사용자 가이드](https://clevekim00.github.io/mj_llm_wapper/user-guide.html#lang=ko)

<details><summary>Markdown 원본 파일</summary>

[한국어](docs/localized/README_ko.md) · [English](docs/localized/README_en.md) · [日本語](docs/localized/README_jp.md) · [Esperanto](docs/localized/README_es.md)

</details>

<!-- document-body -->

## 프로젝트 개요

MJ Local LLM Hub는 현재 장치의 CPU·메모리·운영체제를 확인하고, 실행 가능한 로컬 LLM과 안전한 설정을 추천하는 Rust 기반 도구입니다. 모델 설치부터 테스트 대화까지 한 흐름으로 연결하고, 특정 엔진에 종속되지 않도록 런타임 어댑터 구조를 사용합니다.

현재 MVP는 로컬 [Ollama](https://ollama.com/)를 통해 Qwen, Gemma, DeepSeek, Phi, Mistral과 OpenAI gpt-oss 모델을 설치하고 실행합니다. 모바일 네이티브 추론, RAG와 MCP는 후속 단계입니다.

### 핵심 특징

- **장치 진단** — OS, architecture, CPU, RAM과 런타임 연결 상태 확인
- **안전한 모델 추천** — 모델 크기뿐 아니라 실행 메모리와 시스템 여유를 함께 계산
- **간단한 설치** — 추천 결과에서 모델을 선택하거나 최적 모델을 자동 설치
- **바로 테스트하는 대화** — 설치한 모델로 멀티턴 스트리밍 대화
- **런타임 어댑터** — Ollama를 시작으로 mistral.rs, LiteRT-LM, llama.cpp 확장 가능
- **OpenAI 호환 API** — `/v1/models`, `/v1/chat/completions` 제공
- **로컬 우선 보안** — loopback-only 서버, 로컬 토큰, 승인 기반 확장 설계

## 설치와 실행

### 사전 요구사항

- Rust 1.85 이상
- 로컬 [Ollama](https://ollama.com/)
- 모델을 저장할 디스크 공간과 다운로드 네트워크

### 웹 UI 실행

```bash
ollama serve
cargo run
```

브라우저에서 `http://127.0.0.1:3210`을 엽니다.

### 명령줄 추천과 설치

```bash
# 현재 시스템에 맞는 모델 추천
cargo run -- recommend

# JSON 형식 추천 결과
cargo run -- recommend --json

# 추천된 미설치 모델 자동 선택·설치
cargo run -- auto-install

# 특정 모델 설치
cargo run -- install qwen3-0.6b-q4
```

설치 전에 모델 크기, 예상 peak RAM과 적합도를 보여 주고 확인을 받습니다. 자동화 환경에서는 `--yes`를 사용할 수 있으며, 안전 기준을 벗어난 모델은 `--force` 없이는 설치하지 않습니다.

## 사용자 문서

아래 링크는 HTML 소스가 아니라 GitHub Pages 문서 화면을 엽니다. 각 페이지 위에서 한국어, English, 日本語, Esperanto로 전환할 수 있습니다.

| 문서 | 바로가기 |
| --- | --- |
| 쉬운 사용자 가이드 | [그림으로 설치·사용법 보기](https://clevekim00.github.io/mj_llm_wapper/user-guide.html#lang=ko) |
| 전체 문서 | [문서 허브 열기](https://clevekim00.github.io/mj_llm_wapper/) |
| 제품 기획서 | [기획서 읽기](https://clevekim00.github.io/mj_llm_wapper/reader/product-plan-local-llm-hub.html#lang=ko) |
| 통합 설계서 | [설계서 읽기](https://clevekim00.github.io/mj_llm_wapper/reader/blueprint-local-llm-hub.html#lang=ko) |

## 검증

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## 라이선스

MIT
