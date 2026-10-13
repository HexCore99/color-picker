use gpui::{
    App, Application, Bounds, ClipboardItem, Context, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

mod color;
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
            .child(
                div()
                    .w(px(64.0))
                    .h(px(64.0))
                    .rounded_md()
                    .bg(rgb(preview_color)),
            )
            .child(format!("{} | {}", self.color.to_hex(), self.color.to_rgb(),))
            .child(
                div()
                    .id("change-color")
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x404040))
                    .cursor_pointer()
                    .child("Change color")
                    .on_click(cx.listener(|picker, _event, _window, cx| {
                        picker.color = Rgb { r: 0, g: 0, b: 255 };
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .id("Copy-text")
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x404040))
                    .cursor_pointer()
                    .child("Copy HEX")
                    .on_click(cx.listener(|picker, _event, _window, cx| {
                        let text = picker.color.to_hex();
                        cx.write_to_clipboard(ClipboardItem::new_string(text));
                    })),
            )
    }
}

fn main() {
    match windows_sampling::cursor_position() {
        Ok((x, y)) => println!("Cursor position: ({x}, {y})"),
        Err(error) => println!("Could not read teh cursor position: {error}"),
    }

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
