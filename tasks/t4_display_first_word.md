---
id: t4
title: Display the first word of a text document
status: Review
created: 2026-10-22
updated: 2026-10-22T17:04:11Z
---

## Goal

First static-word milestone using Rust + Slint.

## Acceptance criteria

- Load UTF-8 input.txt by default or one optional CLI file path.
- Split on whitespace, preserving attached punctuation.
- Include an original 200-word English sample.
- Center the first word in a resizable white window, black sans-serif at 48 px.
- Configure Windows, macOS, Linux X11 and Wayland support.
- Report unreadable or wordless files clearly.

## Scope

WPM, words per display and playback are deferred. User approved all choices.

## Checklist

- [x] Create sample, Rust project, and Slint view.
- [x] Build and exercise input and display behavior.
- [x] Review standards and acceptance criteria; record verification evidence.

## Progress

Rust toolchain installation underway; no prior compiler was installed.

Build discovery: Fontconfig runtime is installed but development metadata is not.
Configured upstream-supported RUST_FONTCONFIG_DLOPEN=1 in .cargo/config.toml.
Standards review completed independently: no findings.

The Fontconfig dynamic-loading experiment failed because fontique requires
statically exposed Fontconfig bindings. Removed that configuration. Using a
temporary extracted development package because sudo requires a password.
Independent Spec review: no source-level findings. Runtime checks pending.

## Verification environment

Linux x86_64; Rust 1.99.1; Slint 1.18.1; X11 display :0.
No Wayland socket or Windows/macOS execution environment is available.
Build dependencies were extracted under /tmp/fasteyes-build-deps/root.
Reproduce local build with:

```sh
RUST_MIN_STACK=33554432 PKG_CONFIG_PATH=/tmp/fasteyes-build-deps/root/usr/lib/x86_64-linux-gnu/pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/fasteyes-build-deps/root cargo build
```

On a normal Debian/Ubuntu development machine install pkg-config and
libfontconfig-dev, then use cargo build normally.

Source reviews (independent agents): Standards 0 findings; Spec 0 findings.
The repository has no existing commits, so new application files were
reviewed against the empty application baseline.

Runtime verification caught a fixed-size window: explicit width/height emitted
identical minimum and maximum dimensions. Replaced them with preferred-width
and preferred-height; rerunning resize verification.
Compiler stack overflow during Winit debug generation was resolved using
RUST_MIN_STACK=33554432. Build and Clippy passed before the sizing fix.
CLI smoke checks passed: missing/empty/whitespace/invalid UTF-8 input and extra
arguments all return nonzero with clear messages.

## Completion report

Implemented the static first-word viewer, optional file path, clear input errors,
and original 200-word sample. No playback or settings controls yet.

Final verification:
- cargo build passed using the local environment command above.
- cargo clippy -- -D warnings passed with the same environment variables.
- cargo fmt --check passed; wc -w input.txt reports 200.
- X11: default input displayed Morning; custom path containing spaces with a
  UTF-8 BOM and leading whitespace displayed Hello, with punctuation retained.
- Both windows initially measured 800x400 and resized to 1000x500 through
  the window manager; screenshots visually confirmed centered black sans-serif
  text on white after resizing.
- CLI: missing file, empty file, whitespace-only file, invalid UTF-8, and extra
  arguments returned nonzero with expected diagnostic messages.
- Final source review: sizing issue resolved; Standards 0 remaining findings,
  Spec 0 remaining findings.

Local screenshots: /tmp/fasteyes-first-word-resized.png and
/tmp/fasteyes-custom-word-resized.png (temporary evidence).

Run the built application from the repo root with ./target/debug/fasteyes.
Windows/macOS and Wayland are configured but were not executed here.
Awaiting user acceptance in Review.
