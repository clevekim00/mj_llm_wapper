# 통합 설계서

> 한국어 독자판 · [상세 원문](../../blueprint-local-llm-hub.md)

Rust 제어 계층, 런타임 어댑터, 모델 카탈로그, 메모리 입장 제어, 대화·RAG·MCP 계약을 정의한 구현 기준 문서입니다.

## 핵심

- API와 UI는 특정 런타임을 직접 알지 않고 `ModelRuntime` 계약만 사용합니다.
- Ollama는 MVP, mistral.rs는 PC/서버 주력 후보, LiteRT-LM은 모바일 경로입니다.
- 추천은 다운로드 크기가 아니라 peak RAM, KV cache, OS 여유를 기준으로 합니다.
- OpenAI 호환 API와 자체 관리 API를 분리합니다.
- MCP는 최소 권한, 승인, 비밀값 격리, 감사 로그를 기본으로 합니다.

## 공통 원칙

로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.

---

다른 언어: [한국어](blueprint-local-llm-hub_ko.md) · [English](blueprint-local-llm-hub_en.md) · [日本語](blueprint-local-llm-hub_jp.md) · [Esperanto](blueprint-local-llm-hub_es.md)
