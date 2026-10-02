use notesync::{Replica, SystemClock};
use std::io;
use std::path::{Path, PathBuf};

/// The device folder: the `--device` flag, else `$NOTES_DEVICE`, else
/// `./device`.
pub fn dir(flag: Option<PathBuf>) -> PathBuf {
    flag.or_else(|| std::env::var_os("NOTES_DEVICE").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("device"))
}

/// Opens the device folder at `dir`, creating it if needed. The folder's
/// name is the device's id.
pub fn open(dir: &Path) -> io::Result<Replica<SystemClock>> {
    Replica::open(dir, SystemClock)
}
