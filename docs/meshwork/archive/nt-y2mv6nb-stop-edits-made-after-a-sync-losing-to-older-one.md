---
id: nt-y2mv6nb
title: Stop edits made after a sync losing to older ones
status: done
category: sync
verify: run cargo test reported_gate_rewrite_survives
seq: 10
created: 2026-10-02T17:51Z
attachments: [attachments/nt-y2mv6nb/laptop-changes.log, attachments/nt-y2mv6nb/phone-changes.log]
---

A user's rewrite was lost. The report:

> At the gate I fixed a typo in a note on my laptop and let both devices sync. After takeoff I rewrote the note on my phone. When I landed and synced, my rewrite was gone and the laptop's version won.

The phone made its rewrite after it had synced the laptop's typo fix, so the rewrite should win on both devices. Both devices' change logs are attached.

Done when `reported_gate_rewrite_survives` passes. It replays the two logs on fresh device folders, with each device's clock reading what the log recorded and a sync wherever a log shows one, and expects both devices to end on the phone's rewrite.

## log
- 2026-10-02T17:51Z created
- 2026-10-02T17:51Z open→doing — claimed by claude (notes-1)
- 2026-10-02T17:51Z close attempt — verify failed (dsl)
- 2026-10-02T17:53Z doing→done — verify exit 0 @ 9273d6e+4

## comments
- 2026-10-02T17:51Z [claude (notes-1)] Smoking gun: notes sync pushes before it pulls, so the laptop's typo fix lands on top of the rewrite. Pulling first.
- 2026-10-02T17:51Z [claude (notes-1)] Wrong: notesync merges last-writer-wins on timestamps, so push/pull order cannot matter. The phone stored the laptop's typo fix before it wrote the rewrite, but the laptop's clock runs about five minutes fast, so the fix carries the later stamp and wins. Ordering is notesync's protocol, not ours.
