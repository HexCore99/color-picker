use gpui::{
    App, Application, Bounds, Context, Entity, Window, WindowBounds, WindowOptions, prelude::*, px,
    size,
};

use color::Rgb;

use crate::ui::Slider;

mod assets;
mod color;
mod mouse_click;
mod ui;
mod windows_sampling;

struct Picker {
    color: Rgb,
    red_slider: Entity<ui::Slider>,
    green_slider: Entity<ui::Slider>,
    blue_slider: Entity<ui::Slider>,
}

impl Render for Picker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ui::view(
            &self.color,
            cx,
            &self.red_slider,
            &self.green_slider,
            &self.blue_slider,
        )
    }
}

fn main() {
    // create the color sharing channel

    let color_receiver = match mouse_click::create_color_channel() {
        Ok(receiver) => receiver,
        Err(error) => {
            eprintln!("couldn't create the color channel: {error}");
            return;
        }
    };

    // register the hook
    let hook = match mouse_click::register_mouse_hook() {
        Ok(hook) => hook,
        Err(error) => {
            eprintln!("could not register mouse hook: {error}");
            return;
        }
    };

    Application::new()
        .with_assets(assets::Assets)
        .run(move |cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(700.0), px(450.0)), cx);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                move |_, cx| {
                    cx.new(move |picker_context: &mut Context<Picker>| {
                        //cx.new() creates Entity
                        // Context<Picker> gives additional functionality to the Picker
                        let picker = Picker {
                            color: Rgb { r: 0, g: 0, b: 0 },
                            red_slider: picker_context
                                .new(|_| ui::Slider::new().fill_color(ui::SliderColor::Red)),

                            green_slider: picker_context
                                .new(|_| ui::Slider::new().fill_color(ui::SliderColor::Green)),

                            blue_slider: picker_context
                                .new(|_| ui::Slider::new().fill_color(ui::SliderColor::Blue)),
                        };

                        picker_context
                            .observe(&picker.red_slider, |picker, slider, cx| {
                                picker.color.r = slider.read(cx).current_value();
                                cx.notify();
                            })
                            .detach();

                        picker_context
                            .observe(&picker.green_slider, |picker, slider, cx| {
                                picker.color.g = slider.read(cx).current_value();
                                cx.notify();
                            })
                            .detach();

                        picker_context
                            .observe(&picker.blue_slider, |picker, slider, cx| {
                                picker.color.b = slider.read(cx).current_value();
                                cx.notify();
                            })
                            .detach();
                        // weird shitty Rust
                        picker_context
                            .spawn(async move |picker_entity, cx| {
                                while let Ok(color) = color_receiver.recv().await {
                                    let result = picker_entity.update(cx, |picker, cx| {
                                        picker.color = color;
                                        cx.notify();

                                        picker.red_slider.update(cx, |slider, cx| {
                                            slider.set_slider_value(color.r);
                                            cx.notify();
                                        });

                                        picker.green_slider.update(cx, |slider, cx| {
                                            slider.set_slider_value(color.g);
                                            cx.notify();
                                        });

                                        picker.blue_slider.update(cx, |slider, cx| {
                                            slider.set_slider_value(color.b);
                                            cx.notify();
                                        });
                                    });
                                    if result.is_err() {
                                        break;
                                    }
                                }
                            })
                            .detach();

                        picker
                    })
                },
            )
            .expect("failed to open the window");

            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            cx.activate(true);
        });
    if let Err(error) = mouse_click::unregister_mouse_hook(hook) {
        eprintln!("Could not remove mouse hook: {error}");
    }
}
