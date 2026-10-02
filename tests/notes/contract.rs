//! What notes needs from notesync's ordering.

use crate::scratch;
use notes::{note, sync::sync};
use notesync::{ManualClock, Replica};

const MINUTE: u64 = 60_000;
const T: u64 = 1_790_000_000_000;

/// An edit beats the changes its author had already seen, even when the
/// other device's clock runs fast.
#[test]
#[ignore = "waits on nt-jmvjckh"]
fn causal_order_survives_fast_clock() {
    let fast = ManualClock::new(T + 5 * MINUTE);
    let truth = ManualClock::new(T);
    let root = scratch("causal_order_survives_fast_clock");
    let mut laptop = Replica::open(&root.join("laptop"), &fast).unwrap();
    let mut phone = Replica::open(&root.join("phone"), &truth).unwrap();

    let first = note::create(&mut laptop, "Keynote outline", "typo fixed").unwrap();
    sync(&mut phone, &mut laptop).unwrap();
    fast.advance(MINUTE);
    truth.advance(MINUTE);
    note::edit(&mut phone, &first.id, None, Some("rewritten after the fix")).unwrap();
    sync(&mut phone, &mut laptop).unwrap();

    for (device, replica) in [("laptop", &laptop), ("phone", &phone)] {
        let body = note::get(replica, &first.id).unwrap().body;
        assert_eq!(body, "rewritten after the fix", "on the {device}");
    }
}
