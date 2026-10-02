---
id: nt-pn08835
title: Store notes as notesync documents in a device folder
status: done
category: storage
verify: run cargo test device_
created: 2026-10-02T17:23Z
---

A device is a folder holding a notesync replica, so a laptop and a phone are two directories. A note is a notesync document with two fields, `title` and `body`. Its id is `<device>-<seq>`, after the change that created it, so ids never collide across devices.

Reopening the folder gives back every note.

## log
- 2026-10-02T17:23Z created
- 2026-10-02T17:23Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:23Z doing→done — verify exit 0 @ dd36053+1
