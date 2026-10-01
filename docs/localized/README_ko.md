# MJ Local LLM Hub 빠른 시작

> 한국어 독자판 · [상세 원문](../../README.md)

장치에 맞는 로컬 모델을 추천하고, Ollama로 설치한 뒤 웹 또는 CLI에서 대화하는 Rust MVP입니다.

## 핵심

- Rust 1.85+와 실행 중인 Ollama가 필요합니다.
- `cargo run -- recommend`로 추천을 보고 `cargo run -- auto-install`로 안전하게 설치합니다.
- 웹 UI는 기본적으로 `http://127.0.0.1:3210`에서 열립니다.
- Qwen, Gemma, DeepSeek, Phi, Mistral, OpenAI gpt-oss를 카탈로그로 관리합니다.
- 현재 모바일 네이티브 추론, RAG, MCP는 후속 단계입니다.

## 공통 원칙

로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.

---

다른 언어: [한국어](README_ko.md) · [English](README_en.md) · [日本語](README_jp.md) · [Esperanto](README_es.md)
