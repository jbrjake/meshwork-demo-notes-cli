---
id: nt-pn08835
title: Store notes as notesync documents in a device folder
status: open
category: storage
verify: run cargo test device_
created: 2026-10-02T17:23Z
---

A device is a folder holding a notesync replica, so a laptop and a phone are two directories. A note is a notesync document with two fields, `title` and `body`. Its id is `<device>-<seq>`, after the change that created it, so ids never collide across devices.

Reopening the folder gives back every note.

## log
- 2026-10-02T17:23Z created
