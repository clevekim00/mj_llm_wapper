<p align="center"><img src="docs/assets/mj-local-llm-hub-logo.png" width="144" height="144" alt="MJ Local LLM Hub 아이콘"></p>
<h1 align="center">MJ Local LLM Hub</h1>
<p align="center"><strong>내 장치에 맞는 오픈 웨이트 LLM을 찾고, 설치하고, 바로 대화하세요.</strong></p>
<p align="center">로컬 AI를 PC · 서버 · 모바일에서 더 쉽게.</p>
<p align="center">Rust · mj-llm / LiteRT-LM · 한국어 / English / 日本語 / Esperanto · MIT</p>

<p align="center"><a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=ko">한국어 탭으로 읽기</a> · <a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=en">Read in English</a> · <a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=jp">日本語で読む</a> · <a href="https://clevekim00.github.io/mj_llm_wapper/readme.html#lang=es">Legi Esperante</a></p>

문서 사이트에서는 같은 페이지의 네 언어 탭으로 내용을 바꿔 읽을 수 있습니다. GitHub README에서는 이동 링크로 제공합니다. [전체 문서](https://clevekim00.github.io/mj_llm_wapper/) · [그림으로 보는 쉬운 사용자 가이드](https://clevekim00.github.io/mj_llm_wapper/user-guide.html#lang=ko)

<details><summary>Markdown 원본 파일</summary>

[한국어](docs/localized/README_ko.md) · [English](docs/localized/README_en.md) · [日本語](docs/localized/README_jp.md) · [Esperanto](docs/localized/README_es.md)

</details>

<!-- document-body -->

## 프로젝트 개요

MJ Local LLM Hub는 현재 장치의 CPU·메모리·운영체제를 확인하고, 실행 가능한 로컬 LLM과 안전한 설정을 추천하는 Rust 기반 도구입니다. 모델 설치부터 테스트 대화까지 한 흐름으로 연결하고, 특정 엔진에 종속되지 않도록 런타임 어댑터 구조를 사용합니다.

기본 실행 엔진을 [mj-llm](https://github.com/clevekim00/mj-llm)의 내장 LiteRT-LM 어댑터로 전환했습니다. Ollama/Python 서버 없이 **macOS CPU 대화와 텍스트 임베딩**을 실행합니다. 기존 Ollama 어댑터는 `MJ_HUB_RUNTIME=ollama`로 선택할 수 있습니다.

현재는 짧은 대화용 미리보기입니다. 생성 입력은 합계 UTF-8 1024 bytes, context 512 tokens, 출력 최대 32 tokens이며 답변을 완성한 뒤 표시합니다. 토큰별 스트리밍·도구 호출·JSON 강제 출력·모바일 native 실행은 아직 지원하지 않습니다.

## 설치와 실행

Rust 1.95에서 검증하며 macOS 빌드 도구와 고정 LiteRT SDK가 필요합니다. **[SDK 준비·모델 설치·실행 안내](docs/mj-llm-runtime.md)**를 따라 설정하세요.

```bash
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" recommend
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" install mj-llm/qwen3-0.6b
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" install google/embeddinggemma-2
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" serve
```

웹 UI는 `http://127.0.0.1:3210`에서 열립니다. CLI와 웹 모델 센터는 선택한 런타임의 모델만 표시하며 다운로드한 파일은 크기·SHA-256 검증 후 설치합니다. 기존 N0 모델 파일을 직접 지정할 수도 있습니다.

- `/v1/chat/completions`: `mj-llm/qwen3-0.6b`, `stream=false`, 역할을 보존한 대화 기록
- `/v1/embeddings`: `google/embeddinggemma-2`, text 입력, 768차원, embedding-space 식별자
- `/api/status`: 실제 선택한 엔진과 설치 상태
- 인증된 loopback 전용 서버와 기존 대화 저장 기능 유지

SDK 없는 `cargo run`은 서버와 설정 상태를 확인하는 용도로 사용할 수 있으며, 추론은 미지원 오류를 반환합니다. Ollama를 자동으로 실행하거나 fallback하지 않습니다. 기존 방식은 다음처럼 명시합니다.

```bash
MJ_HUB_RUNTIME=ollama cargo run -- serve
```

Python 비교 embedding 서비스는 `MJ_EMBEDDING_URL`을 지정할 때만 사용합니다. [기존 reference 안내](docs/embeddinggemma-2.md).

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
