fn main() {
    println!("cargo:rerun-if-changed=color_picker_app_icons/color-picker.ico");

    winresource::WindowsResource::new()
        .set_icon("color_picker_app_icons/color-picker.ico")
        .compile()
        .expect("Failed to embed app icon");
}
