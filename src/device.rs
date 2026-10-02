use notesync::{Replica, SystemClock};
use std::io;
use std::path::Path;

/// Opens the device folder at `dir`, creating it if needed. The folder's
/// name is the device's id.
pub fn open(dir: &Path) -> io::Result<Replica<SystemClock>> {
    Replica::open(dir, SystemClock)
}
