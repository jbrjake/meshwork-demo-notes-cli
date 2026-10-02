---
id: nt-5cjbhqa
title: "Run fmt, clippy and tests in CI with actions pinned by SHA"
status: open
category: build
verify: exists .github/workflows/ci.yml
created: 2026-10-02T17:27Z
---

CI runs `scripts/gate.sh` on every push and pull request, the same fmt, clippy and test run as a local gate. The toolchain comes from `rust-toolchain.toml`. Actions are pinned by commit SHA with the version in a comment, never by tag, so a moved tag cannot change what runs.

## log
- 2026-10-02T17:27Z created
