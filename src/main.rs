use gpui::{
    App, Application, Bounds, Context, Entity, TitlebarOptions, Window, WindowBounds,
    WindowOptions, prelude::*, px, size,
};

mod color;
mod mouse_click;
mod windows_sampling;
use color::Rgb;
mod assets;
mod ui;

struct Picker {
    color: Rgb,
    controls: Entity<ui::Controls>,
}

impl Render for Picker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        ui::view(&self.color, &self.controls)
    }
}

fn main() {
    match windows_sampling::sample_cursor_color() {
        Ok(color) => println!("Sample color: {}", color.to_hex()),
        Err(error) => println!("Sampling failed: {error}"),
    }

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
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(700.0), px(450.0)), cx);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|picker_context| Picker {
                        color: Rgb {
                            r: 255,
                            g: 128,
                            b: 0,
                        },
                        controls: picker_context.new(|_| ui::Controls::default()),
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
