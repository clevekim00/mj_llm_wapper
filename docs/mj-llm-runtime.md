# mj-llm 내장 런타임 적용

기본 런타임은 `mj-llm`이다. 기존 웹 UI·CLI·인증 API에 Rust → mj-llm →
LiteRT-LM CPU 실행을 연결했다. Ollama와 Python 서버는 필요 없다.
현재 native 구현·실행 검증 대상은 **macOS arm64**다. Windows/Linux/모바일
native 지원 완료를 뜻하지 않는다. SDK 없는 기본 빌드도 가능하지만 추론은
`runtime_unavailable`로 실패하며 Ollama로 자동 전환하지 않는다.

## 준비

Rust 1.95에서 검증한다. macOS C++ 빌드 도구와 고정 LiteRT SDK가 필요하다.
SDK 다운로드 주소·체크섬은 `vendor/mj-llm/catalog/n0.lock.json`에 있다.
원본 N0 파일은 아직 미게시 작업공간에서 가져왔으며, 다음 명령으로 SDK를 준비한다.

```bash
mkdir -p .mj-local-llm-hub/sdk
curl -fL https://github.com/google-ai-edge/LiteRT-LM/releases/download/v0.18.0/CLiteRTLM_mac.xcframework.zip -o .mj-local-llm-hub/sdk/sdk.zip
printf '%s\n' '5f6ee68d95eeccb084c6e66d5ee47255e3020fa0fb29696dd0301ae26d6cfb4f  .mj-local-llm-hub/sdk/sdk.zip' | shasum -a 256 -c - && unzip -q .mj-local-llm-hub/sdk/sdk.zip -d .mj-local-llm-hub/sdk
export MJ_LITERT_SDK_DIR="$PWD/.mj-local-llm-hub/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64"
```

빌드할 때 SDK 헤더와 라이브러리 해시를 다시 검사한다.

## 설치·실행

현재 개발 Mac에는 검증된 SDK와 모델 두 개를 `.mj-local-llm-hub`에 준비했다.
이 장치에서는 프로젝트 루트에서 `bash scripts/run-mj-llm-macos.sh`만 실행하면 된다.
이 파일들은 Git에서 제외되므로 다른 clone에서는 아래 준비·설치 과정이 필요하다.


```bash
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" recommend
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" install mj-llm/qwen3-0.6b
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" install google/embeddinggemma-2
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" serve
```

설치는 CLI에서 용량과 확인을 표시한다 (`--yes` 지원). 웹 모델 센터의 설치 버튼도
같은 고정 artifact를 다운로드한다. revision·크기·SHA-256 검증 후에만 임시 파일을
모델 경로로 원자적으로 옮긴다. 다운로드 중단 재개는 아직 없다. 대화 모델은 약
614 MB, 검색 모델은 약 388 MB이며 모델 라이선스는 각 Hugging Face 원본을 따른다.
`auto-install`은 먼저 안전한 미설치 대화 모델을 선택한다.

기존 N0 파일이 있으면 재다운로드 없이 다음 환경 변수를 사용한다. 파일은 SDK가
사용하는 동안 다른 프로그램에서 변경하지 않아야 한다.

```bash
export MJ_LLM_GENERATION_MODEL=/absolute/path/to/Qwen3-0.6B.litertlm
export MJ_LLM_EMBEDDING_MODEL=/absolute/path/to/embeddinggemma-2-text-vision-440m.litertlm
bash scripts/run-mj-llm-macos.sh "$MJ_LITERT_SDK_DIR" serve
```

브라우저: `http://127.0.0.1:3210`. 데이터 기본 위치는 `.mj-local-llm-hub`.
`MJ_HUB_DATA_DIR`로 데이터/캐시 위치, `MJ_LLM_MODEL_DIR`로 기본 모델 디렉터리를
변경할 수 있다. 위 파일별 변수는 모델 디렉터리보다 우선한다.

## API 범위

- `/api/status`: `kind=mj-llm`, `endpoint_id=embedded://litert-cpu`.
- `/api/models`: 이 runtime의 Qwen3/EmbeddingGemma artifact만 표시한다.
- `/v1/models`: 무결성 확인된 설치 모델과 모델별 capability를 표시한다.
- `/api/chat`: system/user/assistant 기록을 역할별로 전달하고 최종 답변을 SSE로
  한 번 보낸다. 토큰별 스트리밍이 아니며 metrics의 `delivery=buffered`로 구분한다.
