---
id: nt-25af1n9
title: "Add new, edit, list and show"
status: done
category: cli
verify: run cargo test cli_
created: 2026-10-02T17:24Z
---

The command line over a device folder:

```
notes [--device DIR] new TITLE [--body TEXT|-]
notes [--device DIR] edit ID [--title TEXT] [--body TEXT|-]
notes [--device DIR] list
notes [--device DIR] show ID
```

The device folder is `--device`, else `$NOTES_DEVICE`, else `./device`. `--body -` reads the body from stdin. `new` prints the note's id; `list` prints one note per line.

## log
- 2026-10-02T17:24Z created
- 2026-10-02T17:24Z open→doing — claimed by claude (602c381b-d7db-491e-8df6-85682e6152ed)
- 2026-10-02T17:25Z doing→done — verify exit 0 @ cd3beca+1
