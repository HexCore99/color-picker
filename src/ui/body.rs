use crate::color::Rgb;
use gpui::{div, prelude::*, px, rgb};

pub fn view(color: &Rgb) -> impl IntoElement {
    let preview_color = (u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b);

    div()
        .w_full()
        .flex()
        .gap_3()
        .child(
            div() // Left section (magnifier)
                .flex_1()
                .h(px(350.0))
                .rounded_lg()
                .bg(rgb(0x292d30)),
        )
        .child(
            div() // Right section (color values)
                .flex_1()
                .h(px(350.0))
                .flex()
                .flex_col()
                .gap_3()
                .p_4()
                .rounded_lg()
                .bg(rgb(0x292d38))
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .child(
                            div() // Color swatch
                                .w(px(120.0))
                                .h(px(160.0))
                                .rounded_lg()
                                .bg(rgb(preview_color)),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_3()
                                .child(color.to_hex())
                                .child(color.to_rgb()),
                        ),
                ),
        )
}
