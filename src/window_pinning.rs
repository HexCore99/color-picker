use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::ffi::c_void;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetWindowPos,
};

#[cfg(target_os = "windows")]
pub fn pin_window(window: &Window, pinned: bool) -> Result<(), String> {
    let handle = HasWindowHandle::window_handle(window).map_err(|error| error.to_string())?;

    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return Err("Expected a Windows window handle".into());
    };

    let hwnd = HWND(handle.hwnd.get() as *mut c_void);

    let position = if pinned { HWND_TOPMOST } else { HWND_NOTOPMOST };

    unsafe {
        SetWindowPos(
            hwnd,
            Some(position),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        )
    }
    .map_err(|error| error.to_string())
}
