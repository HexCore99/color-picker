use crate::{
    Picker,
    mouse_click::{cancel_picking, current_picking_state, start_picking},
};
use gpui::{div, prelude::*, rgb};

pub fn view(cx: &mut Context<Picker>) -> impl IntoElement {
    div()
        .id("pick-button")
        .w_full()
        .h_12()
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .px_3()
        .rounded_md()
        .bg(rgb(0x8054f5))
        .text_color(rgb(0xffffff))
        .cursor_pointer()
        .child(if current_picking_state() {
            "Picking...."
        } else {
            "Pick"
        })
        .on_click(cx.listener(|_picker, _, _, cx| {
            match current_picking_state() {
                true => cancel_picking(),
                false => start_picking(),
            }
            cx.notify();
        }))
}
