# MJ Local LLM Hub クイックスタート

> 日本語読者版 · [詳細な原文](../../README.md)

mj-llm内蔵LiteRT-LMでOllamaなしにmacOS CPU会話とテキスト埋め込みを実行するRustプレビューです。

## 要点

- 既定ランタイムはmj-llmです。Rust 1.95と固定LiteRT SDKで検証しました。
- SDKの準備と起動方法は詳細原文のdocs/mj-llm-runtime.mdを参照してください。
- Web UIはhttp://127.0.0.1:3210で開きます。
- 会話はQwen3 0.6B、埋め込みはEmbeddingGemma 2を使います。
- 生成入力は合計1024 bytes、出力は最大32 tokensで、完了後に表示します。
- OllamaはMJ_HUB_RUNTIME=ollamaで選択します。モバイルnativeとトークン配信は未実装です。

## 共通原則

ローカル優先、安全なメモリ余裕、検証済みアーティファクト、ランタイム中立契約、明示的承認、根拠記録を基本とします。

---

[ページで読む](../readme.html#lang=jp)

他の言語: [한국어](../readme.html#lang=ko) · [English](../readme.html#lang=en) · [日本語](../readme.html#lang=jp) · [Esperanto](../readme.html#lang=es)
