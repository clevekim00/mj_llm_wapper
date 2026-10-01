# Narmer 연동 요청

> 한국어 독자판 · [상세 원문](../../mj_llm_wrapper_request.md)

mj-narmer가 로컬 LLM 게이트웨이에 요구하는 OpenAI 호환, 구조화 출력, 취소·시간제한, 보안과 provenance 계약입니다.

## 핵심

- `/v1/chat/completions`의 스트리밍·비스트리밍을 제공합니다.
- JSON object/schema를 검증하고 실패를 안정적인 오류 코드로 변환합니다.
- timeout과 client cancellation을 런타임까지 전달합니다.
- 응답에 provider, 실제 모델, 지연시간 등 provenance를 기록합니다.
- Gateway와 Narmer의 도메인 책임을 분리합니다.

## 공통 원칙

로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.

---

[페이지에서 읽기](../reader/mj_llm_wrapper_request.html#lang=ko)

다른 언어: [한국어](../reader/mj_llm_wrapper_request.html#lang=ko) · [English](../reader/mj_llm_wrapper_request.html#lang=en) · [日本語](../reader/mj_llm_wrapper_request.html#lang=jp) · [Esperanto](../reader/mj_llm_wrapper_request.html#lang=es)
