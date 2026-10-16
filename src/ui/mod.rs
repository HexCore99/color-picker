mod body;
mod controls;
mod copy_button;
mod header;
mod value_row;

pub use controls::Controls;

use crate::color::Rgb;
use gpui::{Entity, div, prelude::*, rgb};

pub fn view(color: &Rgb, controls: &Entity<Controls>) -> impl IntoElement {
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
        .child(body::view(color, controls))
}
