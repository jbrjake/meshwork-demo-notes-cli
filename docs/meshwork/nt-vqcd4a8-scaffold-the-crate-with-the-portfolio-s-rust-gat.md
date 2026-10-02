---
id: nt-vqcd4a8
title: Scaffold the crate with the portfolio's Rust gate scaffold
status: doing
category: build
verify: "all(contains Cargo.toml /^\\[profile\\.dev\\]/, exists .cargo/config.toml, exists rust-toolchain.toml, exists tests/notes/main.rs)"
created: 2026-10-02T17:22Z
claimed-by: claude (602c381b-d7db-491e-8df6-85682e6152ed)
---

A std-only crate, `meshwork-demo-notes-cli`: the library `notes` and the binary `notes`, on Rust 1.97.0. It depends on notesync by git tag, the way a separate project consumes it, and commits `Cargo.lock`.

The scaffold goes in with the first code:
- `[profile.dev]` keeps line tables only, and no debug info for dependencies.
- `-D warnings` lives in `.cargo/config.toml`, so the gate, a plain `cargo test` and the editor share one build.
- One test target, `tests/notes/main.rs`, with each topic a module.
- `scripts/gate.sh` runs fmt, clippy and the tests, and marks `target/` so Spotlight skips it.

## log
- 2026-10-02T17:22Z created
- 2026-10-02T17:22Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
