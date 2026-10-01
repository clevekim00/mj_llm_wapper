<div align="center">
  <img src="docs/assets/mj-local-llm-hub-logo.png" width="132" alt="MJ Local LLM Hub logo">
  <h1>MJ Local LLM Hub</h1>
  <p><strong>Find, install, and chat with an open-weight LLM that fits your device.</strong></p>
  <p>PC · Server · Mobile · Local-first</p>
</div>

---

<p align="center"><strong>언어를 눌러 펼쳐 보세요 · Select a language · 言語を選択 · Elektu lingvon</strong></p>

<details open>
<summary><strong>한국어</strong></summary>

### 소개

MJ Local LLM Hub는 컴퓨터 사양을 확인하고 알맞은 오픈 웨이트 LLM을 추천하여 설치와 대화까지 이어 주는 Rust 기반 도구입니다. 현재 MVP는 Ollama를 사용하며 Qwen, Gemma, DeepSeek, Phi, Mistral과 OpenAI gpt-oss 모델을 지원합니다.

### 설치와 실행

Rust 1.85 이상과 [Ollama](https://ollama.com/)가 필요합니다.

```bash
ollama serve
cargo run
```

브라우저에서 `http://127.0.0.1:3210`을 엽니다.

### 명령줄 사용법

```bash
# 내 시스템에 맞는 모델 추천
cargo run -- recommend

# 추천 모델 자동 설치
cargo run -- auto-install
```

- [아주 쉬운 사용자 가이드](docs/user-guide.html#lang=ko)
- [상세 문서](docs/index.html#lang=ko)

</details>

<details>
<summary><strong>English</strong></summary>

### Introduction

MJ Local LLM Hub is a Rust tool that checks your computer, recommends a fitting open-weight LLM, installs it, and lets you chat with it. The MVP currently uses Ollama and supports Qwen, Gemma, DeepSeek, Phi, Mistral, and OpenAI gpt-oss models.

### Install and run

You need Rust 1.85 or newer and [Ollama](https://ollama.com/).

```bash
ollama serve
cargo run
```

Open `http://127.0.0.1:3210` in your browser.

### Command-line usage

```bash
# Recommend models for this system
cargo run -- recommend

# Install the recommended model
cargo run -- auto-install
```

- [Very easy user guide](docs/user-guide.html#lang=en)
- [Detailed documentation](docs/index.html#lang=en)

</details>

<details>
<summary><strong>日本語</strong></summary>

### 紹介

MJ Local LLM Hubは、パソコンの仕様を確認し、適切なオープンウェイトLLMを推薦して、インストールから会話まで案内するRust製ツールです。現在のMVPはOllamaを使用し、Qwen、Gemma、DeepSeek、Phi、Mistral、OpenAI gpt-ossに対応します。

### インストールと実行

Rust 1.85以上と[Ollama](https://ollama.com/)が必要です。

```bash
ollama serve
cargo run
```

ブラウザーで`http://127.0.0.1:3210`を開きます。

### コマンドラインの使い方

```bash
# この端末に合うモデルを推薦
cargo run -- recommend

# 推薦モデルを自動インストール
cargo run -- auto-install
```

- [とてもやさしいユーザーガイド](docs/user-guide.html#lang=jp)
- [詳細ドキュメント](docs/index.html#lang=jp)

</details>

<details>
<summary><strong>Esperanto</strong></summary>

### Enkonduko

MJ Local LLM Hub estas Rust-ilo kiu kontrolas vian komputilon, rekomendas taŭgan malfermitpezan LLM-on, instalas ĝin kaj ebligas babili. La nuna MVP uzas Ollama kaj subtenas Qwen, Gemma, DeepSeek, Phi, Mistral kaj OpenAI gpt-oss.

### Instali kaj lanĉi

Vi bezonas Rust 1.85 aŭ pli novan kaj [Ollama](https://ollama.com/).

```bash
ollama serve
cargo run
```

Malfermu `http://127.0.0.1:3210` en via retumilo.

### Komandlinia uzo

```bash
# Rekomendi modelojn por ĉi tiu sistemo
cargo run -- recommend

# Instali la rekomenditan modelon
cargo run -- auto-install
```

- [Tre facila uzantgvidilo](docs/user-guide.html#lang=es)
- [Detala dokumentaro](docs/index.html#lang=es)

</details>
