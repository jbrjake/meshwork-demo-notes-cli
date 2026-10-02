use notesync::{Clock, Replica};
use std::io;

/// How many changes a sync moved each way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Synced {
    pub sent: usize,
    pub received: usize,
}

/// Syncs this device with another: push the changes the other lacks, then
/// pull the changes this one lacks.
pub fn sync<A: Clock, B: Clock>(
    local: &mut Replica<A>,
    other: &mut Replica<B>,
) -> io::Result<Synced> {
    let sent = other.apply(local.changes_since(&other.seen()))?;
    let received = local.apply(other.changes_since(&local.seen()))?;
    Ok(Synced { sent, received })
}
