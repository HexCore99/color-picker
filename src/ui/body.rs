use crate::color::Rgb;
use gpui::{FontWeight, div, prelude::*, px, rgb};

use super::{copy_button, value_row};

pub fn view(color: &Rgb) -> impl IntoElement {
    let preview_color = (u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b);
    let (hsl_h, hsl_s, hsl_l) = color.hsl_channels();
    let (hsv_h, hsv_s, hsv_v) = color.hsv_channels();

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
                                .flex_shrink_0()
                                .rounded_lg()
                                .bg(rgb(preview_color)),
                        )
                        .child(
                            div() //Right of the Swatch
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_3()
                                .child(
                                    div()
                                        .w_full()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .gap_3()
                                        .child(
                                            div() // HEX heading
                                                .text_size(px(28.0))
                                                .font_weight(FontWeight::BOLD)
                                                .child(color.to_hex()),
                                        )
                                        .child(copy_button::view("copy-hex", color.to_hex())),
                                )
                                .child(value_row::view(
                                    "RGB",
                                    format!("{}, {}, {}", color.r, color.g, color.b),
                                    "copy-rgb",
                                    color.to_rgb(),
                                ))
                                .child(value_row::view(
                                    "HSL",
                                    format!("{hsl_h}°, {hsl_s}%, {hsl_l}%"),
                                    "copy-hsl",
                                    color.to_hsl(),
                                ))
                                .child(value_row::view(
                                    "HSV",
                                    format!("{hsv_h}°, {hsv_s}%, {hsv_v}%"),
                                    "copy-hsv",
                                    color.to_hsv(),
                                )),
                        ),
                ),
        )
}
