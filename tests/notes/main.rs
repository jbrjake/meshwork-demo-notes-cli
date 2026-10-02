//! The crate's one test target. Each topic is a module, so the suite links
//! into a single binary.

mod cli;
mod device;

use std::path::PathBuf;

/// An empty directory for one test, under cargo's per-target temp dir.
pub fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
