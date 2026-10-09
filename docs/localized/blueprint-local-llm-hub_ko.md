# 통합 설계서

> 한국어 독자판 · [상세 원문](../../blueprint-local-llm-hub.md)

PC·모바일 공통 Rust 코어, 내장 런타임, 멀티모달 자료·인덱스·API, 메모리·복구 정책과 구현 순서를 정의합니다.

## 핵심

- 설계서 5장에 목표 아키텍처, 데이터 계약, API와 파일별 전환 작업을 정리했습니다.
- 모바일은 native bridge로 코어를 호출하고 PC HTTP API는 선택형으로 둡니다.
- LiteRT-LM을 PC·모바일 주력 후보로 검증하며 Ollama는 새 제품 의존성에서 제외합니다.
- 모델·양자화·차원·전처리가 다른 벡터 공간은 혼합하지 않습니다.
- 실기기 오프라인, 취소·복원, 멀티모달 검색 품질을 출시 gate로 삼습니다.

## 공통 원칙

로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.

---

[페이지에서 읽기](../reader/blueprint-local-llm-hub.html#lang=ko)

다른 언어: [한국어](../reader/blueprint-local-llm-hub.html#lang=ko) · [English](../reader/blueprint-local-llm-hub.html#lang=en) · [日本語](../reader/blueprint-local-llm-hub.html#lang=jp) · [Esperanto](../reader/blueprint-local-llm-hub.html#lang=es)
