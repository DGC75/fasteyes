---
id: t19
title: Move Slint UI macro into src/ui.rs
status: Review
created: 2026-10-23
updated: 2026-10-23T04:53:15Z
---

## Goal and constraints

Move the entire Slint macro into `src/ui.rs`, preserving its contents and behavior. Keep document loading and callback wiring in `src/main.rs`. No new dependencies or asset path changes. Skip tests per prior instruction; compile and inspect the extraction.

## Acceptance criteria

- Main imports ReaderWindow from the ui module and compiles.
- The macro is unchanged in its new file.
- A dedicated Git commit allows reverting only the extraction.

## Progress

- [x] Read project guidance and current source.
- [x] Checkpoint the current reader and icons in commit 02b927f; unrelated changes remain untouched.
- [x] Extract the macro unchanged and compile.
- [x] Record evidence and prepare the extraction as a separate Git commit.

No dependencies or blockers.

## Verification and completion report

- The macro now lives in `src/ui.rs`. `src/main.rs` declares the module and imports ReaderWindow plus ComponentHandle, required for `run()` outside the macro's module.
- Compared against the checkpoint: macro and main function contents are preserved exactly. Relative icon paths remain valid because both Rust files share the same directory.
- `cargo check --offline` passed without warnings; `git diff --check` passed on 2026-10-23.
- Tests skipped per prior user instruction. GUI interaction was not exercised.
- The extraction commit includes only `src/main.rs`, `src/ui.rs`, and this task. Revert that commit to undo the extraction; unrelated working-tree files remain untouched.
