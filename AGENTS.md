# AGENTS.md — Rust + GPUI Color Picker

### Never edit my files, until i say so.

## Mission

Build a **lightweight, polished, reliable desktop color picker for Windows** using **Rust and GPUI**. Windows is the only implementation target for now. Organize the code so that macOS and Linux support can be added later, but **do not implement, emulate, or claim support for those platforms yet**.

This is a **learning-first project**. Codex is a mentor, reviewer, debugger, and implementation partner—not an autonomous app factory. Help the developer understand _why_ the code works, make meaningful architectural decisions, and gradually become able to maintain it independently.

### Priorities, in order

1. Correct pixel sampling and predictable user interaction.
2. Understandable, idiomatic Rust and GPUI code.
3. Reliability, good keyboard support, and Windows integration.
4. Small installed footprint, low idle CPU/RAM, and fast startup.
5. Refined visual design and smooth, restrained interactions.
6. Portability-friendly boundaries without premature abstractions.

Do not sacrifice correctness or maintainability merely to reach an arbitrary executable-size target.

## How to work with the developer

- **Honor the requested mode.**
  - **Explain / teach / review / plan:** do not modify files unless explicitly requested. Explain concepts using existing project code, small examples, and trade-offs.
  - **Implement / fix / refactor:** make the smallest complete, tested change that meets the request; do not stop at pseudocode.
  - **Guide me:** break the task into manageable steps, give the developer a chance to attempt meaningful work, and offer hints before a full solution when appropriate.
- For a substantial new feature, start with: **(1) what we are building, (2) the Rust/GPUI/Windows concepts involved, (3) a short plan, (4) how we will verify it**. Then implement if authorized.
- Explain _non-obvious_ details: ownership/borrowing, lifetimes where relevant, `Entity` and `Context`, events/actions, asynchronous work, rendering vs state, and the purpose of Windows API calls.
- Keep explanations proportional to the task. Do not produce lectures for trivial edits. Prefer plain English, then precise Rust terminology.
- Present real alternatives briefly when they matter, then recommend one and explain the trade-off.
- When debugging, identify the root cause and evidence rather than repeatedly trying random changes.
- At the end of a coding task, report **changed files, how it works, verification performed, what remains untested, and one useful takeaway**.
- If the developer asks for a complete solution, provide it. Never artificially withhold code in the name of teaching.
- Do not run destructive commands, overwrite unrelated work, update major dependencies, create commits, or push to remotes without explicit authorization.

## Tech stack and dependency policy

- **Language:** Rust, current compatible stable toolchain; use the MSVC target on Windows.
- **GUI:** upstream **GPUI** from `zed-industries/zed`, with `gpui_platform` when required by the verified version.
- **OS integration:** `windows` crate and narrowly scoped Win32 APIs when GPUI does not offer a reliable equivalent.
- **Persistence:** start with simple local configuration/history files only when needed; avoid adding a database.
- **Build:** Cargo; commit `Cargo.lock` for reproducible application builds.
- **Tests:** standard Rust unit/integration tests and GPUI test support where practical. Real screen, hotkey, tray, and DPI behavior require manual Windows validation.

**GPUI is pre-1.0 and changes frequently.** Before writing examples or modifying GPUI integration:

1. Inspect `Cargo.toml`, `Cargo.lock`, the installed crate version, and the actual code in this repo.
2. Consult the matching upstream GPUI README/examples/source if an API is uncertain.
3. Use **compatible pinned dependency versions or an intentional Git revision**—never wildcard `*` or an unpinned `main` for this project.
4. Never combine API examples from incompatible GPUI versions, invent methods, or assume code compiles.
5. Keep any dependency upgrade in a separate, explainable change unless required for the current task.

Windows environment may require Rust/MSVC build tools, the Windows SDK, CMake, and the shader compiler (`fxc.exe`) depending on the selected GPUI revision. **Verify requirements rather than blindly copying Zed's full development setup.** Prefer native Windows PowerShell/Developer PowerShell for Windows-specific build and interactive testing.

## Product scope

### Essential features (must work before calling the app complete)

- Global keyboard shortcut to enter picking mode; graceful handling if the shortcut is already taken.
- Select a visible desktop pixel with the mouse; **left-click confirms, Escape cancels**.
- Live color preview and a **pixel-accurate zoomed magnifier** with a clear center-pixel indicator.
- Copy selected color to clipboard; user-selectable formats **HEX, RGB, HSL**, with correctly documented formatting/rounding.
- Compact, readable GPUI interface showing preview, values, and copy feedback.
- Recent-color history; deduplicate consecutive identical selections and cap the list.
- System-tray behavior or an equivalently clear background lifecycle; explicit Quit action.
- Keyboard-accessible controls and sensible focus behavior.
- Correct operation on typical Windows multi-monitor arrangements and mixed DPI scaling.
- Errors are handled visibly where appropriate; no panics during ordinary user actions.

