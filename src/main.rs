use slint::{
    ComponentHandle, Model,
    winit_030::{
        EventResult, WinitWindowAccessor,
        winit::{dpi::PhysicalPosition, event::WindowEvent},
    },
};
use std::{
    env,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

mod ui;

use ui::{AboutWindow, ReaderWindow};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let path = args.next().map(PathBuf::from);
    if args.next().is_some() {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "Usage: fasteyes [text-file]").into(),
        );
    }
    let words = match path {
        Some(path) => load_words(&path)?,
        None => Vec::new(),
    };
    let window = ReaderWindow::new()?;
    window.set_words(std::rc::Rc::new(slint::VecModel::from(words)).into());
    let about = AboutWindow::new()?;
    about.set_version(env!("CARGO_PKG_VERSION").into());
    let about_weak = about.as_weak();
    about.on_dismiss(move || {
        if let Some(about) = about_weak.upgrade() {
            let _ = about.hide();
        }
    });
    window.on_show_about(move || {
        let _ = about.show();
    });
    window.window().on_close_requested(|| {
        let _ = slint::quit_event_loop();
        slint::CloseRequestResponse::HideWindow
    });
    window.on_quit(|| {
        let _ = slint::quit_event_loop();
    });
    window.on_group_text(|words, start, count| {
        words
            .iter()
            .skip(start as usize)
            .take(count as usize)
            .map(|word| word.to_string())
            .collect::<Vec<_>>()
            .join(" ")
            .into()
    });
    let window_weak = window.as_weak();
    window.on_open_file(move || {
        let Some(window) = window_weak.upgrade() else {
            return;
        };
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Text documents", &["txt"])
            .add_filter("All files", &["*"])
            .pick_file()
        else {
            return;
        };
        match load_words(&path) {
            Ok(words) => {
                window.invoke_reset_document();
                window.set_words(std::rc::Rc::new(slint::VecModel::from(words)).into());
                window.set_load_error("".into());
            }
            Err(error) => window.set_load_error(error.to_string().into()),
        }
    });
    let mut startup_done = false;
    window.window().on_winit_window_event(move |window, event| {
        if !startup_done && matches!(event, WindowEvent::RedrawRequested) {
            startup_done = true;
            window.with_winit_window(|native| {
                if let Some(monitor) = native
                    .current_monitor()
                    .or_else(|| native.primary_monitor())
                {
                    let origin = monitor.position();
                    let screen = monitor.size();
                    let size = native.outer_size();
                    native.set_outer_position(PhysicalPosition::new(
                        origin.x + (screen.width as i32 - size.width as i32) / 2,
                        origin.y + (screen.height as i32 - size.height as i32) / 2,
                    ));
                }
                native.focus_window();
            });
        }
        EventResult::Propagate
    });
    window.run()?;
    Ok(())
}

fn load_words(path: &Path) -> io::Result<Vec<slint::SharedString>> {
    let text = fs::read_to_string(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("Could not read {}: {error}", path.display()),
        )
    })?;
    // A UTF-8 byte-order mark is an encoding marker, not part of the first word.
    let words: Vec<slint::SharedString> = text
        .trim_start_matches('\u{feff}')
        .split_whitespace()
        .map(slint::SharedString::from)
        .collect();
    if words.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} contains no words", path.display()),
        ));
    }
    i32::try_from(words.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Document contains too many words",
        )
    })?;
    Ok(words)
}
