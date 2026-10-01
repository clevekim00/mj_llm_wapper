# Integrita Arkitekturo

> Esperanta legant-eldono · [Detala fonto](../../blueprint-local-llm-hub.md)

Realiga skizo por la Rust-regtavolo, rultempaj adaptiloj, modela katalogo, memor-akcepto kaj kontraktoj por babilo/RAG/MCP.

## Ĉefaj punktoj

- API kaj UI dependas nur de la kontrakto `ModelRuntime`, ne de specifa motoro.
- Ollama servas la MVP-on; mistral.rs estas ĉefa kandidato por komputilo/servilo; LiteRT-LM estas la poŝtelefona vojo.
- Akcepto uzas pintan RAM, KV-kaŝmemoron kaj OS-rezervon, ne nur dosiergrandon.
- Apartigu OpenAI-kongruan inferencon de la denaska administra API.
- MCP defaŭlte uzas minimumajn rajtojn, aprobon, sekret-izoladon kaj protokolojn.

## Komunaj principoj

La defaŭltoj estas loka-unua funkciado, sekura memorrezervo, validigitaj artefaktoj, rultempo-neŭtralaj kontraktoj, eksplicita aprobo kaj pruvaj registroj.

---

[Legi en la paĝo](../reader/blueprint-local-llm-hub.html#lang=es)

Aliaj lingvoj: [한국어](../reader/blueprint-local-llm-hub.html#lang=ko) · [English](../reader/blueprint-local-llm-hub.html#lang=en) · [日本語](../reader/blueprint-local-llm-hub.html#lang=jp) · [Esperanto](../reader/blueprint-local-llm-hub.html#lang=es)
