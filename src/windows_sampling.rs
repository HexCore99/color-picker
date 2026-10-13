use windows::{
    Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos},
    core::Result,
};

pub fn cursor_position() -> Result<(i32, i32)> {
    let mut position = POINT::default(); // sets the default value as (0,0)

    unsafe {
        GetCursorPos(&mut position)?; // if this function call fails, ? returns the error
    }

    Ok((position.x, position.y))
}
