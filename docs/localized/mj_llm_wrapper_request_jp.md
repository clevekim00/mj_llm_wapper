# Narmer連携要件

> 日本語読者版 · [詳細な原文](../../mj_llm_wrapper_request.md)

mj-narmerがローカルLLMゲートウェイに求めるOpenAI互換、構造化出力、キャンセル、タイムアウト、セキュリティ、来歴の契約です。

## 要点

- `/v1/chat/completions`のストリーミング／非ストリーミングを提供します。
- JSON object/schemaを検証し、失敗を安定したエラーコードに変換します。
- タイムアウトとクライアントキャンセルをランタイムまで伝播します。
- provider、実モデル、遅延などの来歴を記録します。
- GatewayとNarmerのドメイン責任を分離します。

## 共通原則

ローカル優先、安全なメモリ余裕、検証済みアーティファクト、ランタイム中立契約、明示的承認、根拠記録を基本とします。

---

[タブで読む](../reader/mj_llm_wrapper_request.html#lang=jp)

他の言語: [한국어](../reader/mj_llm_wrapper_request.html#lang=ko) · [English](../reader/mj_llm_wrapper_request.html#lang=en) · [日本語](../reader/mj_llm_wrapper_request.html#lang=jp) · [Esperanto](../reader/mj_llm_wrapper_request.html#lang=es)
