use std::{env, error::Error, fs, io, path::PathBuf};

slint::slint! {
    export component ReaderWindow inherits Window {
        in property <string> word;
        title: "Fasteyes";
        preferred-width: 800px;
        preferred-height: 400px;
        background: white;
        Text {
            width: parent.width;
            height: parent.height;
            text: root.word;
            color: black;
            font-family: "sans-serif";
            font-size: 48px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}

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
    let word = text
        .trim_start_matches('\u{feff}')
        .split_whitespace()
        .next()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{} contains no words", path.display()),
            )
        })?;
    let window = ReaderWindow::new()?;
    window.set_word(word.into());
    window.run()?;
    Ok(())
}
