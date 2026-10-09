# 製品企画書

> 日本語読者版 · [詳細な原文](../../product-plan-local-llm-hub.md)

OllamaやPythonの別途インストールなしでPC・Android・iOS上で動作するマルチモーダルAIの目標設計と開発段階です。

## 要点

- 初回リリースは文書・画像検索と根拠付き対話を基本想定とします。
- 検索用EmbeddingGemma 2と回答生成モデルを分離します。
- PCとモバイルで共通Rustコアと内蔵LiteRT-LMを検証します。
- 技術検証、PC、モバイル、音声・動画、MCPの順に進めます。
- 現在のOllama/Python開発版と目標製品の完成状態を区別します。

## 共通原則

ローカル優先、安全なメモリ余裕、検証済みアーティファクト、ランタイム中立契約、明示的承認、根拠記録を基本とします。

---

[ページで読む](../reader/product-plan-local-llm-hub.html#lang=jp)

他の言語: [한국어](../reader/product-plan-local-llm-hub.html#lang=ko) · [English](../reader/product-plan-local-llm-hub.html#lang=en) · [日本語](../reader/product-plan-local-llm-hub.html#lang=jp) · [Esperanto](../reader/product-plan-local-llm-hub.html#lang=es)
