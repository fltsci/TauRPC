---
"@fltsci/taurpc": patch
---

Pin Rust toolchain to 1.95.0 via `rust-toolchain.toml`.

dprint's exec plugin shells out to `rustfmt --edition 2024` for `.rs` files, but neither CI (`ubuntu-latest`) nor contributor environments had a pinned `rustfmt` version. Different rustfmt builds produce different layouts for `if let` / let-chain bodies even with the same `--edition` flag, which surfaced as "works locally, fails CI" formatter drift. Pinning makes CI deterministic.

Components: `rustfmt`, `clippy`. Contributors with `rustup` will auto-fetch 1.95.0 on first use; no extra setup needed.
