#!/usr/bin/env python3
"""Verify a running hub with real EmbeddingGemma 2 weights (no mock vectors)."""
import argparse
import json
import math
import os
from urllib.error import HTTPError
from urllib.request import Request, urlopen

MODEL = "google/embeddinggemma-2"


def run(endpoint, token):
    def call(path, body=None, authenticated=True):
        headers = {"Content-Type": "application/json"}
        if authenticated:
            headers["Authorization"] = f"Bearer {token}"
        request = Request(endpoint + path, headers=headers,
                          data=None if body is None else json.dumps(body).encode())
        with urlopen(request, timeout=120) as response:
            return json.load(response)

    try:
        call("/v1/embeddings", {"model": MODEL, "input": "test"}, False)
        raise AssertionError("Unauthenticated request was accepted")
    except HTTPError as error:
        assert error.code == 401
    models = call("/v1/models")
    assert any(model["id"] == MODEL for model in models["data"])
    scores = {}
    for dimensions in (128, 256, 512, 768):
        query = call("/v1/embeddings", {"model": MODEL, "input": "로컬 모델을 어떻게 설치하나요?",
                     "dimensions": dimensions, "mj": {"task": "search_query"}})
        documents = call("/v1/embeddings", {"model": MODEL, "input": [
            "cargo run -- auto-install 명령을 실행하면 추천된 로컬 언어 모델을 내려받아 설치합니다.",
            "토마토와 달걀을 넣고 볶으면 간단한 요리를 만들 수 있습니다.",
        ], "dimensions": dimensions, "mj": {"task": "document", "titles": ["설치 안내", "요리법"]}})
        vectors = [query["data"][0]["embedding"]] + [row["embedding"] for row in documents["data"]]
        for vector in vectors:
            assert len(vector) == dimensions
            assert all(math.isfinite(value) for value in vector)
            assert abs(sum(value * value for value in vector) - 1) < 0.01
        relevant, unrelated = [sum(a*b for a, b in zip(vectors[0], doc)) for doc in vectors[1:]]
        assert relevant > unrelated, (dimensions, relevant, unrelated)
        scores[dimensions] = {"relevant": round(relevant, 4), "unrelated": round(unrelated, 4)}
    try:
        call("/v1/embeddings", {"model": MODEL, "input": "test", "dimensions": 300})
        raise AssertionError("Invalid dimension was accepted")
    except HTTPError as error:
        assert error.code == 400
        assert json.load(error)["error"]["code"] == "invalid_request"
    print(json.dumps({"model": MODEL, "checks": "passed", "scores": scores}, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--endpoint", default="http://127.0.0.1:3210")
    args = parser.parse_args()
    run(args.endpoint.rstrip("/"), os.environ["MJ_HUB_TOKEN"])
