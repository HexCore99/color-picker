use gpui::{
    Context, DragMoveEvent, FontWeight, MouseButton, MouseDownEvent, Pixels, Window, div,
    prelude::*, px, rgb,
};

pub struct Slider {
    color: u8, // i think i will use an ENUM here
    val_x: u8,
    track_width: f32,
    thumb_size: f32,
    drag_start: Option<(Pixels, u8)>,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            color: 0,
            val_x: 0,
            track_width: 250.0,
            thumb_size: 22.0,
            drag_start: None,
        }
    }
}

impl Render for Slider {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let travel = self.track_width - self.thumb_size;
        let progress = f32::from(self.val_x) / 255.0;
        let thumb_x = progress * travel;

        div()
            .flex()
            .items_center()
            .gap_3()
            .child(
                div() // Positions the track and thumb
                    .relative()
                    .w(px(self.track_width))
                    .h(px(self.thumb_size))
                    .flex_shrink_0()
                    .child(
                        div() // Gray track
                            .absolute()
                            .top(px(6.0))
                            .w_full()
                            .h(px(10.0))
                            .rounded_full()
                            .bg(rgb(0x999999)),
                    )
                    .child(
                        div() // Colored portion
                            .absolute()
                            .top(px(6.0))
                            .w(px(thumb_x + self.thumb_size / 2.0))
                            .h(px(10.0))
                            .rounded_full()
                            .bg(rgb(0xff5544)),
                    )
                    .child(
                        div() // White thumb
                            .id("slider-thumb")
                            .absolute()
                            .left(px(thumb_x))
                            .w(px(self.thumb_size))
                            .h(px(self.thumb_size))
                            .rounded_full()
                            .cursor_pointer()
                            .bg(rgb(0xffffff))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|slider,event:&MouseDownEvent,_,_|{
                                    slider.drag_start = Some((event.position.x,slider.val_x));
                                }) ,
                    )
                    .on_drag(SliderDrag, |_,_,_,cx| cx.new(|_|SliderDrag))
                /*
                on_click(
                on_push_down_over_slider,
                get the initial pos x = 0(assuimng)
                continuosly read the x pos of the mouse pointer,
                then adding it to the initial x until 255.
                if it crosses 255 values, dont add.
                )


                 */
            )
            .child(
                div() // Slider Value Label
                    .w(px(44.0))
                    .h(px(26.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_sm()
                    .bg(rgb(0x363636))
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .child(self.val_x.to_string()),
            )
    }
}
