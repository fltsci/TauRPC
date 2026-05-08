---
"@fltsci/taurpc": patch
---

Bump specta to 2.0.0-rc.25 (and specta-typescript / specta-serde to 0.0.12).

Rewrites `taurpc/src/export.rs` and the surrounding internals against specta's new visitor-based type-walking API (the rc.25 release removed the in-place `*_mut` accessors on `Struct`/`Enum`/`Tuple`/`Fields` and reshaped `Reference`/`NamedReference`). Generated TS bindings are unchanged at the user-facing level; the proxy and dispatcher pick up the new specta render shape in `src/index.ts` accordingly.

Rust consumers will need to bump their own `specta` dep to `=2.0.0-rc.25` in lockstep, since `=` exact-version pinning makes any rc skew a resolver collision.
