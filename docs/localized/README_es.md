# Rapida Komenco de MJ Local LLM Hub

> Esperanta legant-eldono · [Detala fonto](../../README.md)

Rust-a antaŭvido kun enkonstruita mj-llm/LiteRT-LM por macOS-CPU-babilado kaj tekstaj enkorpigoj sen Ollama.

## Ĉefaj punktoj

- La defaŭlta rultempo estas mj-llm, kontrolita per Rust 1.95 kaj fiksita LiteRT SDK.
- Sekvu docs/mj-llm-runtime.md en la detala fonto por SDK-agordo kaj lanĉo.
- La reta fasado aperas ĉe http://127.0.0.1:3210.
- Babilado uzas Qwen3 0.6B; enkorpigoj uzas EmbeddingGemma 2.
- Generado permesas 1024 enigajn bajtojn kaj 32 eligajn tokenojn; la respondo aperas post fino.
- Elektu Ollama per MJ_HUB_RUNTIME=ollama. Denaska poŝtelefona uzo kaj tokena fluado ankoraŭ ne estas realigitaj.

## Komunaj principoj

La defaŭltoj estas loka-unua funkciado, sekura memorrezervo, validigitaj artefaktoj, rultempo-neŭtralaj kontraktoj, eksplicita aprobo kaj pruvaj registroj.

---

[Legi en la paĝo](../readme.html#lang=es)

Aliaj lingvoj: [한국어](../readme.html#lang=ko) · [English](../readme.html#lang=en) · [日本語](../readme.html#lang=jp) · [Esperanto](../readme.html#lang=es)
