# EmbeddingGemma 2 적용

확인일: 2026-10-08. [Google 공식 설명](https://ai.google.dev/gemma/docs/embeddinggemma),
[모델 카드](https://huggingface.co/google/embeddinggemma-2),
[라이브러리 요구사항·접두어 설정](https://huggingface.co/google/embeddinggemma-2/blob/main/config_sentence_transformers.json).

## 모델 요약

EmbeddingGemma 2는 답변을 생성하는 채팅 모델이 아니라 검색·분류에 사용할 벡터를 만드는 모델이다.
전체 740M 파라미터이고, 텍스트만 실행할 때는 이미지·오디오 인코더를 제외해 270M 구성으로 사용할 수 있다.
100개 이상 언어와 코드, 이미지·음성·영상을 지원하며 입력 한도는 8,192 토큰이다.
기본 벡터는 768차원이고 512·256·128차원으로 줄일 수 있다. Apache-2.0 라이선스다.
공식 평가에서 코드 검색이 이전 세대보다 개선되었지만 프로젝트의 한국어 문서 검색 품질은 별도로 측정해야 한다.

검색 질문과 문서는 서로 다른 접두어를 사용한다. 차원을 줄인 뒤에는 다시 정규화해야 한다.
수치 정밀도는 float32 또는 bfloat16을 사용하며 float16은 피한다.

## 이번 프로젝트의 구현 범위

- Rust 게이트웨이 → 인증된 loopback Python 서비스 → SentenceTransformers 순서로 실행한다.
- `/v1/embeddings`는 문자열 또는 최대 32개 문자열을 받아 정규화된 벡터를 반환한다.
- 기본값은 문서용·768차원이다. 검색어는 `mj.task=search_query`, 코드 검색어는 `code_retrieval`로 지정한다.
- 문서는 `mj.titles`로 제목을 전달할 수 있다. 분류·군집·유사도 비교도 task로 구분한다.
- 접두어 포함 토큰 한도를 넘으면 413으로 거부한다. 호출자가 문서를 분할해야 한다.
- 모델은 서비스 시작 시 한 번 로드하고 요청 간 재사용한다. 메모리 부담을 줄이기 위해 한 번에 한 입력씩 계산하며 동시 요청은 429로 거부한다.
- 서비스가 준비되었을 때만 `/v1/models`에 임베딩 모델을 표시한다. 채팅 카탈로그와 자동 설치 대상에는 넣지 않는다.
- 이미지·음성·영상 입력, 벡터 DB, 문서 수집, 검색 결과를 채팅에 주입하는 전체 RAG는 아직 구현하지 않았다.

## 설치·실행

Python 3.10+ 환경에서 실행한다. 모델 설정은 sentence-transformers 6.1.0 이상을 요구한다.
Transformers에는 `embedding_gemma2` 아키텍처가 포함되어야 한다.
CPU가 기본이고 Apple Silicon은 `--device mps`, CUDA는 `--device cuda`를 선택할 수 있다.
실제 메모리 사용량은 장치·길이·라이브러리에 따라 달라지며 파라미터 수만으로 보장하지 않는다.

```bash
python3 -m venv .venv-embeddings
.venv-embeddings/bin/python -m pip install -U -r scripts/requirements-embeddings.txt

# 현재 셸과 하위 프로세스에서 같은 토큰을 사용한다.
export MJ_HUB_TOKEN="$(python3 -c 'import secrets; print(secrets.token_hex(24))')"
export MJ_EMBEDDING_URL=http://127.0.0.1:3211

# 처음에는 Hugging Face 모델 파일을 내려받는다.
.venv-embeddings/bin/python scripts/embedding_server.py &
EMBEDDING_PID=$!
cargo run -- serve

# 종료 후 서비스도 종료한다.
kill "$EMBEDDING_PID"
```

캐시가 준비된 이후에는 `--offline`으로 네트워크 다운로드 없이 실행할 수 있다.
모델 가중치와 의존성은 저장소에 포함하지 않는다. `MJ_EMBEDDING_URL`을 생략하면
기존 채팅 기능만 실행되고 임베딩 API는 503을 반환한다. 서비스 URL은 HTTP loopback IP만 허용하며
리다이렉트와 프록시를 사용하지 않는다. Python 서비스에도 동일한 토큰이 필요하다.

## 호출 예제

서버와 동일한 `MJ_HUB_TOKEN`을 설정한 셸에서 호출한다.

```bash
curl http://127.0.0.1:3210/v1/embeddings \
  -H "Authorization: Bearer $MJ_HUB_TOKEN" \
  -H 'Content-Type: application/json' \
  -d '{"model":"google/embeddinggemma-2","input":"모델을 어떻게 설치하나요?","dimensions":256,"mj":{"task":"search_query"}}'

curl http://127.0.0.1:3210/v1/embeddings \
  -H "Authorization: Bearer $MJ_HUB_TOKEN" \
  -H 'Content-Type: application/json' \
  -d '{"model":"google/embeddinggemma-2","input":["cargo run -- auto-install 명령으로 추천 모델을 설치합니다."],"dimensions":256,"mj":{"task":"document","titles":["모델 설치"]}}'
```

질문과 문서에 동일한 모델·차원을 사용하고 정규화 벡터의 내적으로 순위를 계산한다.
기존 인덱스가 있다면 모델 ID, 모델 revision, 차원, 문서 접두어 규칙을 함께 기록하고
변경 시 문서를 재임베딩한다. 다른 모델의 벡터와 혼합하지 않는다.
`encoding_format`은 `float`만 지원하며 토큰 ID 배열과 미디어 입력은 거부한다.
API 상세 계약은 [OpenAPI](../contracts/openapi.yaml)를 참고한다.

## 검증

```bash
cargo test
cargo clippy --all-targets -- -D warnings
python3 -m unittest discover -s scripts -p 'test_embedding_server.py' -v
# 실제 서비스와 허브 실행 후, 동일한 MJ_HUB_TOKEN이 있는 셸에서:
python3 scripts/smoke_embeddings.py
```

단위 테스트는 가중치 없이 입력 검증·task별 포맷·차원·정규화 옵션·토큰 한도·인증을 검사한다.
실제 모델 검증은 별도로 위 서비스를 실행한 후 한국어 관련/무관 문서의 검색 순위와
128/256/512/768차원 출력 길이·유한값·단위 노름을 확인해야 한다.

### 2026-10-08 실행 결과

macOS CPU float32에서 Python 3.13.7, sentence-transformers 6.1.0,
transformers 5.19.0, torch 2.14.1, torchvision 0.29.1, Pillow 12.3.0으로 검증했다.
텍스트 전용 모델도 공용 프로세서를 로드하므로 Pillow와 torchvision이 필요했다.
캐시 다운로드 후 `--offline`으로 Python 서비스와 Rust 게이트웨이를 실행했다.
인증 거부, 모델 목록 노출, 잘못된 차원 거부, 네 차원의 출력 길이·유한값·정규화를 통과했다.

`scripts/smoke_embeddings.py`의 한국어 설치 질문에 대한 코사인 유사도:

| 차원 | 설치 안내 문서 | 무관한 요리 문서 |
| --- | --- | --- |
| 128 | 0.8055 | 0.6933 |
| 256 | 0.7760 | 0.5844 |
| 512 | 0.7712 | 0.5743 |
| 768 | 0.7725 | 0.5718 |

이는 한 질문·두 문서의 연결 확인 테스트이며 검색 품질 전체를 보장하는 벤치마크는 아니다.
MPS/CUDA 실행과 장시간 부하 검증은 수행하지 않았다. 검증용 서버는 종료했다.
