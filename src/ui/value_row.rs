use gpui::{div, prelude::*, px, rgb};

use super::copy_button;

pub fn view(
    label: &'static str,
    display: String,
    copy_id: &'static str,
    copy_value: String,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .w(px(32.0))
                .flex_shrink_0()
                .text_sm()
                .text_color(rgb(0xa8adc0))
                .child(label),
        )
        .child(
            div()
                .flex_1()
                .h(px(34.0))
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .pl_2()
                .pr_1()
                .rounded_md()
                .border_1()
                .border_color(rgb(0x3b4050))
                .bg(rgb(0x22252e))
                .text_sm()
                .child(display)
                .child(copy_button::inline_view(copy_id, copy_value)),
        )
}
