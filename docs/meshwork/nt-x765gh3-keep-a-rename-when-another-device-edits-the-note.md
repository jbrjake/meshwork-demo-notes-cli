---
id: nt-x765gh3
title: Keep a rename when another device edits the note's body at the same time
category: sync
seq: 30
needs: [meshwork-demo-notes-sync#sy-cycv60g]
verify: run cargo test concurrent_title_edits
status: open
created: 2026-10-02T17:28Z
---
If the laptop renames a note while the phone, without having seen the rename, edits its body, one edit is lost after they sync. A notesync change carries the whole document, so the winning change brings its stale copy of the other field with it.

This waits on notesync's per-field merge, which resolves each field on its own. Once it ships, move to that release and pin it with a test: rename on one device, edit the body on the other, sync, and both edits survive on both devices.

## log
- 2026-10-02T17:28Z created
