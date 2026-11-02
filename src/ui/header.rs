use std::sync::atomic::{
    AtomicBool,
    Ordering::{self},
};

use gpui::{WindowControlArea, div, prelude::*, px, rgb, svg};

static PIN: AtomicBool = AtomicBool::new(false);

pub fn view() -> impl IntoElement {
    let is_pinned = PIN.load(Ordering::Relaxed);
    div()
        .w_full()
        .h(px(48.0))
        .flex()
        .items_center()
        .bg(rgb(0x252832))
        .child(
            div()
                .flex_1()
                .h_full()
                .flex()
                .items_center()
                .px_4()
                .window_control_area(WindowControlArea::Drag)
                .child("Color Picker"),
        )
        .child(
            div()
                .id("pin")
                .mr_2()
                .px_4()
                .cursor_pointer()
                .hover(|style| style.bg(rgb(0xc42b1c)))
                .child(
                    svg()
                        .path(if is_pinned {
                            "icons/pinn.svg"
                        } else {
                            "icons/unpin.svg"
                        })
                        .size(px(16.0))
                        .text_color(rgb(0xffffff)),
                )
                .on_click(|_, window, _| {
                    PIN.fetch_xor(true, Ordering::Relaxed);
                    window.refresh();
                }),
        )
        .child(
            div()
                .id("minimize")
                .px_4()
                .cursor_pointer()
                .on_click(|_, window, _| {
                    window.minimize_window();
                })
                .hover(|style| style.bg(rgb(0xc42b1c)))
                .child("−"),
        )
        .child(
            div()
                .id("close")
                .mr_2()
                .px_4()
                .cursor_pointer()
                .hover(|style| style.bg(rgb(0xc42b1c)))
                .on_click(|_, window, _| {
                    window.remove_window();
                })
                .child("×"),
        )
}
