---
"@fltsci/taurpc": patch
---

Add Rust toolchain setup step to the Release workflow.

#26 pinned the Rust toolchain via `rust-toolchain.toml` and added a `dtolnay/rust-toolchain` setup step to `.github/workflows/main.yml` (CI). The Release workflow's "Sync formatting if necessary" step also runs `pnpm format` -> `dprint fmt` -> shells to `rustfmt`, but had no setup step. With no pre-installed 1.95.0 toolchain on the runner, dprint's parallel rustfmt invocations all triggered concurrent rustup auto-installs that raced on the `~/.rustup/downloads/*.partial` rename, surfacing as "component download failed: No such file or directory" errors.

Mirroring the CI step into the Release workflow installs the toolchain serially before the parallel formatter runs.
