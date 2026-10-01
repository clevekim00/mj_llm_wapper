#!/usr/bin/env python3
"""Generate the four reader-oriented documentation editions and HTML hubs.

The Korean source documents remain the canonical detailed specifications. The
localized editions preserve decisions, constraints, commands, and source links
in a compact form that is practical to keep synchronized.
"""

from html import escape
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs" / "localized"
OUT.mkdir(parents=True, exist_ok=True)

LANGS = {
    "ko": {"label": "한국어", "html": "ko", "edition": "한국어 독자판", "source": "상세 원문"},
    "en": {"label": "English", "html": "en", "edition": "English reader edition", "source": "Canonical source"},
    "jp": {"label": "日本語", "html": "ja", "edition": "日本語読者版", "source": "詳細な原文"},
    "es": {"label": "Esperanto", "html": "eo", "edition": "Esperanta legant-eldono", "source": "Detala fonto"},
}

DOCS = [
    {
        "slug": "README",
        "source": "README.md",
        "title": {
            "ko": "MJ Local LLM Hub 빠른 시작",
            "en": "MJ Local LLM Hub Quick Start",
            "jp": "MJ Local LLM Hub クイックスタート",
            "es": "Rapida Komenco de MJ Local LLM Hub",
        },
        "summary": {
            "ko": "장치에 맞는 로컬 모델을 추천하고, Ollama로 설치한 뒤 웹 또는 CLI에서 대화하는 Rust MVP입니다.",
            "en": "A Rust MVP that recommends a local model for the device, installs it through Ollama, and chats through the web UI or CLI.",
            "jp": "端末に合うローカルモデルを推薦し、Ollamaで導入してWebまたはCLIから会話するRust製MVPです。",
            "es": "Rust-a MVP kiu rekomendas lokan modelon por la aparato, instalas ĝin per Ollama, kaj babilas per la reto aŭ komandlinio.",
        },
        "points": {
            "ko": ["Rust 1.85+와 실행 중인 Ollama가 필요합니다.", "`cargo run -- recommend`로 추천을 보고 `cargo run -- auto-install`로 안전하게 설치합니다.", "웹 UI는 기본적으로 `http://127.0.0.1:3210`에서 열립니다.", "Qwen, Gemma, DeepSeek, Phi, Mistral, OpenAI gpt-oss를 카탈로그로 관리합니다.", "현재 모바일 네이티브 추론, RAG, MCP는 후속 단계입니다."],
            "en": ["Requires Rust 1.85+ and a running Ollama service.", "Use `cargo run -- recommend`, then `cargo run -- auto-install` for a guarded install.", "The web UI opens at `http://127.0.0.1:3210` by default.", "The catalog covers Qwen, Gemma, DeepSeek, Phi, Mistral, and OpenAI gpt-oss.", "Native mobile inference, RAG, and MCP are later phases."],
            "jp": ["Rust 1.85以上と起動中のOllamaが必要です。", "`cargo run -- recommend`で推薦を確認し、`cargo run -- auto-install`で安全に導入します。", "Web UIは既定で`http://127.0.0.1:3210`に開きます。", "Qwen、Gemma、DeepSeek、Phi、Mistral、OpenAI gpt-ossをカタログ管理します。", "モバイル推論、RAG、MCPは後続段階です。"],
            "es": ["Necesas Rust 1.85+ kaj funkcianta servo Ollama.", "Uzu `cargo run -- recommend`, poste `cargo run -- auto-install` por gardata instalado.", "La reta fasado aperas ĉe `http://127.0.0.1:3210` defaŭlte.", "La katalogo enhavas Qwen, Gemma, DeepSeek, Phi, Mistral kaj OpenAI gpt-oss.", "Denaska poŝtelefona inferenco, RAG kaj MCP venos poste."],
        },
    },
    {
        "slug": "product-plan-local-llm-hub",
        "source": "product-plan-local-llm-hub.md",
        "title": {"ko": "제품 기획서", "en": "Product Plan", "jp": "製品企画書", "es": "Produkta Plano"},
        "summary": {
            "ko": "PC·서버·모바일에서 오픈 웨이트 모델을 쉽게 찾고 설치하며 RAG와 MCP를 연결하는 로컬 우선 제품의 범위와 로드맵입니다.",
            "en": "Scope and roadmap for a local-first product that discovers and installs open-weight models on PCs, servers, and mobile devices, with RAG and MCP integration.",
            "jp": "PC・サーバー・モバイルでオープンウェイトモデルを探して導入し、RAGとMCPを接続するローカル優先製品の範囲とロードマップです。",
            "es": "Amplekso kaj vojmapo por loka-unua produkto, kiu trovas kaj instalas malfermitpezajn modelojn en komputiloj, serviloj kaj poŝaparatoj, kun RAG kaj MCP.",
        },
        "points": {
            "ko": ["사용자의 시스템 사양을 먼저 측정하고 안전한 모델·양자화·컨텍스트를 추천합니다.", "모델 다운로드, 무결성, 라이선스, 실행 수명주기를 하나의 UI에서 관리합니다.", "데스크톱과 서버는 런타임 어댑터, 모바일은 LiteRT-LM 계열을 우선 검증합니다.", "대화, 프로젝트별 RAG, 승인 기반 MCP를 같은 워크스페이스에 둡니다.", "MVP에서 시작해 모바일, RAG/MCP, 생태계 호환 순서로 확장합니다."],
            "en": ["Measure the device first, then recommend a safe model, quantization, and context size.", "Manage downloads, integrity, licensing, and model lifecycle in one interface.", "Use runtime adapters on desktop/server and validate LiteRT-LM first on mobile.", "Keep chat, per-project RAG, and approval-based MCP in one workspace.", "Grow from the MVP through mobile, RAG/MCP, and ecosystem compatibility."],
            "jp": ["まず端末を測定し、安全なモデル・量子化・コンテキスト長を推薦します。", "ダウンロード、完全性、ライセンス、モデルのライフサイクルを一画面で管理します。", "デスクトップ／サーバーはランタイムアダプター、モバイルはLiteRT-LMを優先検証します。", "会話、プロジェクト別RAG、承認型MCPを一つのワークスペースに置きます。", "MVPからモバイル、RAG/MCP、互換性へ段階的に拡張します。"],
            "es": ["Unue mezuru la aparaton, poste rekomendu sekuran modelon, kvantigon kaj kuntekstan longon.", "Administru elŝuton, integrecon, permesilon kaj modelan vivociklon en unu fasado.", "Uzu rultempajn adaptilojn por komputiloj/serviloj kaj unue validigu LiteRT-LM por poŝaparatoj.", "Kunigu babilon, projektan RAG kaj aprob-bazitan MCP en unu laborspaco.", "Kresku de la MVP al poŝtelefono, RAG/MCP kaj ekosistema kongruo."],
        },
    },
    {
        "slug": "blueprint-local-llm-hub",
        "source": "blueprint-local-llm-hub.md",
        "title": {"ko": "통합 설계서", "en": "Integrated Architecture", "jp": "統合設計書", "es": "Integrita Arkitekturo"},
        "summary": {
            "ko": "Rust 제어 계층, 런타임 어댑터, 모델 카탈로그, 메모리 입장 제어, 대화·RAG·MCP 계약을 정의한 구현 기준 문서입니다.",
            "en": "Implementation blueprint for the Rust control plane, runtime adapters, model catalog, memory admission, and chat/RAG/MCP contracts.",
            "jp": "Rust制御層、ランタイムアダプター、モデルカタログ、メモリ受け入れ、会話・RAG・MCP契約を定義する実装基準です。",
            "es": "Realiga skizo por la Rust-regtavolo, rultempaj adaptiloj, modela katalogo, memor-akcepto kaj kontraktoj por babilo/RAG/MCP.",
        },
        "points": {
            "ko": ["API와 UI는 특정 런타임을 직접 알지 않고 `ModelRuntime` 계약만 사용합니다.", "Ollama는 MVP, mistral.rs는 PC/서버 주력 후보, LiteRT-LM은 모바일 경로입니다.", "추천은 다운로드 크기가 아니라 peak RAM, KV cache, OS 여유를 기준으로 합니다.", "OpenAI 호환 API와 자체 관리 API를 분리합니다.", "MCP는 최소 권한, 승인, 비밀값 격리, 감사 로그를 기본으로 합니다."],
            "en": ["API and UI depend only on the `ModelRuntime` contract, not a specific engine.", "Ollama serves the MVP; mistral.rs is a primary PC/server candidate; LiteRT-LM is the mobile path.", "Admission uses peak RAM, KV cache, and OS reserve—not download size alone.", "Separate OpenAI-compatible inference from the native management API.", "MCP defaults to least privilege, approval, secret isolation, and audit logs."],
            "jp": ["APIとUIは特定エンジンではなく`ModelRuntime`契約だけに依存します。", "OllamaはMVP、mistral.rsはPC／サーバー候補、LiteRT-LMはモバイル経路です。", "推薦はファイル容量だけでなくpeak RAM、KV cache、OS余裕を使います。", "OpenAI互換推論APIと独自管理APIを分離します。", "MCPは最小権限、承認、秘密情報の隔離、監査ログを標準にします。"],
            "es": ["API kaj UI dependas nur de la kontrakto `ModelRuntime`, ne de specifa motoro.", "Ollama servas la MVP-on; mistral.rs estas ĉefa kandidato por komputilo/servilo; LiteRT-LM estas la poŝtelefona vojo.", "Akcepto uzas pintan RAM, KV-kaŝmemoron kaj OS-rezervon, ne nur dosiergrandon.", "Apartigu OpenAI-kongruan inferencon de la denaska administra API.", "MCP defaŭlte uzas minimumajn rajtojn, aprobon, sekret-izoladon kaj protokolojn."],
        },
    },
    {
        "slug": "mj_llm_wrapper_request",
        "source": "mj_llm_wrapper_request.md",
        "title": {"ko": "Narmer 연동 요청", "en": "Narmer Integration Request", "jp": "Narmer連携要件", "es": "Peto pri Integrado kun Narmer"},
        "summary": {
            "ko": "mj-narmer가 로컬 LLM 게이트웨이에 요구하는 OpenAI 호환, 구조화 출력, 취소·시간제한, 보안과 provenance 계약입니다.",
            "en": "The OpenAI compatibility, structured output, cancellation, timeout, security, and provenance contract required by mj-narmer.",
            "jp": "mj-narmerがローカルLLMゲートウェイに求めるOpenAI互換、構造化出力、キャンセル、タイムアウト、セキュリティ、来歴の契約です。",
            "es": "La kontrakto pri OpenAI-kongruo, strukturita eligo, nuligo, tempolimo, sekureco kaj deveno postulata de mj-narmer.",
        },
        "points": {
            "ko": ["`/v1/chat/completions`의 스트리밍·비스트리밍을 제공합니다.", "JSON object/schema를 검증하고 실패를 안정적인 오류 코드로 변환합니다.", "timeout과 client cancellation을 런타임까지 전달합니다.", "응답에 provider, 실제 모델, 지연시간 등 provenance를 기록합니다.", "Gateway와 Narmer의 도메인 책임을 분리합니다."],
            "en": ["Provide streaming and non-streaming `/v1/chat/completions`.", "Validate JSON object/schema output and normalize failures into stable error codes.", "Propagate timeouts and client cancellation to the runtime.", "Record provider, effective model, latency, and other provenance.", "Keep gateway responsibilities separate from Narmer domain logic."],
            "jp": ["`/v1/chat/completions`のストリーミング／非ストリーミングを提供します。", "JSON object/schemaを検証し、失敗を安定したエラーコードに変換します。", "タイムアウトとクライアントキャンセルをランタイムまで伝播します。", "provider、実モデル、遅延などの来歴を記録します。", "GatewayとNarmerのドメイン責任を分離します。"],
            "es": ["Provizu fluantan kaj nefluan `/v1/chat/completions`.", "Validigu JSON-objekton/skemon kaj normigu malsukcesojn al stabilaj erarkodoj.", "Transdonu tempolimojn kaj klientan nuligon ĝis la rultempo.", "Registru provizanton, efektivan modelon, prokraston kaj alian devenon.", "Apartigu la respondecojn de Gateway disde la domajna logiko de Narmer."],
        },
    },
    {
        "slug": "opensource-analysis-mlx-serve",
        "source": "opensource-analysis-mlx-serve.md",
        "title": {"ko": "mlx-serve 분석", "en": "mlx-serve Analysis", "jp": "mlx-serve分析", "es": "Analizo de mlx-serve"},
        "summary": {
            "ko": "Mac 우선 로컬 AI 허브인 mlx-serve에서 채택할 패턴과 그대로 복제하지 않을 부분을 정리한 문서입니다.",
            "en": "An assessment of patterns worth adopting from mlx-serve, a Mac-first local AI hub, and parts that should not be copied directly.",
            "jp": "Mac優先のローカルAIハブmlx-serveから採用すべきパターンと、そのままコピーしない部分の分析です。",
            "es": "Takso de utilaj ŝablonoj el mlx-serve, Mac-unua loka AI-centro, kaj de partoj ne rekte kopiendaj.",
        },
        "points": {
            "ko": ["모델 검색→다운로드→메모리 확인→대화의 짧은 흐름을 참고합니다.", "OpenAI·Anthropic·Ollama facade는 endpoint별 호환 행렬로 관리합니다.", "LM Studio/Hugging Face cache 재사용과 다운로드 재개를 검토합니다.", "MLX 최적화는 macOS 어댑터 안에 격리합니다.", "Rust 공통 코어와 플랫폼 런타임의 책임을 분리합니다."],
            "en": ["Adopt the short search→download→memory check→chat journey.", "Track OpenAI, Anthropic, and Ollama facades with endpoint-level conformance matrices.", "Consider LM Studio/Hugging Face cache reuse and resumable downloads.", "Contain MLX optimization inside the macOS adapter.", "Separate the Rust common core from platform runtime responsibilities."],
            "jp": ["検索→ダウンロード→メモリ確認→会話の短い導線を参考にします。", "OpenAI・Anthropic・Ollama facadeはendpoint単位の互換表で管理します。", "LM Studio/Hugging Face cacheの再利用と再開可能ダウンロードを検討します。", "MLX最適化はmacOSアダプター内に隔離します。", "Rust共通コアとプラットフォームランタイムの責任を分離します。"],
            "es": ["Adoptu la mallongan vojon serĉo→elŝuto→memorkontrolo→babilo.", "Sekvu la OpenAI-, Anthropic- kaj Ollama-fasadojn per endpoint-nivelaj kongruaj matricoj.", "Konsideru reuzon de LM Studio/Hugging Face-kaŝmemoro kaj daŭrigeblajn elŝutojn.", "Izolu MLX-optimumigon ene de la macOS-adaptilo.", "Apartigu la komunan Rust-kernon disde platformaj rultempaj respondecoj."],
        },
    },
    {
        "slug": "video-research-local-llm-guides",
        "source": "video-research-local-llm-guides.md",
        "title": {"ko": "로컬 LLM 영상 연구", "en": "Local LLM Video Research", "jp": "ローカルLLM動画調査", "es": "Video-Esploro pri Lokaj LLM-oj"},
        "summary": {
            "ko": "여러 로컬 LLM 설치·운영 영상을 주제별로 분류하고 제품 기획에 반영할 근거와 한계를 정리합니다.",
            "en": "A categorized review of local LLM setup and operations videos, with product implications and evidence limitations.",
            "jp": "複数のローカルLLM導入・運用動画を分類し、製品への示唆と根拠の限界を整理します。",
            "es": "Kategoria revizio de filmetoj pri instalado kaj funkciado de lokaj LLM-oj, kun produktaj sekvoj kaj limoj de la pruvoj.",
        },
        "points": {
            "ko": ["쉬운 설치 경로와 고급 런타임 제어를 단계적으로 분리합니다.", "CPU, GPU, RAM, 양자화와 컨텍스트를 함께 설명해야 합니다.", "모델 설치 직후 실제 프롬프트로 검증하는 흐름이 필요합니다.", "RAG와 MCP는 버튼 하나가 아니라 권한·출처·오류 처리가 포함된 기능입니다.", "영상 주장은 공식 문서와 실측으로 다시 확인합니다."],
            "en": ["Separate easy setup from advanced runtime control in progressive layers.", "Explain CPU, GPU, RAM, quantization, and context together.", "Validate a model with a real prompt immediately after installation.", "RAG and MCP require permissions, sources, and failure handling—not just a button.", "Recheck video claims against official documentation and measurements."],
            "jp": ["簡単な導入と高度なランタイム制御を段階的に分けます。", "CPU、GPU、RAM、量子化、コンテキストを一緒に説明します。", "導入直後に実際のプロンプトで検証する導線が必要です。", "RAGとMCPには権限、出典、失敗処理が必要です。", "動画の主張は公式文書と実測で再確認します。"],
            "es": ["Apartigu facilan instaladon disde altnivela rultempa regado per sinsekvaj tavoloj.", "Kune klarigu CPU, GPU, RAM, kvantigon kaj kuntekston.", "Validigu modelon per vera prompto tuj post instalado.", "RAG kaj MCP bezonas permesojn, fontojn kaj erartraktadon, ne nur butonon.", "Rekontrolu filmetajn asertojn per oficialaj dokumentoj kaj mezuroj."],
        },
    },
    {
        "slug": "video-analysis-gemma4-12b-mac-mini",
        "source": "video-analysis-gemma4-12b-mac-mini.md",
        "title": {"ko": "Gemma 4 12B 영상 분석", "en": "Gemma 4 12B Video Analysis", "jp": "Gemma 4 12B動画分析", "es": "Video-Analizo de Gemma 4 12B"},
        "summary": {
            "ko": "16GB M4 Mac mini에서 Gemma 4 12B를 LM Studio로 실행한 영상의 메모리·속도·멀티모달·API 실험을 검증합니다.",
            "en": "A review of memory, speed, multimodal, and API experiments from a video running Gemma 4 12B in LM Studio on a 16GB M4 Mac mini.",
            "jp": "16GB M4 Mac miniでGemma 4 12BをLM Studio実行した動画のメモリ、速度、マルチモーダル、API実験を検証します。",
            "es": "Revizio de memor-, rapid-, plurmodala kaj API-eksperimentoj en filmeto pri Gemma 4 12B per LM Studio sur 16GB M4 Mac mini.",
        },
        "points": {
            "ko": ["모델 파일 크기와 실제 peak RAM은 다릅니다.", "공식 최대 컨텍스트, 런타임 최대값, 장치 안전값을 분리해야 합니다.", "OCR 가능 여부는 업무 정확도를 보장하지 않습니다.", "LM Studio는 선택적 데스크톱 어댑터 후보지만 모바일 해법은 아닙니다.", "TTFT, prefill, decode 속도와 메모리를 설치 후 실측해야 합니다."],
            "en": ["Model file size differs from real peak RAM.", "Separate official maximum context, runtime limit, and device-safe context.", "OCR capability does not guarantee business accuracy.", "LM Studio is an optional desktop adapter candidate, not a mobile solution.", "Measure TTFT, prefill, decode speed, and memory after installation."],
            "jp": ["モデルファイル容量と実際のpeak RAMは異なります。", "公式最大コンテキスト、ランタイム上限、端末安全値を分けます。", "OCR対応は業務精度を保証しません。", "LM Studioは任意のデスクトップアダプター候補で、モバイル解ではありません。", "導入後にTTFT、prefill、decode速度、メモリを実測します。"],
            "es": ["La modela dosiergrando diferencas de la vera pinta RAM.", "Apartigu oficialan maksimuman kuntekston, rultempan limon kaj aparato-sekuran kuntekston.", "OCR-kapablo ne garantias komercan precizecon.", "LM Studio estas laŭvola labortabla adaptilo, ne poŝtelefona solvo.", "Mezuru TTFT, prefill, decode-rapidon kaj memoron post instalado."],
        },
    },
    {
        "slug": "video-analysis-ai-knowledge-base-eli5",
        "source": "video-analysis-ai-knowledge-base-eli5.html",
        "title": {"ko": "AI 지식창고 ELI5", "en": "AI Knowledge Store ELI5", "jp": "AI知識倉庫 ELI5", "es": "AI-Scivendejo ELI5"},
        "summary": {
            "ko": "문서 전체 입력, 벡터 검색, SQL, 그래프 DB를 서랍·계산기·지도 비유로 설명합니다.",
            "en": "Explains full-document input, vector search, SQL, and graph databases with basket, calculator, and map metaphors.",
            "jp": "全文入力、ベクトル検索、SQL、グラフDBをかご・計算機・地図の比喩で説明します。",
            "es": "Klarigas tutdokumentan enigon, vektoran serĉon, SQL kaj grafajn datumbazojn per korbo, kalkulilo kaj mapo.",
        },
        "points": {
            "ko": ["가끔 묻는다면 문서를 통째로 넣는 방법이 단순합니다.", "문장 하나를 찾는 질문은 벡터·키워드 검색이 잘합니다.", "총액·평균·건수는 SQL처럼 DB가 계산하게 합니다.", "사람·점포·메뉴 관계는 그래프와 Cypher가 유용합니다.", "혼합 질문은 라우터와 폴백을 사용합니다."],
            "en": ["For rare questions, passing the whole document is simple.", "Vector and keyword search work well for finding a passage.", "Let a database such as SQL compute totals, averages, and counts.", "Graphs and Cypher help with people, store, and menu relationships.", "Use routing and fallback for mixed question types."],
            "jp": ["質問が少ないなら文書を丸ごと渡す方法が簡単です。", "一つの文章を探すならベクトル・キーワード検索が得意です。", "合計・平均・件数はSQLなどのDBに計算させます。", "人・店舗・メニューの関係にはグラフとCypherが有用です。", "混合質問にはルーターとフォールバックを使います。"],
            "es": ["Por maloftaj demandoj, transdoni la tutan dokumenton estas simple.", "Vektora kaj ŝlosilvorta serĉo bone trovas unu teksteron.", "Lasu datumbazon kiel SQL kalkuli sumojn, mezumojn kaj nombrojn.", "Grafikoj kaj Cypher helpas pri rilatoj inter homoj, vendejoj kaj menuoj.", "Uzu enkursigon kaj rezervan serĉon por miksitaj demandoj."],
        },
    },
]


