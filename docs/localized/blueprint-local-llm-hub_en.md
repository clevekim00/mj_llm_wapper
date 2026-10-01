# Integrated Architecture

> English reader edition · [Canonical source](../../blueprint-local-llm-hub.md)

Implementation blueprint for the Rust control plane, runtime adapters, model catalog, memory admission, and chat/RAG/MCP contracts.

## Key points

- API and UI depend only on the `ModelRuntime` contract, not a specific engine.
- Ollama serves the MVP; mistral.rs is a primary PC/server candidate; LiteRT-LM is the mobile path.
- Admission uses peak RAM, KV cache, and OS reserve—not download size alone.
- Separate OpenAI-compatible inference from the native management API.
- MCP defaults to least privilege, approval, secret isolation, and audit logs.

## Shared principles

Defaults are local-first execution, safe memory headroom, verified artifacts, runtime-neutral contracts, explicit approval, and evidence records.

---

[Read on the page](../reader/blueprint-local-llm-hub.html#lang=en)

Other languages: [한국어](../reader/blueprint-local-llm-hub.html#lang=ko) · [English](../reader/blueprint-local-llm-hub.html#lang=en) · [日本語](../reader/blueprint-local-llm-hub.html#lang=jp) · [Esperanto](../reader/blueprint-local-llm-hub.html#lang=es)
