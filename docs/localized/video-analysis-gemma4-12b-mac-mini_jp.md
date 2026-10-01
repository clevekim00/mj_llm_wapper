# Gemma 4 12B動画分析

> 日本語読者版 · [詳細な原文](../../video-analysis-gemma4-12b-mac-mini.md)

16GB M4 Mac miniでGemma 4 12BをLM Studio実行した動画のメモリ、速度、マルチモーダル、API実験を検証します。

## 要点

- モデルファイル容量と実際のpeak RAMは異なります。
- 公式最大コンテキスト、ランタイム上限、端末安全値を分けます。
- OCR対応は業務精度を保証しません。
- LM Studioは任意のデスクトップアダプター候補で、モバイル解ではありません。
- 導入後にTTFT、prefill、decode速度、メモリを実測します。

## 共通原則

ローカル優先、安全なメモリ余裕、検証済みアーティファクト、ランタイム中立契約、明示的承認、根拠記録を基本とします。

---

他の言語: [한국어](video-analysis-gemma4-12b-mac-mini_ko.md) · [English](video-analysis-gemma4-12b-mac-mini_en.md) · [日本語](video-analysis-gemma4-12b-mac-mini_jp.md) · [Esperanto](video-analysis-gemma4-12b-mac-mini_es.md)
