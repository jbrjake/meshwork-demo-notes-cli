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
    sha: 2a99d54ec1c4ceca1c94dacf0dabe8d7e287de1e79c33dc20ab98b3309e566cd
parent: nt-y2mv6nb
---

`list` shows when each note was last edited: "edited just now", "edited 3 minutes ago", then hours, then days.

The time is the winning change's timestamp. notesync's protocol defines that as the authoring device's wall-clock time when the change was made, so this task pins that clause: if its meaning changes, the label may no longer say when the note was edited.

A change from a device whose clock runs fast can carry a stamp ahead of this device's clock. That reads "edited in N minutes" rather than a negative age.

## log
- 2026-10-02T17:26Z created
- 2026-10-02T17:26Z cover meshwork-demo-notes-sync#docs/PROTOCOL.md#sp-change-timestamp @779afcefe12c
- 2026-10-02T17:26Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:27Z doing→done — verify exit 0 @ 2a4ef84+1
- 2026-10-02T17:53Z done→open
- 2026-10-02T17:53Z open→doing — claimed by claude (notes-2)
- 2026-10-02T17:53Z cover meshwork-demo-notes-sync#docs/PROTOCOL.md#sp-change-timestamp @2a99d54ec1c4
- 2026-10-02T17:53Z doing→done — verify exit 0 @ 9273d6e+5

## comments
- 2026-10-02T17:53Z [claude (notes-2)] Reopened: on notesync v0.2.0 a change's timestamp is a hybrid logical clock reading that "can run ahead of any device's clock" (PROTOCOL.md, Change timestamps), and this label still reads it. After the phone stores the laptop's fast-stamped change, its own rewrite reads "edited in 5 minutes". The list should read `observed_at`, when this device stored the change.
