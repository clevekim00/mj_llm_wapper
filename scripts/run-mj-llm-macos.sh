#!/bin/bash
# Build and run the embedded backend; no Ollama or Python service is started.
set -euo pipefail
if [[ "$(uname -s)" != Darwin ]]; then
  printf '%s\n' 'The mj-llm native adapter currently requires macOS.' >&2
  exit 2
fi
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
if [[ $# -gt 0 && -d "$1" ]]; then
  export MJ_LITERT_SDK_DIR="$(cd "$1" && pwd)"
  shift
else
  export MJ_LITERT_SDK_DIR="${MJ_LITERT_SDK_DIR:-$repo_root/.mj-local-llm-hub/sdk/CLiteRTLM_mac.xcframework/macos-arm64_x86_64}"
fi
if [[ ! -f "$MJ_LITERT_SDK_DIR/libCLiteRTLM_mac.dylib" ]]; then
  printf '%s\n' 'SDK not found. See docs/mj-llm-runtime.md or pass SDK_SLICE_DIR as the first argument.' >&2
  exit 2
fi
export DYLD_LIBRARY_PATH="$MJ_LITERT_SDK_DIR"
export MJ_HUB_RUNTIME=mj-llm
cd "$repo_root"
cargo build --release --features native-macos --locked
exec target/release/mj-local-llm-hub "$@"
