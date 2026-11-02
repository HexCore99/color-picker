use gpui::{
    Context, DragMoveEvent, EntityId, FontWeight, MouseButton, MouseDownEvent, Pixels, Window, div,
    prelude::*, px, rgb,
};

pub enum SliderColor {
    Red,
    Green,
    Blue,
}

impl SliderColor {
    fn to_rgb(&self) -> gpui::Rgba {
        match self {
            Self::Red => rgb(0xff0000),
            Self::Green => rgb(0x00ff00),
            Self::Blue => rgb(0x0000ff),
        }
    }
}

pub struct Slider {
    fill_color: SliderColor,
    val_x: u8,
    bar_width: f32,
    thumb_size: f32,
    drag_start: Option<(Pixels, u8)>,
}

#[derive(Clone)]
struct SliderDrag {
    owner: EntityId,
}
impl Render for SliderDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            fill_color: SliderColor::Red,
            val_x: 0,
            bar_width: 250.0,
            thumb_size: 22.0,
            drag_start: None,
        }
    }
}

#[allow(dead_code)]
impl Slider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bar_width(mut self, width: f32) -> Self {
        self.bar_width = width;
        self
    }

    pub fn thumb_size(mut self, size: f32) -> Self {
        self.thumb_size = size;
        self
    }

    pub fn fill_color(mut self, color: SliderColor) -> Self {
        self.fill_color = color;
        self
    }

    pub fn value(mut self, value: u8) -> Self {
        self.val_x = value;
        self
    }

    pub fn current_value(&self) -> u8 {
        self.val_x
    }
    pub fn set_slider_value(&mut self, val: u8) {
        self.val_x = val;
    }
}

impl Render for Slider {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let travel = self.bar_width - self.thumb_size;
        let progress = f32::from(self.val_x) / 255.0;
        let thumb_x = progress * travel;

        div()
            .flex()
            .items_center()
            .gap_3()
            .child(
                div() // Positions the track and thumb
                    .relative()
                    .w(px(self.bar_width))
                    .h(px(self.thumb_size))
                    .flex_shrink_0()
                    .on_drag_move(cx.listener(
                        |slider, event: &DragMoveEvent<SliderDrag>, _, cx| {
                            if event.drag(cx).owner != cx.entity_id() {
                                return;
                            }

                            if let Some((starting_mouse_x, starting_value)) = slider.drag_start {
                                let distance =
                                    (event.event.position.x - starting_mouse_x) / px(1.0);

                                let travel = slider.bar_width - slider.thumb_size;

                                let value = f32::from(starting_value) + (distance / travel) * 255.0;

                                slider.val_x = value.clamp(0.0, 255.0).round() as u8;
                                cx.notify();
                            }
                        },
                    ))
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
                            .bg(self.fill_color.to_rgb()),
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
                                cx.listener(|slider, event: &MouseDownEvent, _, _| {
                                    slider.drag_start = Some((event.position.x, slider.val_x));
                                }),
                            )
                            .on_drag(
                                SliderDrag {
                                    owner: cx.entity_id(),
                                },
                                |drag, _, _, cx| cx.new(|_| drag.clone()),
                            ),
                        /*
                        |drag, offset, window, cx| { ... }

                        drag: &SliderDrag — a reference to your drag data.
                        offset: Point<Pixels> — where the initiating click occurred relative to the element’s origin.
                        window: &mut Window — the current window.
                        cx: &mut App — the GPUI application context.
                        */
                    ),
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
