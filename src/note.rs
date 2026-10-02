use notesync::{Clock, Doc, Fields, Replica};
use std::io;

/// A note: a notesync document with a title and a body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    /// `<device>-<seq>` of the change that created the note.
    pub id: String,
    pub title: String,
    pub body: String,
}

impl Note {
    pub fn from_doc(doc: &Doc) -> Self {
        let field = |name: &str| doc.fields.get(name).cloned().unwrap_or_default();
        Self {
            id: doc.id.clone(),
            title: field("title"),
            body: field("body"),
        }
    }

    pub fn fields(&self) -> Fields {
        Fields::from([
            ("title".to_string(), self.title.clone()),
            ("body".to_string(), self.body.clone()),
        ])
    }
}

/// Creates a note on this device.
pub fn create<C: Clock>(device: &mut Replica<C>, title: &str, body: &str) -> io::Result<Note> {
    let seq = device.seen().get(device.device()) + 1;
    let note = Note {
        id: format!("{}-{seq}", device.device()),
        title: title.to_string(),
        body: body.to_string(),
    };
    device.put(&note.id, note.fields())?;
    Ok(note)
}

/// Replaces a note's title, body or both; a field left `None` keeps its value.
pub fn edit<C: Clock>(
    device: &mut Replica<C>,
    id: &str,
    title: Option<&str>,
    body: Option<&str>,
) -> io::Result<Note> {
    let mut note = get(device, id)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("no note {id}")))?;
    if let Some(title) = title {
        note.title = title.to_string();
    }
    if let Some(body) = body {
        note.body = body.to_string();
    }
    device.put(&note.id, note.fields())?;
    Ok(note)
}

pub fn get<C: Clock>(device: &Replica<C>, id: &str) -> Option<Note> {
    device.doc(id).map(Note::from_doc)
}

/// Every note, oldest first per device.
pub fn all<C: Clock>(device: &Replica<C>) -> Vec<Note> {
    let mut notes: Vec<Note> = device.docs().map(Note::from_doc).collect();
    notes.sort_by(|a, b| id_order(&a.id).cmp(&id_order(&b.id)));
    notes
}

/// Orders `laptop-2` before `laptop-10`.
fn id_order(id: &str) -> (&str, u64) {
    match id.rsplit_once('-') {
        Some((device, seq)) => (device, seq.parse().unwrap_or(u64::MAX)),
        None => (id, 0),
    }
}
