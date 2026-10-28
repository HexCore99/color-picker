use super::{Controls, copy_button, value_row};
use crate::{color::Rgb, ui::slider::Slider};
use gpui::{Entity, FontWeight, div, prelude::*, px, rgb};

pub fn view(color: &Rgb, controls: &Entity<Controls>) -> impl IntoElement {
    let preview_color = (u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b);
    let (hsl_h, hsl_s, hsl_l) = color.hsl_channels();
    let (hsv_h, hsv_s, hsv_v) = color.hsv_channels();
    let slider = Slider::default();

    div()
        .w_full()
        .flex()
        .gap_3()
        .child(
            div() // Color Swatch + Slider
                .flex_1()
                .pt(px(5.0))
                .px(px(4.0)) // 5px left-right pad
                .h(px(300.0))
                .rounded_lg()
                .bg(rgb(0x292d30))
                .child(
                    div() // Color swatch
                        .w_full()
                        .h(px(130.0))
                        .flex_shrink_0()
                        .rounded_lg()
                        .bg(rgb(preview_color)),
                )
                .child(div()
                    .w_full()
                    .pt_10()
                    .child(slider.view())
                )
        )
        .child(
            div() // Right section (color values)
                .flex_1()
                .h(px(300.0))
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
                )
                .child(controls.clone())
        )
}
