# Fasteyes

A Rust + Slint speed reader. Displays groups of 1–5 words
of a UTF-8 file in a resizable white window, centered in black sans-serif
text at 48 px.

## Build and run

Install stable Rust from https://rustup.rs, then run from this directory:

```sh
cargo build --release
cargo run --release
cargo run --release -- input.txt
cargo run --release -- "/path/to/document.txt"
```

The release executable is `target/release/fasteyes` (`fasteyes.exe` on Windows).
`input.txt` is an included sample document.

Without a file argument, the app starts blank. Use File → Open File… to open a
document, or supply a file path at startup. Play and the position slider are
disabled until a document is loaded; reading settings remain adjustable.
Words are split on Unicode whitespace; attached punctuation is retained.
Missing files, invalid UTF-8, and wordless input produce terminal errors at
startup. Errors from File → Open File… appear in the window and preserve the
current document. Opening the file picker pauses playback; successfully opening
a document resets the reading position and leaves playback paused.
Quote paths containing spaces. Only one file argument is accepted.

## Platforms

Slint's Winit backend is enabled for Windows, macOS, Linux X11 and Wayland,
using the software renderer. Linux requires a working desktop session and
the runtime libraries for its display server, Fontconfig, and installed fonts.
Linux builds also need `pkg-config` and Fontconfig development files
(e.g. `libfontconfig-dev` on Debian/Ubuntu).

Platform support is configured; actual verification is recorded in
[T4](tasks/t4_display_first_word.md). Linux X11 has been verified; Windows,
macOS, and Wayland have not been exercised.

## Playback

Select words per display (1–5, default 1) and WPM (60–1200, default 300).
The app starts paused. Click Play to read, Pause to hold the current group,
and Play to resume with a full display duration. After the final group,
Restart starts reading from the beginning.

Each group stays visible for `60000 × displayed words / WPM` milliseconds.
At 300 WPM, one word lasts 200 ms and three words last 600 ms. A shorter
final group uses its actual word count. Timer scheduling depends on the desktop
event loop; WPM is a target rate.

Changing WPM restarts the current group's timer while playing. Changing words
per display regroups from the current first word and restarts its timer while
playing. Settings never start paused playback. Words are joined with single
spaces and wrap when needed. Settings and reading position reset on app restart.

Click the reading area or focus Play, then press Left/Right to move one display
group backward/forward from the current position. Left stops at the first word;
Right stops when the current group includes the final word. Navigation never
wraps and does nothing without a document. While playing, navigation continues
with a fresh timer; while paused, it stays paused. Arrow keys retain their normal
behavior when a reading-setting control or the position slider has focus.

Drag the position slider to seek to a word. Seeking preserves the playback state
and starts a fresh timer when playing. File → Open File… also has a Ctrl+O
shortcut; File → Quit closes the application.

## Current limitations

Only UTF-8 plain-text documents are supported; PDF parsing is not implemented.
Documents are loaded into memory. Reading position and settings are not saved,
and the View, History, and Profiles menu items are placeholders.
