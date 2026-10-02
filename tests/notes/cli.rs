use crate::scratch;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

/// Runs the `notes` binary on a device folder, feeding `stdin`.
pub fn run(device: &Path, args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_notes"))
        .arg("--device")
        .arg(device)
        .args(args)
        .env_remove("NOTES_DEVICE")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

/// Runs `notes` and returns its stdout, failing the test if it fails.
pub fn notes(device: &Path, args: &[&str]) -> String {
    let out = run(device, args, "");
    assert!(out.status.success(), "notes {args:?}: {}", stderr(&out));
    String::from_utf8(out.stdout).unwrap()
}

pub fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn cli_new_edit_list_show() {
    let laptop = scratch("cli_new_edit_list_show").join("laptop");
    assert_eq!(
        notes(
            &laptop,
            &["new", "Keynote outline", "--body", "Open with the demo."]
        ),
        "laptop-1\n"
    );
    assert_eq!(notes(&laptop, &["new", "Groceries"]), "laptop-2\n");

    let piped = run(
        &laptop,
        &["edit", "laptop-1", "--body", "-"],
        "Open with the story.\n",
    );
    assert!(piped.status.success(), "{}", stderr(&piped));
    notes(&laptop, &["edit", "laptop-2", "--title", "Shopping"]);

    assert_eq!(
        notes(&laptop, &["list"]),
        "laptop-1  Keynote outline  (edited just now)\nlaptop-2  Shopping  (edited just now)\n"
    );
    assert_eq!(
        notes(&laptop, &["show", "laptop-1"]),
        "Keynote outline\n\nOpen with the story.\n"
    );
}

#[test]
fn cli_reads_the_device_from_notes_device() {
    let phone = scratch("cli_reads_the_device_from_notes_device").join("phone");
    let out = Command::new(env!("CARGO_BIN_EXE_notes"))
        .args(["new", "Packing list"])
        .env("NOTES_DEVICE", &phone)
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "phone-1\n");
    assert_eq!(
        notes(&phone, &["list"]),
        "phone-1  Packing list  (edited just now)\n"
    );
}

#[test]
fn cli_reports_bad_input() {
    let laptop = scratch("cli_reports_bad_input").join("laptop");

    let missing = run(&laptop, &["show", "laptop-7"], "");
    assert!(!missing.status.success());
    assert_eq!(stderr(&missing), "notes: no note laptop-7\n");

    let unknown = run(&laptop, &["frobnicate"], "");
    assert!(!unknown.status.success());
    assert!(stderr(&unknown).starts_with("notes: unknown command frobnicate\nusage:"));

    let dangling = run(&laptop, &["new"], "");
    assert!(!dangling.status.success());
    assert!(stderr(&dangling).starts_with("notes: new needs a TITLE\nusage:"));
}
