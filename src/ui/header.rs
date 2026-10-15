use gpui::{div, prelude::*, px, rgb};

pub fn view() -> impl IntoElement {
    div()
        .w_full()
        .h(px(48.0))
        .flex()
        .items_center()
        .p_4()
        .bg(rgb(0x252832))
        .child("Color Picker")
}
