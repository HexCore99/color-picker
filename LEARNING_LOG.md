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

## Stage 2 — Static UI

- **Verified:** The GPUI header module renders `Color Picker` in the larger window (user screenshot).
- **Verified:** `.w_full()` makes the header span the content area (user screenshot).
- **Learned:** A row's `.flex()` lays out its own children; panels placed as separate children of the root `.flex_col()` stack vertically.
- **Verified:** The magnifier placeholder and color preview now render side by side; the swatch displays the stored orange color and HEX value (user screenshot).
- **Implemented:** The swatch and value text share an inner flex row, with HEX and RGB stacked to the swatch's right.
- **Verified:** `cargo check --locked`, `cargo test --locked` (2 tests), and Clippy with warnings denied pass. The updated layout still needs a visual check in `cargo run`.
- **Refactored:** `Picker::render()` delegates to `ui::view(&self.color)`; `src/ui/mod.rs` owns the root layout and `src/ui/body.rs` owns the two-panel body. The color is borrowed for rendering, so no extra `Rgb` copy is needed.
- **Verified:** The refactor passes `cargo check --locked`, `cargo test --locked` (2 tests), Clippy with warnings denied, and formatting checks for the edited Rust files. Visual behavior after the refactor still needs a `cargo run` check.
- **Verified:** The user screenshot confirms the refactored UI renders with the HEX heading and labeled RGB values beside the swatch.
- **Implemented:** HEX and a compact copy icon button now share a horizontal row. `src/ui/copy_button.rs` draws the overlapping-outline icon and owns the clipboard click handler; `body.rs` supplies the value to copy.
- **Checked:** The copy-icon change compiles, both color tests pass, and the edited UI files pass formatting. Strict Clippy flags the existing unused `Rgb::to_rgb`; repository-wide formatting flags existing blank lines in `windows_sampling.rs`. The new icon placement and clipboard result still need a Windows visual/paste check.
- **Implemented:** `src/ui/value_row.rs` renders a muted label next to a bordered field with an inset copy icon. RGB, HSL, and HSV share this component; `copy_button.rs` supplies both the boxed HEX button and the smaller inline button.
- **Learned:** Color calculations stay in `color.rs`. HSL/HSV round degrees and percentages to the nearest integer, use hue 0 for grayscale, and wrap a rounded hue of 360 to 0. The rows display channel values; their buttons copy `rgb(...)`, `hsl(...)`, and the app's non-CSS `hsv(...)` notation.
- **Checked:** All 6 color tests, `cargo check --locked`, and strict Clippy pass. Edited Rust files are formatted; the full format check still flags existing blank lines in `windows_sampling.rs`. Row alignment and all three clipboard buttons still need a Windows visual/paste check.

## GPUI interaction

- **Verified:** Click changes the color, preview, and displayed values to blue (user screenshots).
- **Verified:** Copy HEX writes the current color; clipboard contains `#FF8000` and `#0000FF` (user screenshots).
- **Workflow:** Code and explanations in chat; developer edits source files.

## Windows sampling

- **Verified:** `GetCursorPos` returned `(1034, 29)` during `cargo run` (user output).
- **Next:** Read that desktop pixel with `GetDC` / `GetPixel`, release the DC, and convert to `Rgb`.