- `/v1/chat/completions`: `model=mj-llm/qwen3-0.6b`, 문자열 messages,
  `stream=false`, 선택형 `max_tokens=1..32`. 도구 호출·JSON 강제 출력·샘플링 옵션·
  이미지 입력·토큰 스트리밍은 명시적으로 거부한다. SDK가 종료 이유·토큰 사용량을
  제공하지 않아 `finish_reason=null`, usage 생략, `mj`에 미제공 상태를 표시한다.
- `/v1/embeddings`: `model=google/embeddinggemma-2`, 문자열 또는 최대 8개 문자열,
  float 768차원. `mj.task=document`(기본)/`search_query` 지원. 제목·다른 차원·
  이미지 API는 아직 연결하지 않았다. mj-llm 자체의 이미지 embedding 기능과 구분한다.
  반환하는 `mj.embedding_space_id`가 다른 벡터는 같은 인덱스에 혼합하지 않는다.
- `MJ_EMBEDDING_URL`을 명시하면 기존 Python text reference 서비스가 embedding
  요청만 처리한다. 기본값에는 외부 embedding 서비스가 없다.

현재 생성 한도: 모든 메시지 본문 합계 UTF-8 1024 bytes, native context 512 tokens,
출력 최대 32 tokens. Qwen3 원본 probe와 같은 `/no_think` 접미사를 마지막 요청에
적용하며 저장된 사용자 메시지는 변경하지 않는다. 긴 대화는 거부하며 조용히 자르지 않는다. 이는 짧은 대화용
기술 미리보기이며 기존 대형 Ollama 모델의 품질·기능과 동등하지 않다.
embedding은 입력별 8192-byte 사전 제한과 SDK 512-token overflow-error를 적용한다.

요청마다 전용 blocking 작업 안에서 load → infer → unload한다. 작업 admission은
한 개로 제한하며 바쁘면 429를 반환한다. HTTP timeout 또는 연결 해제 후에도 이미
시작한 동기 SDK 호출은 완료될 때까지 permit를 보유한다. 즉시 native 취소와 상주
모델 재사용은 아직 미구현이다. 설치 중 연결이 끊겨도 시작한 검증/설치는 마무리된다.

## 기존 Ollama 사용

명시적 선택일 때만 기존 카탈로그와 어댑터를 사용한다.

```bash
MJ_HUB_RUNTIME=ollama cargo run -- serve
```

기존 대화는 남아 있으며 모델 태그를 새 artifact로 자동 변경하지 않는다.

## 검증

```bash
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
# 실제 SDK와 모델 파일을 설정한 뒤:
export DYLD_LIBRARY_PATH="$MJ_LITERT_SDK_DIR"
cargo test --release --features native-macos --locked --test mj_llm_native -- --ignored --nocapture
```

출처와 변경 범위: [vendor 기록](../vendor/mj-llm/README.md).

### 이 변경에서 확인한 결과 (2026-10-08)

- 일반 Rust 테스트 18개, 기본/native Clippy, fmt 통과.
- 실제 native 통합 테스트: 역할별 대화 기록의 숫자 회상, Qwen3 생성,
  EmbeddingGemma 2의 768차원 출력, 미지원 스트리밍 거부 후 재요청 성공.
- `python3 scripts/smoke_mj_llm.py`: 별도 임시 데이터·loopback 포트로 서버를 띄워
  인증, 런타임별 모델 목록, 생성 및 provenance, 미지원 옵션, 벡터 norm,
  웹 SSE와 대화 저장, timeout 이후 동시 추론 차단을 검증했다.
- `bash scripts/run-mj-llm-macos.sh recommend --json`: 로컬 SDK와 두 모델 준비 확인.
- 네트워크 모델 다운로드 경로의 실다운로드는 이번 검증에서 실행하지 않았다.
  이미 받은 고정 hash 모델을 사용했다. 다른 OS·모바일·실제 토큰 스트리밍은 미검증이다.