def markdown(doc, code):
    lang = LANGS[code]
    lines = [
        f"# {doc['title'][code]}",
        "",
        f"> {lang['edition']} · [{lang['source']}](../../{doc['source']})",
        "",
        doc["summary"][code],
        "",
        "## " + {"ko": "핵심", "en": "Key points", "jp": "要点", "es": "Ĉefaj punktoj"}[code],
        "",
    ]
    lines.extend(f"- {item}" for item in doc["points"][code])
    lines += [
        "",
        "## " + {"ko": "공통 원칙", "en": "Shared principles", "jp": "共通原則", "es": "Komunaj principoj"}[code],
        "",
        {
            "ko": "로컬 우선, 안전한 메모리 여유, 검증된 모델 아티팩트, 런타임 독립 계약, 명시적 사용자 승인과 근거 기록을 기본으로 합니다.",
            "en": "Defaults are local-first execution, safe memory headroom, verified artifacts, runtime-neutral contracts, explicit approval, and evidence records.",
            "jp": "ローカル優先、安全なメモリ余裕、検証済みアーティファクト、ランタイム中立契約、明示的承認、根拠記録を基本とします。",
            "es": "La defaŭltoj estas loka-unua funkciado, sekura memorrezervo, validigitaj artefaktoj, rultempo-neŭtralaj kontraktoj, eksplicita aprobo kaj pruvaj registroj.",
        }[code],
        "",
        "---",
        "",
        {"ko": "다른 언어", "en": "Other languages", "jp": "他の言語", "es": "Aliaj lingvoj"}[code] + ": " + " · ".join(
            f"[{LANGS[other]['label']}]({doc['slug']}_{other}.md)" for other in LANGS
        ),
        "",
    ]
    return "\n".join(lines)


