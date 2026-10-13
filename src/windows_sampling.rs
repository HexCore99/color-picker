use crate::color::Rgb;
use windows::Win32::Graphics::Gdi::{CLR_INVALID, GetDC, GetPixel, ReleaseDC};
use windows::{
    Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos},
    core::Result,
};

pub fn cursor_position() -> Result<(i32, i32)> {
    // this is windows Result return type.
    let mut position = POINT::default(); // sets the default value as (0,0)

    unsafe {
        GetCursorPos(&mut position)?; // if this function call fails, ? returns the error
    }

    Ok((position.x, position.y))
}
pub fn sample_cursor_color() -> std::result::Result<Rgb, String> {
    let (x, y) =
        cursor_position().map_err(|err| format!("could not read teh cursor position: {err}"))?;
    let dc = unsafe { GetDC(None) };

    if dc.is_invalid() {
        return Err("Could not acquire the desktop device contest".into());
    }

    let pixel = unsafe { GetPixel(dc, x, y) };

    let released = unsafe { ReleaseDC(None, dc) };

    if released == 0 {
        return Err("Could not release the desktop device context".into());
    }

    if pixel.0 == CLR_INVALID {
        return Err(format!("Could not read the desktop pixel at {x} {y}"));
    }

    let value = pixel.0;

    Ok(Rgb {
        r: (value & 0xff) as u8,
        g: ((value >> 8) & 0xff) as u8,
        b: ((value >> 16) & 0xff) as u8,
    })
}
