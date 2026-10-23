use slint::ComponentHandle;
use std::{env, error::Error, fs, io, path::PathBuf};

mod ui;

use ui::ReaderWindow;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| "input.txt".into());
    if args.next().is_some() {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "Usage: fasteyes [text-file]").into(),
        );
    }
    let text = fs::read_to_string(&path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("Could not read {}: {error}", path.display()),
        )
    })?;
    // A UTF-8 byte-order mark is an encoding marker, not part of the first word.
    let words: Vec<String> = text
        .trim_start_matches('\u{feff}')
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    if words.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} contains no words", path.display()),
        )
        .into());
    }
    let window = ReaderWindow::new()?;
    window.set_word_count(words.len().try_into()?);
    window.on_group_text(move |start, count| {
        words[start as usize..(start + count) as usize]
            .join(" ")
            .into()
    });
    window.run()?;
    Ok(())
}
