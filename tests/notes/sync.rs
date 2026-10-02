use crate::cli::{notes, run, stderr};
use crate::scratch;

#[test]
fn sync_two_devices_converge() {
    let root = scratch("sync_two_devices_converge");
    let (laptop, phone) = (root.join("laptop"), root.join("phone"));
    let to = |dir: &std::path::Path| dir.to_str().unwrap().to_string();

    notes(&laptop, &["new", "Keynote outline", "--body", "v1"]);
    notes(&phone, &["new", "Packing list"]);
    assert_eq!(
        notes(&laptop, &["sync", &to(&phone)]),
        "sent 1, received 1\n"
    );
    assert_eq!(
        notes(&laptop, &["list"]),
        "laptop-1  Keynote outline  (edited just now)\nphone-1  Packing list  (edited just now)\n"
    );
    assert_eq!(notes(&phone, &["list"]), notes(&laptop, &["list"]));

    notes(&phone, &["edit", "laptop-1", "--body", "v2"]);
    assert_eq!(
        notes(&phone, &["sync", &to(&laptop)]),
        "sent 1, received 0\n"
    );
    assert_eq!(
        notes(&laptop, &["show", "laptop-1"]),
        "Keynote outline\n\nv2\n"
    );
    assert_eq!(
        notes(&laptop, &["sync", &to(&phone)]),
        "sent 0, received 0\n"
    );
}

#[test]
fn sync_two_devices_needs_an_existing_device() {
    let root = scratch("sync_two_devices_needs_an_existing_device");
    let missing = root.join("tablet");
    let out = run(
        &root.join("laptop"),
        &["sync", missing.to_str().unwrap()],
        "",
    );
    assert!(!out.status.success());
    assert_eq!(
        stderr(&out),
        format!("notes: no device folder at {}\n", missing.display())
    );
    assert!(!missing.exists(), "a typo must not create a device");
}
