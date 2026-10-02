# notes

> **This is a demo repo.** It is one of three that show [meshwork](https://github.com/jbrjake/meshwork) tracking work across projects: this app, the [replication library](https://github.com/jbrjake/meshwork-demo-notes-sync) it syncs through, and the [portfolio](https://github.com/jbrjake/meshwork-demo-notes-portfolio) that registers both. The agent sessions in the history after the `story/0-day0` tag are staged: a script performs them with real meshwork commands, real code changes and real outputs. To replay them, clone the portfolio repo and run `story/replay.sh`; it needs git, cargo and the network.

Notes on the command line, synced between devices. A device is a folder, so a laptop and a phone are two directories, and syncing them trades the changes each one lacks through [notesync](https://github.com/jbrjake/meshwork-demo-notes-sync).

## Use

```
$ notes --device laptop new "Keynote outline" --body "Open with the demo."
laptop-1
$ notes --device phone new "Packing list"
phone-1
$ notes --device laptop sync phone
sent 1, received 1
$ notes --device phone list
laptop-1  Keynote outline  (edited just now)
phone-1  Packing list  (edited just now)
$ echo "Open with the story." | notes --device phone edit laptop-1 --body -
laptop-1
$ notes --device phone sync laptop
sent 1, received 0
$ notes --device laptop show laptop-1
Keynote outline

Open with the story.
```

| Command | Does |
|---|---|
| `new TITLE [--body TEXT\|-]` | creates a note and prints its id |
| `edit ID [--title TEXT] [--body TEXT\|-]` | replaces a note's title or body |
| `list` | lists every note and when it was last edited |
| `show ID` | prints a note |
| `sync OTHER_DIR` | trades changes with another device folder |

The device folder is `--device`, else `$NOTES_DEVICE`, else `./device`. `--body -` reads the body from stdin. A note's id is the device that created it and that device's change number.

When two devices edit the same note, the later edit wins, by the editing device's clock; notesync's [protocol](https://github.com/jbrjake/meshwork-demo-notes-sync/blob/main/docs/PROTOCOL.md) has the details.

## Build

```
cargo build --release    # target/release/notes
```

`scripts/gate.sh` runs formatting, lints and the tests; CI runs the same script. Work is tracked with [meshwork](https://github.com/jbrjake/meshwork) in `docs/meshwork/`.
