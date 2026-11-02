mod body;
mod copy_button;
mod header;
mod pick_button;
mod slider;
mod value_row;

pub use slider::{Slider, SliderColor};

use crate::{Picker, color::Rgb};
use gpui::{Entity, div, prelude::*, rgb};

pub fn view(
    color: &Rgb,
    cx: &mut Context<Picker>,

    red_slider: &Entity<Slider>,
    green_slider: &Entity<Slider>,
    blue_slider: &Entity<Slider>,
) -> impl IntoElement {
    div()
        .size_full()
        .flex()
        .flex_col()
        .items_start()
        .gap_3()
        .p_4()
        .bg(rgb(0x202020))
        .text_color(rgb(0xffffff))
        .child(header::view())
        .child(body::view(color, cx, red_slider, green_slider, blue_slider))
}
