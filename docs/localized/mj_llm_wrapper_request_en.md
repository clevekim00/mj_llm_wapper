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

Other languages: [한국어](mj_llm_wrapper_request_ko.md) · [English](mj_llm_wrapper_request_en.md) · [日本語](mj_llm_wrapper_request_jp.md) · [Esperanto](mj_llm_wrapper_request_es.md)
