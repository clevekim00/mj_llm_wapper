# 제품 기획서

> 한국어 독자판 · [상세 원문](../../product-plan-local-llm-hub.md)

Ollama/Python 별도 설치 없이 PC·Android·iOS에서 실행하는 멀티모달 AI 제품의 목표 설계와 출시 단계입니다.

## 핵심

- 첫 출시는 문서·사진 검색과 근거 기반 대화를 기본 가정으로 합니다.
- 검색용 EmbeddingGemma 2와 답변 생성 모델을 분리합니다.
- PC·모바일은 공통 Rust 코어와 내장 LiteRT-LM을 우선 검증합니다.
- 기술 검증 → PC 흐름 → 모바일 출시 → 음성·영상 → MCP·생태계 순서입니다.
- 현재 Ollama/Python 개발판과 목표 제품의 완료 상태를 구분합니다.

## 공통 원칙

로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.

---

[페이지에서 읽기](../reader/product-plan-local-llm-hub.html#lang=ko)

다른 언어: [한국어](../reader/product-plan-local-llm-hub.html#lang=ko) · [English](../reader/product-plan-local-llm-hub.html#lang=en) · [日本語](../reader/product-plan-local-llm-hub.html#lang=jp) · [Esperanto](../reader/product-plan-local-llm-hub.html#lang=es)