for doc in DOCS:
    for code in LANGS:
        (OUT / f"{doc['slug']}_{code}.md").write_text(markdown(doc, code), encoding="utf-8")


def language_panel(code):
    cards = []
    for doc in DOCS:
        cards.append(
            f'''<article class="doc-card"><span>📘</span><div><h3>{escape(doc["title"][code])}</h3>
            <p>{escape(doc["summary"][code])}</p>
            <a href="localized/{doc['slug']}_{code}.md">{escape({"ko":"문서 열기","en":"Open document","jp":"文書を開く","es":"Malfermi dokumenton"}[code])} →</a></div></article>'''
        )
    return "\n".join(cards)


tabs = "".join(
    f'<button class="lang-tab" role="tab" aria-selected="{str(i == 0).lower()}" data-lang="{code}">{info["label"]}</button>'
    for i, (code, info) in enumerate(LANGS.items())
)
panels = "".join(
    f'<section class="lang-panel" data-panel="{code}" lang="{info["html"]}" {"" if i == 0 else "hidden"}><div class="docs">{language_panel(code)}</div></section>'
    for i, (code, info) in enumerate(LANGS.items())
)

STYLE = """
:root{font-family:Inter,-apple-system,BlinkMacSystemFont,"Noto Sans",sans-serif;color:#17312e;background:#f3f7f5;line-height:1.55}*{box-sizing:border-box}body{margin:0}main{width:min(1040px,calc(100% - 28px));margin:28px auto 70px}.hero,.content{background:#fff;border:1px solid #dbe7e2;border-radius:28px;padding:clamp(24px,5vw,50px);box-shadow:0 14px 40px #17312e12}.hero{text-align:center;background:linear-gradient(135deg,#dff8ef,#fff 55%,#fff0bd)}h1{font-size:clamp(2rem,7vw,4.4rem);line-height:1.05;margin:10px}.hero-icon{font-size:clamp(4rem,14vw,8rem)}.tabs{display:flex;gap:8px;flex-wrap:wrap;justify-content:center;margin:24px 0}.lang-tab{border:1px solid #b9cec7;border-radius:999px;background:white;padding:11px 18px;font-weight:800;cursor:pointer}.lang-tab[aria-selected=true]{background:#106d61;color:white;border-color:#106d61}.docs{display:grid;grid-template-columns:repeat(2,1fr);gap:14px}.doc-card{display:flex;gap:16px;border:1px solid #dbe7e2;border-radius:20px;padding:20px;background:white}.doc-card>span{font-size:2rem}.doc-card h3{margin:0}.doc-card p{color:#5d716d}.doc-card a{color:#08665b;font-weight:800}.guide{margin-top:18px;text-align:center}.guide a{display:inline-block;background:#17312e;color:white;padding:13px 20px;border-radius:13px;text-decoration:none;font-weight:800}@media(max-width:700px){.docs{grid-template-columns:1fr}.doc-card{padding:16px}}
"""
SCRIPT = """
const tabs=[...document.querySelectorAll('.lang-tab')];
const panels=[...document.querySelectorAll('.lang-panel')];
function choose(code){tabs.forEach(t=>t.setAttribute('aria-selected',String(t.dataset.lang===code)));panels.forEach(p=>p.hidden=p.dataset.panel!==code);localStorage.setItem('mj-doc-language',code);}
tabs.forEach(t=>t.addEventListener('click',()=>choose(t.dataset.lang)));
choose(localStorage.getItem('mj-doc-language')||((navigator.language||'ko').startsWith('ja')?'jp':(navigator.language||'ko').startsWith('en')?'en':(navigator.language||'ko').startsWith('eo')?'es':'ko'));
"""

