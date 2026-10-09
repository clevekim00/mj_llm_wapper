# MJ Local LLM Hub 빠른 시작

> 한국어 독자판 · [상세 원문](../../README.md)

mj-llm 내장 LiteRT-LM으로 Ollama 없이 macOS CPU 대화와 텍스트 임베딩을 실행하는 Rust 미리보기입니다.

## 핵심

- 기본 런타임은 mj-llm입니다. Rust 1.95와 고정 LiteRT SDK에서 검증했습니다.
- SDK 준비와 실행은 상세 원문의 docs/mj-llm-runtime.md 안내를 따르세요.
- 웹 UI는 http://127.0.0.1:3210에서 열립니다.
- 대화 모델은 Qwen3 0.6B, 임베딩 모델은 EmbeddingGemma 2입니다.
- 생성은 합계 1024-byte 입력·최대 32-token 출력·완료 후 표시 방식입니다.
- Ollama는 MJ_HUB_RUNTIME=ollama로 선택합니다. 모바일 native와 토큰 스트리밍은 아직 미지원입니다.

## 공통 원칙

로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.

---

[페이지에서 읽기](../readme.html#lang=ko)

다른 언어: [한국어](../readme.html#lang=ko) · [English](../readme.html#lang=en) · [日本語](../readme.html#lang=jp) · [Esperanto](../readme.html#lang=es)
