# mj-llm source snapshot

Source: https://github.com/clevekim00/mj-llm (MIT; LICENSE included).
Snapshot: 2026-10-08 working tree on base b764eeb. The N0 implementation was
uncommitted in the source project. SOURCE.json records original file hashes;
it is not a claim that the base commit contains these files.

Only app-core, runtime-litert and the artifact lock are included. This keeps a
standalone clone buildable without an unpublished sibling path dependency.
SDK binaries and model weights are excluded. Do not update the snapshot silently.

Hub extensions: generation.rs/native.rs/bridge.cc preserve system and history
roles with the pinned SDK conversation config and accept an output cap of 1–32.
SDK thinking is disabled and the Qwen3 /no_think control from the N0 probe is
appended to the last user message during model preprocessing only.
Context remains 512 tokens; aggregate message content is capped at 1024 bytes.
Workspace membership excludes n0-probe. Formatting may differ from the source.
The original mj-llm checkout was not edited.
