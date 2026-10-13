# Learn GPUI

## Rust stuffs

- `#[cfg(test)]` the `#[...]` is `attribute syntax`. it gives extra information to the compiler about the item immediately after it.
- `#[cfg(test)]` cfg means `configuration condition` it tells rust to include an item when condition is true. in this case `test`

## GPUI stuffss

```rust
impl Render for Picker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Color Picker")
    }

}
```

- `Picker` struct implements `Render` trait
- `render` draws the UI
- `self` borrows `Picker`
- `window`
- `cx`
- `-> impl IntoElement` a return type. meaning this function is expecting a element which implement `IntoElement`

- `Picker` implements the `Render` trait.
- `render` builds and returns the UI for GPUI to display.
- `&mut self` is a mutable borrow of this `Picker` instance.
- `window` is a mutable reference to the GPUI window being rendered.
- `cx` is a mutable GPUI context for `Picker` (`Self` means `Picker` here). It allows track the interaction with the ui.
- `-> impl IntoElement` says the function returns a concrete type that implements GPUI’s `IntoElement` trait.

##

```rust
 Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(360.0), px(240.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Picker),
        )
        .expect("failed to poen the window");
        cx.activate(true);
    })
```

- `Application::new()` creates the GPUI application.
- `.run(|cx:&mut App|{})` starts the event loop and calls the closure.
- `Bounds::centered(....)` sets the window in center with given sizes.
- `..Default::default()` fills the other `WindowOptions` with the default values.
- `|_, cx| cx.new(|_| Picker),` tells what to show in the window.

##

```rust

                    .on_click(cx.listener(|picker, _event, _window, cx| {
                        picker.color = Rgb { r: 0, g: 0, b: 255 };
                        cx.notify();
                    })),
```

- `.on_click(...)` registers a callback on this `Div`. GPUI runs the callback when the `Div` receives a click.
- `cx.listener(...)` wraps our closure so it can access the `Picker` view's state. It is created while `render` builds the UI; the closure itself runs later, when the click happens.
- `picker` borrows the `Picker` mutably.
- `event` `window` `cx` is passed by `the on_click()`. And i dont know what i can do with these right now.
