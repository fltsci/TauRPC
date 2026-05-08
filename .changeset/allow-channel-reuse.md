---
"@fltsci/taurpc": patch
---

Allow `Channel`s to be reused across multiple TauRPC command invocations.

Previously, when a command argument contained a `Channel<T>` and the runtime ran a transform pass (e.g. semantic-types decoding), the generated TS proxy mutated the original channel's `onmessage` handler in place. Passing the same channel to a second command (or any code path that re-invoked the proxy) would re-wrap the already-wrapped handler, double-decoding payloads.

The proxy now wraps via `new Channel((response) => v.onmessage(transform(response)))` instead of mutating, so the original channel is untouched and safe to reuse.

Generated bindings now import `Channel` from `@tauri-apps/api/core` whenever the affected transform path is reachable.
