# Narmer Integration Request

> English reader edition · [Canonical source](../../mj_llm_wrapper_request.md)

The OpenAI compatibility, structured output, cancellation, timeout, security, and provenance contract required by mj-narmer.

## Key points

- Provide streaming and non-streaming `/v1/chat/completions`.
- Validate JSON object/schema output and normalize failures into stable error codes.
- Propagate timeouts and client cancellation to the runtime.
- Record provider, effective model, latency, and other provenance.
- Keep gateway responsibilities separate from Narmer domain logic.

## Shared principles

Defaults are local-first execution, safe memory headroom, verified artifacts, runtime-neutral contracts, explicit approval, and evidence records.

---

[Read with tabs](../reader/mj_llm_wrapper_request.html#lang=en)

Other languages: [한국어](../reader/mj_llm_wrapper_request.html#lang=ko) · [English](../reader/mj_llm_wrapper_request.html#lang=en) · [日本語](../reader/mj_llm_wrapper_request.html#lang=jp) · [Esperanto](../reader/mj_llm_wrapper_request.html#lang=es)
