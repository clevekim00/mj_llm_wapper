#!/usr/bin/env python3
"""Opt-in real native HTTP smoke. Python is a test tool, never a product service."""
import json
import math
import os
from pathlib import Path
import secrets
import socket
import subprocess
import tempfile
import time
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
CHAT = "mj-llm/qwen3-0.6b"
EMBED = "google/embeddinggemma-2"


def main():
    env = os.environ.copy()
    env["MJ_HUB_RUNTIME"] = "mj-llm"
    env.pop("MJ_EMBEDDING_URL", None)
    env["MJ_LLM_GENERATION_MODEL"] = env.get("MJ_LLM_GENERATION_MODEL", str(ROOT / ".mj-local-llm-hub/models/Qwen3-0.6B.litertlm"))
    env["MJ_LLM_EMBEDDING_MODEL"] = env.get("MJ_LLM_EMBEDDING_MODEL", str(ROOT / ".mj-local-llm-hub/models/embeddinggemma-2-text-vision-440m.litertlm"))
    env["DYLD_LIBRARY_PATH"] = env.get("MJ_LITERT_SDK_DIR", str(ROOT / ".mj-local-llm-hub/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64"))
    for key in ("MJ_LLM_GENERATION_MODEL", "MJ_LLM_EMBEDDING_MODEL"):
        assert Path(env[key]).is_file(), f"Required artifact missing: {key}"
    with socket.socket() as probe:
        probe.bind(("127.0.0.1", 0))
        port = probe.getsockname()[1]
    token = secrets.token_hex(24)
    env.update(MJ_HUB_PORT=str(port), MJ_HUB_BIND="127.0.0.1", MJ_HUB_TOKEN=token)
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))

    def call(path, body=None, auth=True):
        headers = {"Content-Type": "application/json"}
        if auth:
            headers["Authorization"] = f"Bearer {token}"
        request = urllib.request.Request(f"http://127.0.0.1:{port}{path}", data=None if body is None else json.dumps(body).encode(), headers=headers)
        try:
            with opener.open(request, timeout=60) as response:
                raw = response.read().decode()
                return response.status, json.loads(raw) if "application/json" in response.headers.get("Content-Type", "") else raw
        except urllib.error.HTTPError as error:
            return error.code, json.loads(error.read())

    with tempfile.TemporaryDirectory(prefix="mj-hub-smoke-") as temp:
        env["MJ_HUB_DATA_DIR"] = temp
        with open(Path(temp) / "server.log", "w+") as log:
            process = subprocess.Popen([str(ROOT / "target/release/mj-local-llm-hub"), "serve"], cwd=ROOT, env=env, stdout=log, stderr=log)
            try:
                for _ in range(120):
                    if process.poll() is not None:
                        raise RuntimeError("native server exited before becoming ready")
                    try:
                        status, body = call("/api/status")
                        break
                    except (OSError, urllib.error.URLError):
                        time.sleep(0.25)
                else:
                    raise RuntimeError("native server did not start")
                assert status == 200 and body["runtime"]["kind"] == "mj-llm"
                assert body["runtime"]["reachable"] and set(body["runtime"]["installed_models"]) == {CHAT, EMBED}
                assert call("/api/status", auth=False)[0] == 401
                rows = call("/api/models")[1]["recommended"]
                assert {r["runtime_model"] for r in rows} == {CHAT, EMBED}
                models = {r["id"]: r for r in call("/v1/models")[1]["data"]}
                assert not models[EMBED]["mj"]["capabilities"]["chat"]
                assert not models[CHAT]["mj"]["capabilities"]["embeddings"]
                request = {"model": CHAT, "messages": [{"role": "user", "content": "What is 2+2? Reply briefly."}]}
                status, response = call("/v1/chat/completions", request)
                assert status == 200 and "4" in response["choices"][0]["message"]["content"]
                assert response["mj"]["provider"] == "mj-llm" and response["mj"]["delivery"] == "buffered"
                assert response["mj"]["model_digest"]
                assert call("/v1/chat/completions", {**request, "stream": True})[0] == 422
                assert call("/v1/chat/completions", {**request, "response_format": {"type": "json_object"}})[0] == 422
                status, vectors = call("/v1/embeddings", {"model": EMBED, "input": ["고양이", "강아지"], "mj": {"task": "search_query"}})
                assert status == 200 and len(vectors["data"]) == 2
                for row in vectors["data"]:
                    assert len(row["embedding"]) == 768
                    assert all(math.isfinite(v) for v in row["embedding"])
                    assert abs(sum(v*v for v in row["embedding"]) - 1) < 0.001
                assert vectors["mj"]["embedding_space_id"].startswith("sha256:")
                status, events = call("/api/chat", request)
                assert status == 200 and events.count("event: done") == 1 and events.count("event: token") == 1 and "event: error" not in events
                conversations = call("/api/conversations")[1]
                assert len(conversations) == 1 and conversations[0]["model"] == CHAT
                # A timed-out native call must continue holding admission, avoiding
                # overlapping model loads after HTTP cancellation.
                assert call("/v1/chat/completions", {**request, "metadata": {"timeout_ms": 100}})[0] == 504
                assert call("/v1/chat/completions", request)[0] == 429
                print("PASS: auth, native status/catalog, chat/provenance, capability errors, 768D embeddings, web SSE/persistence, timeout admission")
            finally:
                process.terminate()
                try:
                    process.wait(timeout=60)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()


if __name__ == "__main__":
    main()
