use gpui::{ClipboardItem, div, prelude::*, px, rgb};

pub fn view(id: &'static str, value: String) -> impl IntoElement {
    button(id, value, true)
}

pub fn inline_view(id: &'static str, value: String) -> impl IntoElement {
    button(id, value, false)
}

fn button(id: &'static str, value: String, boxed: bool) -> impl IntoElement {
    let side = if boxed { 36.0 } else { 28.0 };
    let background = if boxed { 0x252832 } else { 0x22252e };
    let border = if boxed { 0x3b4050 } else { background };

    div()
        .id(id)
        .w(px(side))
        .h(px(side))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .border_1()
        .border_color(rgb(border))
        .bg(rgb(background))
        .cursor_pointer()
        .child(
            div()
                .relative()
                .w(px(18.0))
                .h(px(18.0))
                // Two overlapping outlines form the copy icon.
                .child(
                    div()
                        .absolute()
                        .left(px(2.0))
                        .top(px(1.0))
                        .w(px(10.0))
                        .h(px(12.0))
                        .rounded_sm()
                        .border_1()
                        .border_color(rgb(0xded6f3)),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(6.0))
                        .top(px(5.0))
                        .w(px(10.0))
                        .h(px(12.0))
                        .rounded_sm()
                        .border_1()
                        .border_color(rgb(0xded6f3))
                        .bg(rgb(background)),
                ),
        )
        .on_click(move |_, _, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(value.clone()));
        })
}
