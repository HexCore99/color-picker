# Color Picker Learning Log

## Stage 0 — Minimal GPUI window

- **Done:** Pinned GPUI to `=0.2.2`; `cargo generate-lockfile` resolved GPUI 0.2.2.
- **Done:** The native window opened and rendered `Color Picker` (screenshot verified).
- **Added:** Explicit background and white text.

## Stage 1 — Color data

- **Done:** Added `Rgb` with three `u8` fields in `src/color.rs`.
- **Done:** `Picker` stores a color and displays HEX; user confirmed working. HEX formatting test passed (1 test).
- **Done:** Added RGB text formatting; `cargo test` passed both tests (user output).
- **Deferred:** Remaining color conversions/tests tracked in `TODO.md`.

## GPUI interaction

- **Verified:** Click changes the color, preview, and displayed values to blue (user screenshots).
- **Verified:** Copy HEX writes the current color; clipboard contains `#FF8000` and `#0000FF` (user screenshots).
- **Workflow:** Code and explanations in chat; developer edits source files.

## Windows sampling

- **Verified:** `GetCursorPos` returned `(1034, 29)` during `cargo run` (user output).
- **Next:** Read that desktop pixel with `GetDC` / `GetPixel`, release the DC, and convert to `Rgb`.
