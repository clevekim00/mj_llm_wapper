# Gemma 4 12B 영상 분석

> 한국어 독자판 · [상세 원문](../../video-analysis-gemma4-12b-mac-mini.md)

16GB M4 Mac mini에서 Gemma 4 12B를 LM Studio로 실행한 영상의 메모리·속도·멀티모달·API 실험을 검증합니다.

## 핵심

- 모델 파일 크기와 실제 peak RAM은 다릅니다.
- 공식 최대 컨텍스트, 런타임 최대값, 장치 안전값을 분리해야 합니다.
- OCR 가능 여부는 업무 정확도를 보장하지 않습니다.
- LM Studio는 선택적 데스크톱 어댑터 후보지만 모바일 해법은 아닙니다.
- TTFT, prefill, decode 속도와 메모리를 설치 후 실측해야 합니다.

## 공통 원칙

로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.

---

다른 언어: [한국어](video-analysis-gemma4-12b-mac-mini_ko.md) · [English](video-analysis-gemma4-12b-mac-mini_en.md) · [日本語](video-analysis-gemma4-12b-mac-mini_jp.md) · [Esperanto](video-analysis-gemma4-12b-mac-mini_es.md)
