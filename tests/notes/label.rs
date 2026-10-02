use crate::scratch;
use notes::label::edited_ago;
use notes::{cli, note, sync::sync};
use notesync::{Clock, ManualClock, Replica};

const MINUTE: u64 = 60_000;
const NOW: u64 = 1_790_000_000_000;

#[test]
fn edited_label_counts_minutes() {
    assert_eq!(edited_ago(NOW, NOW), "edited just now");
    assert_eq!(edited_ago(NOW, NOW - 59_999), "edited just now");
    assert_eq!(edited_ago(NOW, NOW - MINUTE), "edited 1 minute ago");
    assert_eq!(
        edited_ago(NOW, NOW - 5 * MINUTE - 30_000),
        "edited 5 minutes ago"
    );
    assert_eq!(edited_ago(NOW, NOW - 59 * MINUTE), "edited 59 minutes ago");
}

#[test]
fn edited_label_hours_and_days() {
    assert_eq!(edited_ago(NOW, NOW - 60 * MINUTE), "edited 1 hour ago");
    assert_eq!(edited_ago(NOW, NOW - 200 * MINUTE), "edited 3 hours ago");
    assert_eq!(edited_ago(NOW, NOW - 24 * 60 * MINUTE), "edited 1 day ago");
    assert_eq!(
        edited_ago(NOW, NOW - 12 * 24 * 60 * MINUTE),
        "edited 12 days ago"
    );
}

#[test]
fn edited_label_reads_a_fast_clock_as_ahead() {
    assert_eq!(edited_ago(NOW, NOW + 20_000), "edited just now");
    assert_eq!(edited_ago(NOW, NOW + 5 * MINUTE), "edited in 5 minutes");
    assert_eq!(edited_ago(NOW, NOW + 90 * MINUTE), "edited in 1 hour");
}

#[test]
fn edited_label_shows_in_list() {
    let clock = ManualClock::new(NOW);
    let mut laptop = Replica::open(
        &scratch("edited_label_shows_in_list").join("laptop"),
        &clock,
    )
    .unwrap();
    note::create(&mut laptop, "Keynote outline", "v1").unwrap();
    clock.advance(3 * MINUTE);
    note::create(&mut laptop, "Groceries", "eggs").unwrap();
    clock.advance(MINUTE);

    assert_eq!(
        cli::list(&laptop, clock.now_ms()),
        [
            "laptop-1  Keynote outline  (edited 4 minutes ago)",
            "laptop-2  Groceries  (edited 1 minute ago)",
        ]
    );
}

/// The phone stores the laptop's change, stamped by a clock five minutes
/// fast, then edits the note itself. Its own edit happened just now.
#[test]
fn edited_label_never_reads_ahead() {
    let laptop_clock = ManualClock::new(NOW + 5 * MINUTE);
    let phone_clock = ManualClock::new(NOW);
    let root = scratch("edited_label_never_reads_ahead");
    let mut laptop = Replica::open(&root.join("laptop"), &laptop_clock).unwrap();
    let mut phone = Replica::open(&root.join("phone"), &phone_clock).unwrap();

    let created = note::create(&mut laptop, "Keynote outline", "v1").unwrap();
    sync(&mut phone, &mut laptop).unwrap();
    note::edit(&mut phone, &created.id, None, Some("v2")).unwrap();

    for line in cli::list(&phone, phone_clock.now_ms()) {
        assert!(!line.contains("edited in"), "{line}");
    }
}
