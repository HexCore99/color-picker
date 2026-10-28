use gpui::{
    Context, FontWeight, Window, div, linear_color_stop, linear_gradient, prelude::*, px, rgb, svg,
};

use crate::{mouse_click, ui::controls::PickMode::Idle};
#[derive(Clone, Copy, PartialEq, Eq)]
enum PickMode {
    Idle,
    Picking,
}

pub struct Controls {
    selected_format: &'static str,
    pick_mode: PickMode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            selected_format: "HEX",
            pick_mode: Idle,
        }
    }
}

impl Render for Controls {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_3()
            .child(pick_button(self.pick_mode, cx))
        // .child(format_selector(self.selected_format, cx))
    }
}

fn pick_button(mode: PickMode, cx: &mut Context<Controls>) -> impl IntoElement {
    let is_picking = mode == PickMode::Picking;
    div()
        .id("pick-screen")
        .w_full()
        .h(px(48.0))
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .gap_3()
        .px_3()
        .rounded_lg()
        .border_1()
        .border_color(rgb(0x9665ff))
        .cursor_pointer()
        .bg(linear_gradient(
            90.0,
            linear_color_stop(rgb(0x8054f5), 0.0),
            linear_color_stop(rgb(0x7c32ef), 1.0),
        ))
        .text_color(rgb(0xffffff))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    svg()
                        .path("icons/pipette.svg")
                        .w(px(24.0))
                        .h(px(24.0))
                        .text_color(rgb(0xffffff)),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(if is_picking {
                            "Picking...."
                        } else {
                            "Pick from screen"
                        }),
                ),
        )
        .child(div().w(px(1.0)).h(px(24.0)).bg(rgb(0xb58aff)))
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(0xd4baff))
                .child(if is_picking {
                    "Click to cancel"
                } else {
                    "Win + Shift + C"
                }),
        )
        .on_click(cx.listener(|controls, _, _, cx| {
            controls.pick_mode = match controls.pick_mode {
                PickMode::Idle => {
                    mouse_click::start_picking();
                    PickMode::Picking
                }
                PickMode::Picking => {
                    mouse_click::cancel_picking();
                    PickMode::Idle
                }
            };
            cx.notify();
        }))
}

// fn format_selector(selected_format: &'static str, cx: &mut Context<Controls>) -> impl IntoElement {
//     div()
//         .w_full()
//         .h(px(38.0))
//         .flex_shrink_0()
//         .flex()
//         .rounded_md()
//         .border_1()
//         .border_color(rgb(0x3b4050))
//         .bg(rgb(0x22252e))
//         .cursor_pointer()
//         .children(["HEX", "RGB", "HSL", "HSV", "CMYK"].map(|label| {
//             let selected = label == selected_format;
//
//             div()
//                 .id(label)
//                 .flex_1()
//                 .h_full()
//                 .flex()
//                 .items_center()
//                 .justify_center()
//                 .rounded_md()
//                 .border_1()
//                 .border_color(rgb(if selected { 0xa36cff } else { 0x2e323e }))
//                 .bg(rgb(if selected { 0x533292 } else { 0x22252e }))
//                 .text_size(px(12.0))
//                 .text_color(rgb(if selected { 0xffffff } else { 0xb9bdd2 }))
//                 .child(label)
//                 .on_click(cx.listener(move |controls, _, _, cx| {
//                     controls.selected_format = label;
//                     cx.notify();
//                 }))
//         }))
// }
