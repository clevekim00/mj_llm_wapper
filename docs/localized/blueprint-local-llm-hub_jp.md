# 統合設計書

> 日本語読者版 · [詳細な原文](../../blueprint-local-llm-hub.md)

Rust制御層、ランタイムアダプター、モデルカタログ、メモリ受け入れ、会話・RAG・MCP契約を定義する実装基準です。

## 要点

- APIとUIは特定エンジンではなく`ModelRuntime`契約だけに依存します。
- OllamaはMVP、mistral.rsはPC／サーバー候補、LiteRT-LMはモバイル経路です。
- 推薦はファイル容量だけでなくpeak RAM、KV cache、OS余裕を使います。
- OpenAI互換推論APIと独自管理APIを分離します。
- MCPは最小権限、承認、秘密情報の隔離、監査ログを標準にします。

## 共通原則

ローカル優先、安全なメモリ余裕、検証済みアーティファクト、ランタイム中立契約、明示的承認、根拠記録を基本とします。

---

[ページで読む](../reader/blueprint-local-llm-hub.html#lang=jp)

他の言語: [한국어](../reader/blueprint-local-llm-hub.html#lang=ko) · [English](../reader/blueprint-local-llm-hub.html#lang=en) · [日本語](../reader/blueprint-local-llm-hub.html#lang=jp) · [Esperanto](../reader/blueprint-local-llm-hub.html#lang=es)
