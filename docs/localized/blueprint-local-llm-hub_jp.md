# 統合設計書

> 日本語読者版 · [詳細な原文](../../blueprint-local-llm-hub.md)

共通Rustコア、内蔵ランタイム、マルチモーダル資料・索引・API、資源管理・復旧と実装順序を定義します。

## 要点

- 第5章に目標構成、データ契約、API、ファイル別移行作業をまとめています。
- モバイルはnative bridgeでコアを呼び、PCのHTTP APIは任意にします。
- LiteRT-LMを主エンジン候補として検証し、Ollama依存を外します。
- モデル・量子化・次元・前処理が異なるベクトルを混ぜません。
- 実機オフライン、取消・復旧、マルチモーダル検索品質を公開条件とします。

## 共通原則

ローカル優先、安全なメモリ余裕、検証済みアーティファクト、ランタイム中立契約、明示的承認、根拠記録を基本とします。

---

[ページで読む](../reader/blueprint-local-llm-hub.html#lang=jp)

他の言語: [한국어](../reader/blueprint-local-llm-hub.html#lang=ko) · [English](../reader/blueprint-local-llm-hub.html#lang=en) · [日本語](../reader/blueprint-local-llm-hub.html#lang=jp) · [Esperanto](../reader/blueprint-local-llm-hub.html#lang=es)
