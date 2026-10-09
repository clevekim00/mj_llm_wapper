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
READER = ROOT / "docs" / "reader"
READER.mkdir(parents=True, exist_ok=True)
GITHUB_SOURCE = "https://github.com/clevekim00/mj_llm_wapper/blob/main/"

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
            "ko": "mj-llm 내장 LiteRT-LM으로 Ollama 없이 macOS CPU 대화와 텍스트 임베딩을 실행하는 Rust 미리보기입니다.",
            "en": "A Rust preview using embedded mj-llm/LiteRT-LM for macOS CPU chat and text embeddings without Ollama.",
            "jp": "mj-llm内蔵LiteRT-LMでOllamaなしにmacOS CPU会話とテキスト埋め込みを実行するRustプレビューです。",
            "es": "Rust-a antaŭvido kun enkonstruita mj-llm/LiteRT-LM por macOS-CPU-babilado kaj tekstaj enkorpigoj sen Ollama."
},
        "points": {
            "ko": [
                        "기본 런타임은 mj-llm입니다. Rust 1.95와 고정 LiteRT SDK에서 검증했습니다.",
                        "SDK 준비와 실행은 상세 원문의 docs/mj-llm-runtime.md 안내를 따르세요.",
                        "웹 UI는 http://127.0.0.1:3210에서 열립니다.",
                        "대화 모델은 Qwen3 0.6B, 임베딩 모델은 EmbeddingGemma 2입니다.",
                        "생성은 합계 1024-byte 입력·최대 32-token 출력·완료 후 표시 방식입니다.",
                        "Ollama는 MJ_HUB_RUNTIME=ollama로 선택합니다. 모바일 native와 토큰 스트리밍은 아직 미지원입니다."
            ],
            "en": [
                        "The default runtime is mj-llm, verified with Rust 1.95 and a pinned LiteRT SDK.",
                        "Follow docs/mj-llm-runtime.md in the canonical source for SDK setup and launch.",
                        "The web UI opens at http://127.0.0.1:3210.",
                        "Chat uses Qwen3 0.6B; embeddings use EmbeddingGemma 2.",
                        "Generation allows 1024 input bytes total and 32 output tokens, delivered after completion.",
                        "Select Ollama explicitly with MJ_HUB_RUNTIME=ollama. Native mobile and token streaming are not implemented."
            ],
            "jp": [
                        "既定ランタイムはmj-llmです。Rust 1.95と固定LiteRT SDKで検証しました。",
                        "SDKの準備と起動方法は詳細原文のdocs/mj-llm-runtime.mdを参照してください。",
                        "Web UIはhttp://127.0.0.1:3210で開きます。",
                        "会話はQwen3 0.6B、埋め込みはEmbeddingGemma 2を使います。",
                        "生成入力は合計1024 bytes、出力は最大32 tokensで、完了後に表示します。",
                        "OllamaはMJ_HUB_RUNTIME=ollamaで選択します。モバイルnativeとトークン配信は未実装です。"
            ],
            "es": [
                        "La defaŭlta rultempo estas mj-llm, kontrolita per Rust 1.95 kaj fiksita LiteRT SDK.",
                        "Sekvu docs/mj-llm-runtime.md en la detala fonto por SDK-agordo kaj lanĉo.",
                        "La reta fasado aperas ĉe http://127.0.0.1:3210.",
                        "Babilado uzas Qwen3 0.6B; enkorpigoj uzas EmbeddingGemma 2.",
                        "Generado permesas 1024 enigajn bajtojn kaj 32 eligajn tokenojn; la respondo aperas post fino.",
                        "Elektu Ollama per MJ_HUB_RUNTIME=ollama. Denaska poŝtelefona uzo kaj tokena fluado ankoraŭ ne estas realigitaj."
            ]
},
    },
    {
        "slug": "product-plan-local-llm-hub",
        "source": "product-plan-local-llm-hub.md",
        "title": {"ko": "제품 기획서", "en": "Product Plan", "jp": "製品企画書", "es": "Produkta Plano"},
        "summary": {
            "ko": "Ollama/Python 별도 설치 없이 PC·Android·iOS에서 실행하는 멀티모달 AI 제품의 목표 설계와 출시 단계입니다.",
            "en": "Target design and release stages for a multimodal AI app running on PCs, Android and iOS without separate Ollama or Python installations.",
            "jp": "OllamaやPythonの別途インストールなしでPC・Android・iOS上で動作するマルチモーダルAIの目標設計と開発段階です。",
            "es": "Cela projekto kaj eldonaj etapoj por plurmodala AI en komputiloj, Android kaj iOS sen aparta instalado de Ollama aŭ Python."
        },
        "points": {
            "ko": ["첫 출시는 문서·사진 검색과 근거 기반 대화를 기본 가정으로 합니다.", "검색용 EmbeddingGemma 2와 답변 생성 모델을 분리합니다.", "PC·모바일은 공통 Rust 코어와 내장 LiteRT-LM을 우선 검증합니다.", "기술 검증 → PC 흐름 → 모바일 출시 → 음성·영상 → MCP·생태계 순서입니다.", "현재 Ollama/Python 개발판과 목표 제품의 완료 상태를 구분합니다."],
            "en": ["The first release assumes document/image search and evidence-grounded chat.", "Separate EmbeddingGemma 2 retrieval from the answer-generation model.", "Validate a shared Rust core and embedded LiteRT-LM on desktop and mobile.", "Stages: feasibility, desktop, mobile, audio/video, then MCP and ecosystem.", "Distinguish the current Ollama/Python development build from the target product."],
            "jp": ["初回リリースは文書・画像検索と根拠付き対話を基本想定とします。", "検索用EmbeddingGemma 2と回答生成モデルを分離します。", "PCとモバイルで共通Rustコアと内蔵LiteRT-LMを検証します。", "技術検証、PC、モバイル、音声・動画、MCPの順に進めます。", "現在のOllama/Python開発版と目標製品の完成状態を区別します。"],
            "es": ["La unua eldono supozas dokumentan/bildan serĉon kaj pruv-bazitan babilon.", "Apartigu serĉon per EmbeddingGemma 2 de la respondgenera modelo.", "Validigu komunan Rust-kernon kaj enkonstruitan LiteRT-LM sur komputiloj kaj poŝtelefonoj.", "Etapoj: farebleco, komputilo, poŝtelefono, sono/video, poste MCP kaj ekosistemo.", "Distingu la nunan Ollama/Python-prototipon disde la cela produkto."]
        },
    },
    {
        "slug": "blueprint-local-llm-hub",
        "source": "blueprint-local-llm-hub.md",
        "title": {"ko": "통합 설계서", "en": "Integrated Architecture", "jp": "統合設計書", "es": "Integrita Arkitekturo"},
        "summary": {
            "ko": "PC·모바일 공통 Rust 코어, 내장 런타임, 멀티모달 자료·인덱스·API, 메모리·복구 정책과 구현 순서를 정의합니다.",
            "en": "Design for the shared Rust core, embedded runtimes, multimodal assets/indexes/APIs, resource recovery and migration steps.",
            "jp": "共通Rustコア、内蔵ランタイム、マルチモーダル資料・索引・API、資源管理・復旧と実装順序を定義します。",
            "es": "Projekto por komuna Rust-kerno, enkonstruitaj rultempoj, plurmodalaj datumoj/indeksoj/API, rimed-reakiro kaj migrado."
        },
        "points": {
            "ko": ["설계서 5장에 목표 아키텍처, 데이터 계약, API와 파일별 전환 작업을 정리했습니다.", "모바일은 native bridge로 코어를 호출하고 PC HTTP API는 선택형으로 둡니다.", "LiteRT-LM을 PC·모바일 주력 후보로 검증하며 Ollama는 새 제품 의존성에서 제외합니다.", "모델·양자화·차원·전처리가 다른 벡터 공간은 혼합하지 않습니다.", "실기기 오프라인, 취소·복원, 멀티모달 검색 품질을 출시 gate로 삼습니다."],
            "en": ["Chapter 5 defines the target architecture, data contracts, APIs and file-level migration.", "Mobile calls the core through native bridges; desktop HTTP is optional.", "Validate LiteRT-LM as the primary embedded engine and remove Ollama from product dependencies.", "Keep vectors with different model, quantization, dimension or preprocessing profiles separate.", "Release gates cover physical-device offline use, cancellation/recovery and multimodal retrieval quality."],
            "jp": ["第5章に目標構成、データ契約、API、ファイル別移行作業をまとめています。", "モバイルはnative bridgeでコアを呼び、PCのHTTP APIは任意にします。", "LiteRT-LMを主エンジン候補として検証し、Ollama依存を外します。", "モデル・量子化・次元・前処理が異なるベクトルを混ぜません。", "実機オフライン、取消・復旧、マルチモーダル検索品質を公開条件とします。"],
            "es": ["Ĉapitro 5 difinas la celan arkitekturon, datumkontraktojn, API kaj dosieran migradon.", "Poŝtelefonoj vokas la kernon per denaskaj pontoj; komputila HTTP estas laŭvola.", "Validigu LiteRT-LM kiel ĉefan enkonstruitan motoron kaj forigu la produktan dependecon de Ollama.", "Ne miksu vektorojn kun malsamaj modeloj, kvantigoj, dimensioj aŭ antaŭtraktado.", "Eldonaj kontroloj kovras senretan uzon sur realaj aparatoj, nuligon/reakiron kaj plurmodalan serĉkvaliton."]
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
        "[" + {"ko": "페이지에서 읽기", "en": "Read on the page", "jp": "ページで読む", "es": "Legi en la paĝo"}[code] + "](" + (f"../readme.html#lang={code}" if doc["slug"] == "README" else f"../reader/{doc['slug']}.html#lang={code}") + ")",
        "",
        {"ko": "다른 언어", "en": "Other languages", "jp": "他の言語", "es": "Aliaj lingvoj"}[code] + ": " + " · ".join(
            f"[{LANGS[other]['label']}]({(f'../readme.html#lang={other}' if doc['slug'] == 'README' else f'../reader/{doc["slug"]}.html#lang={other}')})" for other in LANGS
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
        href = f"readme.html#lang={code}" if doc["slug"] == "README" else f"reader/{doc['slug']}.html#lang={code}"
        cards.append(
            f'''<article class="doc-card"><span>📘</span><div><h3>{escape(doc["title"][code])}</h3>
            <p>{escape(doc["summary"][code])}</p>
            <a href="{href}">{escape({"ko":"문서 열기","en":"Open document","jp":"文書を開く","es":"Malfermi dokumenton"}[code])} →</a></div></article>'''
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
:root{font-family:Inter,-apple-system,BlinkMacSystemFont,"Noto Sans",sans-serif;color:#eaf2fb;background:#0b1117;line-height:1.6}*{box-sizing:border-box}body{margin:0}main{width:min(1080px,calc(100% - 28px));margin:30px auto 70px}.hero{padding:clamp(34px,7vw,72px);text-align:center;border-bottom:1px solid #2b3540}.hero img{width:112px;height:112px;border-radius:24px}.hero h1{font-size:clamp(2.3rem,7vw,4.4rem);line-height:1.05;margin:18px 0 12px}.hero p{font-size:clamp(1rem,2.5vw,1.35rem);color:#aab8c7}.tabs{display:flex;gap:9px;flex-wrap:wrap;justify-content:center;margin:26px 0}.lang-tab{border:1px solid #34414e;border-radius:9px;background:#151d26;color:#dbe8f5;padding:11px 18px;font-weight:800;cursor:pointer}.lang-tab:hover,.lang-tab:focus-visible{border-color:#16c7d8;outline:none}.lang-tab[aria-selected=true]{background:#0b8394;color:white;border-color:#22d3e5}.content{background:#101820;border:1px solid #293541;border-radius:18px;padding:clamp(20px,4vw,42px)}.docs{display:grid;grid-template-columns:repeat(2,1fr);gap:14px}.doc-card{display:flex;gap:16px;border:1px solid #2c3945;border-radius:14px;padding:22px;background:#0d141b}.doc-card>span{font-size:1.1rem;color:#1cd6e4}.doc-card h3{margin:0;color:#f2f7fc}.doc-card p{color:#aab8c7}.doc-card a,.source-link{color:#36d8e6;font-weight:800}.guide{margin-top:22px;text-align:center}.guide a{display:inline-block;background:#eaf2fb;color:#0b1117;padding:13px 20px;border-radius:9px;text-decoration:none;font-weight:850}.reader{max-width:820px;margin:auto}.reader h1{font-size:clamp(2rem,6vw,3.4rem);line-height:1.12}.reader .lead{font-size:1.2rem;color:#c3d0dc}.reader li{margin:.7rem 0}.reader code{background:#061018;border:1px solid #2a3b47;padding:.15rem .4rem;border-radius:5px}.reader-nav{display:flex;justify-content:space-between;gap:12px;flex-wrap:wrap;margin-top:30px}.reader-nav a{color:#36d8e6}.step-number{font-size:3rem;font-weight:900;color:#24d4e4}@media(max-width:700px){.docs{grid-template-columns:1fr}.doc-card{padding:16px}.hero img{width:88px;height:88px}}
"""
SCRIPT = """
const tabs=[...document.querySelectorAll('.lang-tab')];
const panels=[...document.querySelectorAll('.lang-panel')];
const valid=new Set(tabs.map(t=>t.dataset.lang));
function choose(code,writeHash=false){if(!valid.has(code))code='ko';tabs.forEach(t=>t.setAttribute('aria-selected',String(t.dataset.lang===code)));panels.forEach(p=>p.hidden=p.dataset.panel!==code);localStorage.setItem('mj-doc-language',code);document.documentElement.lang=({ko:'ko',en:'en',jp:'ja',es:'eo'})[code];if(writeHash)history.replaceState(null,'','#lang='+code);}
tabs.forEach(t=>t.addEventListener('click',()=>choose(t.dataset.lang,true)));
const direct=new URLSearchParams(location.hash.slice(1)).get('lang');
choose(direct||localStorage.getItem('mj-doc-language')||((navigator.language||'ko').startsWith('ja')?'jp':(navigator.language||'ko').startsWith('en')?'en':(navigator.language||'ko').startsWith('eo')?'es':'ko'));
"""

(ROOT / "docs" / "index.html").write_text(f'''<!doctype html><html lang="ko"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>MJ Local LLM Hub Docs</title><style>{STYLE}</style></head><body><main><header class="hero"><img src="assets/mj-local-llm-hub-logo.png" alt="MJ Local LLM Hub"><h1>MJ Local LLM Hub</h1><p>Local-first model installer, chat, RAG &amp; MCP hub</p></header><nav class="tabs" role="tablist" aria-label="Language">{tabs}</nav><div class="content">{panels}<div class="guide"><a href="user-guide.html">ELI5 User Guide</a></div></div></main><script>{SCRIPT}</script></body></html>''', encoding="utf-8")


def inline_code(value):
    parts = value.split("`")
    return "".join(f"<code>{escape(part)}</code>" if i % 2 else escape(part) for i, part in enumerate(parts))


def reader_panel(doc, code, first, site_prefix):
    source_labels = {"ko": "상세 원문", "en": "Canonical source", "jp": "詳細な原文", "es": "Detala fonto"}
    raw_labels = {"ko": "Markdown 판", "en": "Markdown edition", "jp": "Markdown版", "es": "Markdown-eldono"}
    points = "".join(f"<li>{inline_code(point)}</li>" for point in doc["points"][code])
    return f'''<section class="lang-panel reader" data-panel="{code}" lang="{LANGS[code]['html']}" {'' if first else 'hidden'}>
      <h1>{escape(doc['title'][code])}</h1><p class="lead">{escape(doc['summary'][code])}</p><ul>{points}</ul>
      <div class="reader-nav"><a href="{GITHUB_SOURCE}{doc['source']}">{escape(source_labels[code])}</a><a href="{site_prefix}localized/{doc['slug']}_{code}.md">{escape(raw_labels[code])}</a><a href="{site_prefix}index.html#lang={code}">Documentation</a></div>
    </section>'''


for doc in DOCS:
    is_readme = doc["slug"] == "README"
    site_prefix = "" if is_readme else "../"
    output = ROOT / "docs" / "readme.html" if is_readme else READER / f"{doc['slug']}.html"
    reader_panels = "".join(reader_panel(doc, code, i == 0, site_prefix) for i, code in enumerate(LANGS))
    output.write_text(
        f'''<!doctype html><html lang="ko"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{escape(doc['title']['en'])} · MJ Local LLM Hub</title><style>{STYLE}</style></head><body><main><header class="hero"><img src="{site_prefix}assets/mj-local-llm-hub-logo.png" alt="MJ Local LLM Hub"><p>MJ Local LLM Hub Documentation</p></header><nav class="tabs" role="tablist" aria-label="Language">{tabs}</nav><div class="content">{reader_panels}</div></main><script>{SCRIPT}</script></body></html>''',
        encoding="utf-8",
    )

GUIDE = {
    "ko": ("내 컴퓨터에 AI 친구 놓기", "1. 컴퓨터를 살펴봐요", "2. 맞는 모델을 골라요", "3. 내려받고 이야기해요", "작은 모델부터 시작하면 안전해요."),
    "en": ("Put an AI friend on your computer", "1. Check the computer", "2. Pick a model that fits", "3. Download and chat", "Start small. It is safer."),
    "jp": ("自分のパソコンにAIの友だちを置こう", "1. パソコンを調べる", "2. 入るモデルを選ぶ", "3. ダウンロードして話す", "小さいモデルから始めると安全です。"),
    "es": ("Metu AI-amikon en vian komputilon", "1. Kontrolu la komputilon", "2. Elektu modelon kiu taŭgas", "3. Elŝutu kaj babilu", "Komencu per malgranda modelo. Tio estas pli sekura."),
}
guide_panels = "".join(
    f'''<section class="lang-panel" data-panel="{code}" lang="{LANGS[code]["html"]}" {"" if i==0 else "hidden"}><h1>{escape(text[0])}</h1><div class="steps"><article><b class="step-number">01</b><h2>{escape(text[1])}</h2><code>bash scripts/run-mj-llm-macos.sh recommend</code></article><article><b class="step-number">02</b><h2>{escape(text[2])}</h2><p>RAM + model + context</p></article><article><b class="step-number">03</b><h2>{escape(text[3])}</h2><code>bash scripts/run-mj-llm-macos.sh auto-install</code></article></div><div class="tip">{escape(text[4])}</div></section>'''
    for i,(code,text) in enumerate(GUIDE.items())
)
guide_style = STYLE + ".content{text-align:center}.steps{display:grid;grid-template-columns:repeat(3,1fr);gap:16px}.steps article{background:#0d141b;border:1px solid #2c3945;border-radius:14px;padding:26px}.steps h2{font-size:1.25rem}.steps code{display:block;background:#061018;color:#fff;padding:10px;border-radius:7px;overflow:auto}.tip{font-size:1.25rem;font-weight:850;background:#12343a;color:#baf8ff;border-radius:12px;padding:20px;margin-top:18px}@media(max-width:700px){.steps{grid-template-columns:1fr}}"
(ROOT / "docs" / "user-guide.html").write_text(f'''<!doctype html><html lang="ko"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>MJ Local LLM Hub ELI5 Guide</title><style>{guide_style}</style></head><body><main><header class="hero"><img src="assets/mj-local-llm-hub-logo.png" alt="MJ Local LLM Hub"><h1>ELI5</h1><p>Local AI, explained simply.</p></header><nav class="tabs" role="tablist" aria-label="Language">{tabs}</nav><div class="content">{guide_panels}<div class="guide"><a href="index.html">Documentation</a></div></div></main><script>{SCRIPT}</script></body></html>''', encoding="utf-8")

print(f"Generated {len(DOCS) * len(LANGS)} Markdown editions, {len(DOCS)} tabbed readers, and two HTML hubs")
