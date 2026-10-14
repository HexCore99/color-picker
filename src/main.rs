use gpui::{
    App, Application, Bounds, ClipboardItem, Context, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

mod color;
mod mouse_click;
mod windows_sampling;
use color::Rgb;

struct Picker {
    color: Rgb,
}

impl Render for Picker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let preview_color = (u32::from(self.color.r) << 16)
            | (u32::from(self.color.g) << 8)
            | (u32::from(self.color.b));

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .gap_3()
            .p_4()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .child(format!("{} | {}", self.color.to_hex(), self.color.to_rgb(),))
    }
}

fn main() {
    match windows_sampling::sample_cursor_color() {
        Ok(color) => println!("Sample color: {}", color.to_hex()),
        Err(error) => println!("Sampling failed: {error}"),
    }

    // register the hook
    let hook = match mouse_click::register_mouse_hook() {
        Ok(hook) => hook,
        Err(error) => {
            eprintln!("could not register mouse hook: {error}");
            return;
        }
    };

    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(360.0), px(240.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Picker {
                    color: Rgb {
                        r: 255,
                        g: 128,
                        b: 0,
                    },
                })
            },
        )
        .expect("failed to open the window");

        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        cx.activate(true);
    });
    if let Err(error) = mouse_click::unregister_mouse_hook(hook) {
        eprintln!("Could not remove mouse hook: {error}");
    }
}