### Polish after the essentials

- Configurable shortcut and preferred output format.
- Local persistence for settings/history; predictable behavior if the settings file is missing or corrupted.
- Customizable copy formats (e.g. HSV or CSS notation) only after HEX/RGB/HSL are correct.
- Light/dark theme, subtle animations, copy toast, history management, and optional startup-on-login.
- Portable release packaging, app icon, version information, and a concise user README.

### Explicitly out of scope for the first release

- macOS/Linux implementations; cloud accounts/sync; analytics or telemetry; network services; an embedded browser; plugin architectures; auto-update frameworks; installer frameworks unless requested; advanced palette extraction; design-token export.
- Promising perfect HDR, wide-gamut, ICC color-profile, DRM-protected, or exclusive-fullscreen sampling without separate investigation and validation.

## Architecture principles

Start with a small crate and **add modules only as functionality arrives**. Suggested structure, not a mandatory empty scaffold:

```text
src/
  main.rs                 # application entry and startup
  app.rs                  # GPUI application lifecycle / shared state
  color.rs                # pure color types, parsing, conversions, formatting
  ui/
    mod.rs
    picker.rs             # primary view / controls
    magnifier.rs          # magnifier presentation
    history.rs            # recent selections
  platform/
    mod.rs                # narrow traits/interfaces as needed
    windows/
      mod.rs
      sampling.rs         # desktop pixel/region sampling
      hotkey.rs           # global shortcut registration/lifecycle
      tray.rs             # notification area integration
  settings.rs             # load/save when persistence is implemented
```

- Keep `color.rs` **pure and platform-independent**, with strong unit tests.
- Keep Win32 handles, event hooks, and unsafe code within narrowly scoped Windows modules. Document safety invariants at each `unsafe` boundary; release GDI resources, unregister hotkeys, and clean up tray icons.
- Separate **capturing a pixel/region**, **storing selected color**, and **rendering UI**.
- Use GPUI's standard state/event patterns; never block the UI thread with expensive capture, file operations, or busy loops.
- Favor simple explicit types over generic frameworks. Introduce traits when there are real consumers/test doubles or a concrete portability need.
- Avoid unnecessary dependencies, cloned state, excessive allocations, background polling, and rendering when nothing changed.
- Prefer event-driven updates. If magnifier updates require periodic capture, keep the sampled region small, throttle deliberately, and suspend the work when picking mode is inactive.

## Windows-specific correctness

- For an **initial sampling spike**, Win32 `GetCursorPos`, `GetDC` / `GetPixel`, and `ReleaseDC` are reasonable candidates. Validate results and errors (including `CLR_INVALID`), coordinate interpretation, and `COLORREF` byte ordering. This simple approach is not a guarantee for all desktop surfaces.
- For the **magnifier**, investigate efficient **region capture** rather than naively calling `GetPixel` hundreds of times every frame. Compare GDI bitmap capture with newer capture APIs if needed; choose based on measured correctness and cost.
- Prevent **self-sampling/feedback**: do not let the color preview, magnifier, or overlay become the pixel being picked. Test overlay positioning/visibility, capture timing, and click-through/input behavior.
- Handle physical vs logical coordinates carefully. Use the DPI behavior already provided by the selected GPUI version; prefer **Per-Monitor V2 DPI awareness** only after verifying how GPUI's Windows manifest/configuration sets it. Do not introduce conflicting DPI settings.
- Account for monitors left/above the primary screen (negative virtual-desktop coordinates), different scaling factors, different resolutions, and unplugged displays.
- Register the global shortcut using a supported API such as `RegisterHotKey`, and always unregister it. Do not steal reserved combinations or silently replace another application's shortcut.
- Keyboard/mouse hooks, window styles, focus, topmost behavior, taskbar visibility, and notification-area behavior must be tested on actual Windows; do not assume GPUI exposes every native capability.
- Treat color values as a clearly defined representation (typically 8-bit desktop RGB for the baseline). Explain limitations of color profiles and HDR instead of calling the result universally colorimetrically accurate.
- No elevation, administrator privileges, hidden input monitoring, or broad screen recording outside active picking unless demonstrably necessary and explicitly approved.

