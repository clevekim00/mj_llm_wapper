# Integrita Arkitekturo

> Esperanta legant-eldono · [Detala fonto](../../blueprint-local-llm-hub.md)

Projekto por komuna Rust-kerno, enkonstruitaj rultempoj, plurmodalaj datumoj/indeksoj/API, rimed-reakiro kaj migrado.

## Ĉefaj punktoj

- Ĉapitro 5 difinas la celan arkitekturon, datumkontraktojn, API kaj dosieran migradon.
- Poŝtelefonoj vokas la kernon per denaskaj pontoj; komputila HTTP estas laŭvola.
- Validigu LiteRT-LM kiel ĉefan enkonstruitan motoron kaj forigu la produktan dependecon de Ollama.
- Ne miksu vektorojn kun malsamaj modeloj, kvantigoj, dimensioj aŭ antaŭtraktado.
- Eldonaj kontroloj kovras senretan uzon sur realaj aparatoj, nuligon/reakiron kaj plurmodalan serĉkvaliton.

## Komunaj principoj

La defaŭltoj estas loka-unua funkciado, sekura memorrezervo, validigitaj artefaktoj, rultempo-neŭtralaj kontraktoj, eksplicita aprobo kaj pruvaj registroj.

---

[Legi en la paĝo](../reader/blueprint-local-llm-hub.html#lang=es)

Aliaj lingvoj: [한국어](../reader/blueprint-local-llm-hub.html#lang=ko) · [English](../reader/blueprint-local-llm-hub.html#lang=en) · [日本語](../reader/blueprint-local-llm-hub.html#lang=jp) · [Esperanto](../reader/blueprint-local-llm-hub.html#lang=es)
