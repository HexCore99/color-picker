# Learn GPUI

## Rust stuffs

- `#[cfg(test)]` the `#[...]` is `attribute syntax`. it gives extra information to the compiler about the item immediately after it.
- `#[cfg(test)]` cfg means `configuration condition` it tells rust to include an item when condition is true. in this case `test`
- `let value = pixel.0;` this weird shit is possible , because pixel is holding a `tuple struct`
- `tuple struct` a struct without variable names, just the types. e.g `pub struct Point(i32,i32)`
- `Cow` (Clone on Write) can hold borrowed or owned data. `to_mut()` clones borrowed data only when you need to change it; for example, `Cow::Borrowed(include_bytes!(...))` can pass bundled SVG bytes without making another copy.

### `AtomicBool`

`AtomicBool` stores a boolean (`true` or `false`) that can be safely read and changed across threads.

```rust
static PICKING: AtomicBool = AtomicBool::new(false);
```

- `static` — one shared value that exists for the entire program.
- `AtomicBool::new(false)` — starts the flag as `false`: picking is inactive.
- **Atomic** means each operation happens as one indivisible step.

```rust
PICKING.store(true, Ordering::Relaxed); // Set the flag to true.
let active = PICKING.load(Ordering::Relaxed); // Read the flag.
let was_picking = PICKING.swap(false, Ordering::Relaxed); // Read the old value and set false together.
```

`Ordering::Relaxed` keeps the operation on this flag atomic without coordinating access to other data in memory.

### `Mutex`

`Mutex` means **mutual exclusion**: it protects shared data by allowing only one lock holder at a time. Think of a room with one key.

```rust
static PICKED_COLOR: Mutex<Option<Rgb>> = Mutex::new(None);

if let Ok(mut slot) = PICKED_COLOR.lock() {
    *slot = Some(color);
}
```

- `Option<Rgb>` — `None` means no color is waiting; `Some(color)` holds a sampled color.
- `.lock()` — requests access, waiting if the mutex is already locked. It returns a `Result`.
- `if let Ok(mut slot) = ...` — if locking succeeds, name the returned lock guard `slot` and make the binding mutable. If it returns `Err`, skip the block.
- `*slot = Some(color)` — replace the protected data through the guard.
- When `slot` leaves scope, the guard is dropped and the lock is released automatically.

Here, `=` matches the result against the `Ok(mut slot)` pattern; it is not an equality comparison. `Mutex` protects the data but does not notify GPUI when it changes.

### Channels (sender and receiver)

A channel passes values between separate parts of a program, such as the Windows mouse hook and a GPUI task.

```rust
let (sender, receiver) = async_channel::unbounded::<Rgb>();

// In the hook:
let result = sender.try_send(color); // Ok(()) or an error.

// In the receiving async task:
while let Ok(color) = receiver.recv().await {
    // Update Picker with this color.
}
```

- `unbounded::<Rgb>()` creates a channel carrying `Rgb` values and returns its two ends as a tuple. Sent colors wait in a queue with no fixed capacity; the queue uses memory.
- `try_send(color)` moves the color into the channel without waiting and wakes a waiting receiver.
- `recv().await` waits for one color without blocking the UI thread. The loop handles future colors too; it does not repeatedly check the channel while waiting.
- `OnceLock<Sender<Rgb>>` stores the sender once, not the colors. That same sender can send many colors.

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

```rust

.on_click(move |_, _, cx| {
    cx.write_to_clipboard(ClipboardItem::new_string(
        hex_to_copy.clone(),
    ));
}),

```

- `move` passes the ownershipt of the `hex_to_copy` to the callback.
- because when the `view()` function return, `hex_to_copy` vanishes.

### Observing another entity with `cx.observe()`

`observe()` registers a callback that runs when another entity calls `cx.notify()`.

It does not automatically compare fields or detect assignments.

```rust
observer_context
    .observe(&entity_to_watch, |observer_state, changed_entity, cx| {
        // Respond to the notification.
    })
    .detach();
```

`observe()` takes two arguments:

1. `&entity_to_watch` — a reference to the entity handle we want to watch.
2. A closure — the code GPUI runs when that entity notifies.

The closure receives three arguments, in this order:

- `observer_state`: `&mut T` — mutable access to the entity doing the watching.
- `changed_entity`: `Entity<U>` — a handle to the entity that sent the notification.
- `cx`: `&mut Context<T>` — the observing entity’s GPUI context.

