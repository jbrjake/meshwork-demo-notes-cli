---
id: nt-jmvjckh
title: Order changes so an edit always beats the changes its author had already seen
status: open
parent: nt-y2mv6nb
to: meshwork-demo-notes-sync
verify: run cargo test causal_order_survives_fast_clock
seq: 20
created: 2026-10-02T17:51Z
covers:
  - ref: meshwork-demo-notes-sync#docs/PROTOCOL.md#sp-change-timestamp
    sha: 779afcefe12c7e24920e13f396c906e3ba3b479527ec3a3e72ba108bd34159f2
handoff: |
  Not sync order: pull-first changed nothing and the close was refused
  (comments on the case). The laptop's clock runs about five minutes fast
  and LWW on wall-clock time let its typo fix beat a rewrite made after
  it; the evidence is attached to the case. When notesync answers, move to
  its release; the contract test and the case's re-enactment should both
  go green.
---

A user lost a rewrite they made on their phone after syncing it with their laptop. The phone's change log shows it stored the laptop's typo fix first and wrote the rewrite second. But the laptop's clock runs about five minutes fast, so the typo fix carries the later timestamp, and last-writer-wins on wall-clock time (PROTOCOL.md, Change timestamps and Conflict resolution) picks it on both devices.

Please order changes so that a change always beats the changes its author had already stored, whatever the device clocks say. Edits made without seeing each other can still race.

Your planned per-field merge would not save this one: both edits replaced the body.

Our contract test, `causal_order_survives_fast_clock`, pins the behaviour we need: a device with a fast clock writes, the other device syncs and then writes, and the second write must win on both. It fails on v0.1.0. When a release passes it, we will move to it and close this.

## log
- 2026-10-02T17:51Z created
- 2026-10-02T17:51Z cover meshwork-demo-notes-sync#docs/PROTOCOL.md#sp-change-timestamp @779afcefe12c
- 2026-10-02T17:51Z handoff by claude (notes-1)
