use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

mod color;
use color::Rgb;

struct Picker {
    color: Rgb,
}

impl Render for Picker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .child(format!("{} | {}", self.color.to_hex(), self.color.to_rgb(),))
    }
}

fn main() {
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
        cx.activate(true);
    })
}