## Learning roadmap — complete one vertical slice at a time

Treat these as checkpoints, not authorization to implement the entire roadmap automatically.

| Stage                | Deliverable                                            | Learn                                                 | Acceptance gate                                           |
| -------------------- | ------------------------------------------------------ | ----------------------------------------------------- | --------------------------------------------------------- |
| 0. Environment       | Minimal GPUI window launches on Windows                | Cargo, MSVC, GPUI `Application`, `Entity`, `Render`   | `cargo run` works locally; version/toolchain recorded     |
| 1. Color domain      | RGB type, HEX/RGB/HSL formatting + tests               | structs, enums, conversions, `Result`, unit tests     | Black, white, primaries, boundary/rounding cases pass     |
| 2. Static UI         | Clean color preview and selectable copy format         | GPUI elements, state, actions, focus                  | UI renders and responds without capture                   |
| 3. Sampling          | Read pixel under cursor with a temporary test UI       | FFI boundaries, handles, Win32 GDI, coordinate spaces | Matches known test colors on real Windows desktop         |
| 4. Picking workflow  | Activate, track cursor, click to accept, Esc to cancel | state machines, windows, events, lifecycle            | Works from another application without focus bugs         |
| 5. Magnifier         | Smooth nearest-neighbor zoom and central pixel marker  | region capture, image buffers, performance            | No blur, off-by-one errors, or overlay self-capture       |
| 6. Clipboard/history | Copy formats and maintain recent colors                | serialization, state, data flow                       | Copies correct strings; history ordering/dedup works      |
| 7. Background use    | Global hotkey, tray, reopen/quit                       | native event loops, resource cleanup                  | Shortcut conflicts handled; Quit releases resources       |
| 8. Settings/polish   | Persistent preferences and accessible UI               | configuration, error handling, UX                     | Restart preserves valid settings; corrupt config recovers |
| 9. Release QA        | Portable release and documented limitations            | profiling, packaging, DPI/display testing             | Checklist passes and size/RAM/startup measured            |

**For each stage:** provide a concise explanation, the smallest running demonstration, tests/manual checks, and the next learning exercise. Do not move to the next stage until the current stage works or the developer explicitly redirects the work.

## Quality and verification

For code changes, run relevant checks when the environment supports them:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

For a release or performance change, also use `cargo build --release` and inspect the actual release artifact. If a command cannot run, state **exactly why** and what the developer should execute on Windows; do not claim success by inference. Do not silently add Clippy suppressions or disable tests just to make CI green.

Minimum manual Windows checklist as features become available:

- Picking from other apps; left-click confirm; Escape cancel; clipboard contents.
- 100%, 125%, 150%, 200% scaling; moving between differently scaled displays.
- Secondary monitor on the left or above the primary; edges/corners and taskbar.
- Hotkey registration failure, repeated activation, cancel/reopen, tray Quit.
- Sleep/resume, monitor disconnect, rapid pointer movement; GPU/display errors where feasible.
- Screenshots or diagnostics for reproduction, without collecting private screen contents or retaining captured images unexpectedly.
- Measure **release `.exe` size**, any bundled files/installer size, startup time, idle CPU, idle memory, and active-picker load. Record machine/configuration, method, and before/after figures. Do not compare debug build directories with installed-app footprint.

## Definition of done

A feature is done only when it has usable behavior, appropriate automated/manual verification, sound cleanup/error handling, and a brief explanation understandable to the developer. The **app** is done only when all essential features work on Windows, the manual checklist has been executed, known limitations are documented, and release measurements have been recorded. Never describe a mockup, UI-only control, or untested integration as working functionality.

## Starting behavior for Codex

If the repository is empty or no milestone is identified, **start with Stage 0 only**: check available tools, propose the smallest compatible pinned GPUI setup based on current upstream sources, explain the key GPUI concepts, and—only if the developer has asked you to implement—create a minimal native Windows window and verify the build. Do not generate the full application or a large directory tree at startup.

## References (consult only when relevant)

- Upstream GPUI README: https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md
- GPUI examples: https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/README.md
- GPUI crate manifest: https://github.com/zed-industries/zed/blob/main/crates/gpui/Cargo.toml
- Windows/Zed toolchain notes (not automatically a standalone-app checklist): https://github.com/zed-industries/zed/blob/main/docs/src/development/windows.md
- Microsoft GetPixel: https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getpixel
- Microsoft global hotkeys: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey
- Microsoft desktop DPI awareness: https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows
