---
id: nt-25af1n9
title: "Add new, edit, list and show"
status: open
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
