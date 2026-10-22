# Fasteyes

A Rust + Slint speed reader. This first milestone displays the first word
of a UTF-8 file in a resizable white window, centered in black sans-serif
text at 48 px.

## Run

Install stable Rust from https://rustup.rs, then run from this directory:

```sh
cargo run
cargo run -- /path/to/document.txt
```

The default input is `input.txt` in the current working directory. It contains
an original 200-word English passage beginning with `Morning`.
Words are split on Unicode whitespace; attached punctuation is retained.
Missing files, invalid UTF-8, and wordless input produce terminal errors.
Quote paths containing spaces. Only one file argument is accepted.

## Platforms

Slint's Winit backend is enabled for Windows, macOS, Linux X11 and Wayland,
using the software renderer. Linux requires a working desktop session and
the runtime libraries for its display server, Fontconfig, and installed fonts.
Linux builds also need `pkg-config` and Fontconfig development files
(e.g. `libfontconfig-dev` on Debian/Ubuntu).

Platform support is configured; actual verification is recorded in
`tasks/t4_display_first_word.md`.
WPM, words per display, and playback are later milestones.
