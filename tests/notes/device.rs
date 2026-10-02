use crate::scratch;
use notes::{device, note};

#[test]
fn device_store_round_trips() {
    let dir = scratch("device_store_round_trips").join("laptop");
    let created = {
        let mut laptop = device::open(&dir).unwrap();
        let first = note::create(&mut laptop, "Keynote outline", "Open with the demo.").unwrap();
        let second = note::create(&mut laptop, "Groceries", "eggs\nmilk").unwrap();
        assert_eq!(
            (first.id.as_str(), second.id.as_str()),
            ("laptop-1", "laptop-2")
        );
        vec![first, second]
    };

    let laptop = device::open(&dir).unwrap();
    assert_eq!(note::all(&laptop), created);

    let doc = laptop.doc("laptop-2").unwrap();
    assert_eq!(doc.fields["title"], "Groceries");
    assert_eq!(doc.fields["body"], "eggs\nmilk");
}

#[test]
fn device_edit_keeps_the_id_and_replaces_fields() {
    let mut laptop = device::open(&scratch("device_edit").join("laptop")).unwrap();
    let created = note::create(&mut laptop, "Keynote outline", "v1").unwrap();

    let edited = note::edit(&mut laptop, &created.id, None, Some("v2")).unwrap();
    assert_eq!(edited.id, created.id);
    assert_eq!(
        (edited.title.as_str(), edited.body.as_str()),
        ("Keynote outline", "v2")
    );
    assert_eq!(note::get(&laptop, &created.id).unwrap(), edited);

    let next = note::create(&mut laptop, "Another", "").unwrap();
    assert_eq!(
        next.id, "laptop-3",
        "an id names the change that created the note"
    );

    assert!(note::edit(&mut laptop, "phone-9", Some("x"), None).is_err());
}
