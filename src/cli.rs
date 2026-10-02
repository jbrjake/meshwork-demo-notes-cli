//! The command line: parse the arguments, open the device folder, run one
//! command.

use crate::{device, note};
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub const USAGE: &str = "\
usage: notes [--device DIR] <command>
  new TITLE [--body TEXT|-]               create a note and print its id
  edit ID [--title TEXT] [--body TEXT|-]  replace a note's title or body
  list                                    list every note
  show ID                                 print a note
--body - reads the body from stdin. The device folder is --device, else
$NOTES_DEVICE, else ./device.";

#[derive(Debug)]
pub enum Error {
    /// The arguments were wrong; the caller prints the usage after it.
    Usage(String),
    Failed(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Failed(e.to_string())
    }
}

/// Runs one invocation. `args` excludes the program name.
pub fn run(args: &[String], stdin: &mut dyn Read, out: &mut dyn Write) -> Result<(), Error> {
    let mut args = args.iter().map(String::as_str);
    let mut command = args.next();
    let mut flag = None;
    if command == Some("--device") {
        flag = Some(PathBuf::from(value(&mut args, "--device")?));
        command = args.next();
    }
    let dir = device::dir(flag);
    match command {
        Some("new") => {
            let title = positional(&mut args, "new needs a TITLE")?;
            let options = options(&mut args, &["--body"])?;
            let body = text(options.get("--body"), stdin)?.unwrap_or_default();
            let created = note::create(&mut device::open(&dir)?, title, &body)?;
            writeln!(out, "{}", created.id)?;
        }
        Some("edit") => {
            let id = positional(&mut args, "edit needs a note ID")?;
            let options = options(&mut args, &["--title", "--body"])?;
            let body = text(options.get("--body"), stdin)?;
            let title = options.get("--title").copied();
            let edited = note::edit(&mut device::open(&dir)?, id, title, body.as_deref())?;
            writeln!(out, "{}", edited.id)?;
        }
        Some("list") => {
            options(&mut args, &[])?;
            for note in note::all(&device::open(&dir)?) {
                writeln!(out, "{}  {}", note.id, note.title)?;
            }
        }
        Some("show") => {
            let id = positional(&mut args, "show needs a note ID")?;
            options(&mut args, &[])?;
            let found = note::get(&device::open(&dir)?, id)
                .ok_or_else(|| Error::Failed(format!("no note {id}")))?;
            writeln!(out, "{}\n\n{}", found.title, found.body)?;
        }
        Some(other) => return Err(Error::Usage(format!("unknown command {other}"))),
        None => return Err(Error::Usage("no command".to_string())),
    }
    Ok(())
}

fn positional<'a>(
    args: &mut impl Iterator<Item = &'a str>,
    missing: &str,
) -> Result<&'a str, Error> {
    match args.next() {
        Some(arg) if !arg.starts_with("--") => Ok(arg),
        _ => Err(Error::Usage(missing.to_string())),
    }
}

fn value<'a>(args: &mut impl Iterator<Item = &'a str>, flag: &str) -> Result<&'a str, Error> {
    args.next()
        .ok_or_else(|| Error::Usage(format!("{flag} needs a value")))
}

/// Reads `--flag VALUE` pairs, allowing only `allowed` flags.
fn options<'a>(
    args: &mut impl Iterator<Item = &'a str>,
    allowed: &[&str],
) -> Result<BTreeMap<&'a str, &'a str>, Error> {
    let mut options = BTreeMap::new();
    while let Some(flag) = args.next() {
        if !allowed.contains(&flag) {
            return Err(Error::Usage(format!("unexpected {flag}")));
        }
        options.insert(flag, value(args, flag)?);
    }
    Ok(options)
}

/// A text option's value; `-` reads stdin, dropping one trailing newline.
fn text(value: Option<&&str>, stdin: &mut dyn Read) -> Result<Option<String>, Error> {
    match value {
        None => Ok(None),
        Some(&"-") => {
            let mut text = String::new();
            stdin.read_to_string(&mut text)?;
            if text.ends_with('\n') {
                text.pop();
            }
            Ok(Some(text))
        }
        Some(text) => Ok(Some(text.to_string())),
    }
}
