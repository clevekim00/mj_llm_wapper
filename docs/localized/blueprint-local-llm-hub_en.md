# Integrated Architecture

> English reader edition · [Canonical source](../../blueprint-local-llm-hub.md)

Design for the shared Rust core, embedded runtimes, multimodal assets/indexes/APIs, resource recovery and migration steps.

## Key points

- Chapter 5 defines the target architecture, data contracts, APIs and file-level migration.
- Mobile calls the core through native bridges; desktop HTTP is optional.
- Validate LiteRT-LM as the primary embedded engine and remove Ollama from product dependencies.
- Keep vectors with different model, quantization, dimension or preprocessing profiles separate.
- Release gates cover physical-device offline use, cancellation/recovery and multimodal retrieval quality.

## Shared principles

Defaults are local-first execution, safe memory headroom, verified artifacts, runtime-neutral contracts, explicit approval, and evidence records.

---

[Read on the page](../reader/blueprint-local-llm-hub.html#lang=en)

Other languages: [한국어](../reader/blueprint-local-llm-hub.html#lang=ko) · [English](../reader/blueprint-local-llm-hub.html#lang=en) · [日本語](../reader/blueprint-local-llm-hub.html#lang=jp) · [Esperanto](../reader/blueprint-local-llm-hub.html#lang=es)
