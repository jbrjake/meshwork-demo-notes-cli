---
id: nt-emb4j6k
title: Show when each note was last edited in list
status: done
category: cli
verify: run cargo test edited_label
docs:
  - meshwork-demo-notes-sync#docs/PROTOCOL.md#change-timestamps-sp-change-timestamp
created: 2026-10-02T17:26Z
covers:
  - ref: meshwork-demo-notes-sync#docs/PROTOCOL.md#sp-change-timestamp
    sha: 779afcefe12c7e24920e13f396c906e3ba3b479527ec3a3e72ba108bd34159f2
---

`list` shows when each note was last edited: "edited just now", "edited 3 minutes ago", then hours, then days.

The time is the winning change's timestamp. notesync's protocol defines that as the authoring device's wall-clock time when the change was made, so this task pins that clause: if its meaning changes, the label may no longer say when the note was edited.

A change from a device whose clock runs fast can carry a stamp ahead of this device's clock. That reads "edited in N minutes" rather than a negative age.

## log
- 2026-10-02T17:26Z created
- 2026-10-02T17:26Z cover meshwork-demo-notes-sync#docs/PROTOCOL.md#sp-change-timestamp @779afcefe12c
- 2026-10-02T17:26Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:27Z doing→done — verify exit 0 @ 2a4ef84+1
