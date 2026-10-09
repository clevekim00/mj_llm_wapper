# MJ Local LLM Hub Quick Start

> English reader edition · [Canonical source](../../README.md)

A Rust preview using embedded mj-llm/LiteRT-LM for macOS CPU chat and text embeddings without Ollama.

## Key points

- The default runtime is mj-llm, verified with Rust 1.95 and a pinned LiteRT SDK.
- Follow docs/mj-llm-runtime.md in the canonical source for SDK setup and launch.
- The web UI opens at http://127.0.0.1:3210.
- Chat uses Qwen3 0.6B; embeddings use EmbeddingGemma 2.
- Generation allows 1024 input bytes total and 32 output tokens, delivered after completion.
- Select Ollama explicitly with MJ_HUB_RUNTIME=ollama. Native mobile and token streaming are not implemented.

## Shared principles

Defaults are local-first execution, safe memory headroom, verified artifacts, runtime-neutral contracts, explicit approval, and evidence records.

---

[Read on the page](../readme.html#lang=en)

Other languages: [한국어](../readme.html#lang=ko) · [English](../readme.html#lang=en) · [日本語](../readme.html#lang=jp) · [Esperanto](../readme.html#lang=es)
