# MJ Local LLM Hub Quick Start

> English reader edition · [Canonical source](../../README.md)

A Rust MVP that recommends a local model for the device, installs it through Ollama, and chats through the web UI or CLI.

## Key points

- Requires Rust 1.85+ and a running Ollama service.
- Use `cargo run -- recommend`, then `cargo run -- auto-install` for a guarded install.
- The web UI opens at `http://127.0.0.1:3210` by default.
- The catalog covers Qwen, Gemma, DeepSeek, Phi, Mistral, and OpenAI gpt-oss.
- Native mobile inference, RAG, and MCP are later phases.

## Shared principles

Defaults are local-first execution, safe memory headroom, verified artifacts, runtime-neutral contracts, explicit approval, and evidence records.

---

[Read with tabs](../reader/README.html#lang=en)

Other languages: [한국어](../reader/README.html#lang=ko) · [English](../reader/README.html#lang=en) · [日本語](../reader/README.html#lang=jp) · [Esperanto](../reader/README.html#lang=es)