(ROOT / "docs" / "index.html").write_text(f'''<!doctype html><html lang="ko"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>MJ Local LLM Hub Docs</title><style>{STYLE}</style></head><body><main><header class="hero"><div class="hero-icon">🧠📚</div><h1>MJ Local LLM Hub</h1><p>한국어 · English · 日本語 · Esperanto</p></header><nav class="tabs" role="tablist" aria-label="Language">{tabs}</nav><div class="content">{panels}<div class="guide"><a href="user-guide.html">🎈 ELI5 User Guide</a></div></div></main><script>{SCRIPT}</script></body></html>''', encoding="utf-8")

GUIDE = {
    "ko": ("내 컴퓨터에 AI 친구 놓기", "1. 컴퓨터를 살펴봐요", "2. 맞는 모델을 골라요", "3. 내려받고 이야기해요", "작은 모델부터 시작하면 안전해요."),
    "en": ("Put an AI friend on your computer", "1. Check the computer", "2. Pick a model that fits", "3. Download and chat", "Start small. It is safer."),
    "jp": ("自分のパソコンにAIの友だちを置こう", "1. パソコンを調べる", "2. 入るモデルを選ぶ", "3. ダウンロードして話す", "小さいモデルから始めると安全です。"),
    "es": ("Metu AI-amikon en vian komputilon", "1. Kontrolu la komputilon", "2. Elektu modelon kiu taŭgas", "3. Elŝutu kaj babilu", "Komencu per malgranda modelo. Tio estas pli sekura."),
}
guide_panels = "".join(
    f'''<section class="lang-panel" data-panel="{code}" lang="{LANGS[code]["html"]}" {"" if i==0 else "hidden"}><h1>{escape(text[0])}</h1><div class="steps"><article><b>🩺</b><h2>{escape(text[1])}</h2><code>cargo run -- recommend</code></article><article><b>📦</b><h2>{escape(text[2])}</h2><p>RAM + model + context</p></article><article><b>💬</b><h2>{escape(text[3])}</h2><code>cargo run -- auto-install</code></article></div><div class="tip">🌱 {escape(text[4])}</div></section>'''
    for i,(code,text) in enumerate(GUIDE.items())
)
guide_style = STYLE + ".content{text-align:center}.steps{display:grid;grid-template-columns:repeat(3,1fr);gap:16px}.steps article{background:#f8fbfa;border:2px solid #dbe7e2;border-radius:24px;padding:26px}.steps b{font-size:4rem}.steps h2{font-size:1.25rem}.steps code{display:block;background:#17312e;color:#fff;padding:10px;border-radius:10px;overflow:auto}.tip{font-size:1.35rem;font-weight:850;background:#fff0bd;border-radius:18px;padding:20px;margin-top:18px}@media(max-width:700px){.steps{grid-template-columns:1fr}}"
(ROOT / "docs" / "user-guide.html").write_text(f'''<!doctype html><html lang="ko"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>MJ Local LLM Hub ELI5 Guide</title><style>{guide_style}</style></head><body><main><header class="hero"><div class="hero-icon">💻＋🧠＝✨</div></header><nav class="tabs" role="tablist" aria-label="Language">{tabs}</nav><div class="content">{guide_panels}<div class="guide"><a href="index.html">← Documentation</a></div></div></main><script>{SCRIPT}</script></body></html>''', encoding="utf-8")

print(f"Generated {len(DOCS) * len(LANGS)} Markdown editions plus two HTML pages")
