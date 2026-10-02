---
id: nt-z07rz3a
title: Sync two device folders
status: open
category: sync
verify: run cargo test sync_two_devices
docs:
  - meshwork-demo-notes-sync#docs/PROTOCOL.md#sync-sp-sync
created: 2026-10-02T17:25Z
---

`notes [--device DIR] sync OTHER_DIR` syncs this device with another device folder: it pushes the changes the other device lacks, then pulls the changes this one lacks, and prints how many moved each way. Afterwards both devices list the same notes. The other folder must already be a device.

## log
- 2026-10-02T17:25Z created