Here, `T` is the observer’s type and `U` is the watched entity’s type.

Rust determines these types from the `Context` calling `observe()` and the entity being watched. Parameter names are our choice; their positions determine their meanings.

#### Example: Picker observes the red slider

```rust
picker_context
    .observe(&picker.red_slider, |picker, slider, cx| {
        picker.color.r = slider.read(cx).val_x;
        cx.notify();
    })
    .detach();
```

In this example:

- `picker_context` is `Context<Picker>`, so the observer is `Picker`.
- `picker.red_slider` is `Entity<Slider>`, so the watched entity is `Slider`.
- `picker` is `&mut Picker` — we update its color.
- `slider` is `Entity<Slider>` — we read its current value.
- `cx` is `&mut Context<Picker>` — we notify GPUI that Picker needs refreshing.
- `.read(cx)` borrows the state stored inside the slider entity.
- This example assumes `val_x` is public.

```text
User drags the red slider
  → Slider updates its val_x
  → Slider calls its own cx.notify()
  → GPUI runs Picker’s observer callback
  → Callback reads Slider.val_x
  → Callback updates Picker.color.r
  → Callback calls Picker’s cx.notify()
  → GPUI schedules Picker to render again
```

The two `notify()` calls belong to different entities: the first notifies about Slider; the second notifies about Picker.

#### Why `.detach()`?

`observe()` returns a `Subscription`.

Dropping the subscription cancels the observation. We can either store it in a field or call `.detach()` to keep the observation active without storing the subscription handle.

Register observers once during entity construction, rather than repeatedly inside `render()`.

### `on_click` closure arguments

Without `cx.listener(...)`, `on_click` expects three closure arguments, in this order:

```rust
.on_click(|event, window, app| {
    crate::mouse_click::start_picking();
})
```

- `event: &ClickEvent` — information about the click.
- `window: &mut Window` — access to the window where the click happened.
- `app: &mut App` — access to GPUI's application state and operations.

Parameter names are our choice; their positions determine their meanings.

```rust
.on_click(|_, _, _| {
    crate::mouse_click::start_picking();
})
```

Each `_` means “accept this argument, but ignore it.” Here, `start_picking()` needs none of these values.

### `cx.listener()`

`cx.listener()` wraps a closure so it can access an entity's state and context. It returns a callback compatible with `on_click`.

```rust
// Here, cx is Context<Picker>.
.on_click(cx.listener(|picker, event, window, cx| {
    picker.color = Rgb { r: 0, g: 0, b: 255 };
    cx.notify();
}))
```

The closure receives these arguments in order:

- `picker: &mut Picker` — mutable access to the entity's state.
- `event: &ClickEvent` — information about the click.
- `window: &mut Window` — the window receiving the event.
- `cx: &mut Context<Picker>` — GPUI operations for this entity, such as `notify()` to request another render.

In general, the types are `&mut T`, `&E`, `&mut Window`, and `&mut Context<T>`. `T` is the entity type; `E` is the event type. Argument names are our choice, and unused arguments can be written as `_`.

A button function does not need its own struct: it can receive `&mut Context<Picker>` from its parent and use that context to create the listener.

## Windows Stuffs

- `let dc = unsafe{GetDC(None)};` dc holds the entire screen as a reference

```rust
    let pixel = unsafe {
        GetPixel(dc, x, y);
    };
```

- get the pixel from mouse positioin (x,y) from the screen which reference is held by `dc`,

```c
WPARAM(usize) // unsigned, pointer-sized
LPARAM(isize) // signed, pointer-sized
LRESULT(isize) // signed, pointer-sized
```

```rust
unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
}
```

- `extern` Dont use Rust default `ABI` for this function.
- `system` enforces the system e.g(windows/linux/mac) ABI.
- What is ABI (Application Binary Interface):
  - `Argument Passing`: How arguments are passed to a function
  - `Return Values`: How a function returns its result
  - `Register & Stack management`: How the stack and registers are handled during function calls.
  - summary: defines how the function is called.

```rust
    if code == HC_ACTION as i32 && wparam.0 == WM_LBUTTONDOWN as usize {
        let event = lparam.0 as *const MSLLHOOKSTRUCT;

        let point = unsafe { (*event).pt };
    }
```

- `code == HC_ACTION` => tells that, this event can be processed by this hook.
- `lparam` contains the `event` data address.
- `MSSLHOOKSTRUCT` kind of a rust wrapper which converts the `event` to something that rust can interact
