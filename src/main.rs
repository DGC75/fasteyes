use std::{env, error::Error, fs, io, path::PathBuf};

slint::slint! {
    import { Button, Slider, SpinBox } from "std-widgets.slint";
    export component ReaderWindow inherits Window {
        in property <int> word-count;
        pure callback group-text(int, int) -> string;
        private property <int> position: 0;
        changed position => { navigation.value = root.position; }
        private property <bool> playing: false;
        private property <bool> finished: false;
        private property <int> displayed-count: min(words-per-display.value, root.word-count - root.position);

        playback := Timer {
            interval: 60000ms * root.displayed-count / wpm.value;
            running: root.playing;
            triggered => {
                if root.position + root.displayed-count >= root.word-count {
                    root.playing = false;
                    root.finished = true;
                } else {
                    root.position += root.displayed-count;
                }
            }
        }
        title: "Fasteyes";
        preferred-width: 800px;
        preferred-height: 400px;
        background: white;
        VerticalLayout {
            spacing: 0px;
            Text {
                vertical-stretch: 1;
                text: root.group-text(root.position, root.displayed-count);
                wrap: word-wrap;
                color: black;
                font-family: "sans-serif";
                font-size: 48px;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
            navigation := Slider {
                accessible-label: "Reading position";
                minimum: 0;
                maximum: root.word-count - 1;
                step: 1;
                enabled: root.word-count > 1;
                changed(value) => {
                    root.position = round(value);
                    root.finished = false;
                    if root.playing { playback.restart(); }
                }
            }
            HorizontalLayout {
                vertical-stretch: 0;
                padding: 16px;
                spacing: 12px;
                HorizontalLayout {
                    horizontal-stretch: 1;
                    min-width: 0px;
                    preferred-width: 0px;
                    alignment: center;
                    spacing: 12px;
                    Text {
                        text: "Words per display";
                        color: black;
                        vertical-alignment: center;
                    }
                    words-per-display := SpinBox {
                        accessible-label: "Words per display";
                        minimum: 1;
                        maximum: 5;
                        value: 1;
                        step-size: 1;
                        changed value => { if root.playing { playback.restart(); } }
                    }
                }
                Button {
                    width: 48px;
                    height: 48px;
                    accessible-label: root.finished ? "Restart" : (root.playing ? "Pause" : "Play");
                    icon: root.finished ? @image-url("../assets/restart.svg")
                        : (root.playing ? @image-url("../assets/pause.svg") : @image-url("../assets/play.svg"));
                    clicked => {
                        if root.finished {
                            root.position = 0;
                            root.finished = false;
                        }
                        root.playing = !root.playing;
                    }
                }
                HorizontalLayout {
                    horizontal-stretch: 1;
                    min-width: 0px;
                    preferred-width: 0px;
                    alignment: center;
                    spacing: 12px;
                    Text {
                        text: "WPM";
                        color: black;
                        vertical-alignment: center;
                    }
                    wpm := SpinBox {
                        accessible-label: "WPM";
                        minimum: 60;
                        maximum: 1200;
                        value: 300;
                        step-size: 1;
                        changed value => { if root.playing { playback.restart(); } }
                    }
                }
            }
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
