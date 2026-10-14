use crate::windows_sampling::sample_color_at;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, HC_ACTION, HHOOK, MSLLHOOKSTRUCT, SetWindowsHookExW, UnhookWindowsHookEx,
    WH_MOUSE_LL, WM_LBUTTONDOWN,
};
unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 && wparam.0 == WM_LBUTTONDOWN as usize {
        let event = lparam.0 as *const MSLLHOOKSTRUCT;

        let point = unsafe { (*event).pt };
        let sample = sample_color_at(point.x, point.y);

        match sample {
            Ok(rgb) => println!("Sample color: {}", rgb.to_hex()),
            Err(message) => eprintln!("Sampleing failed : {message}"),
        }
    }
    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

pub fn register_mouse_hook() -> windows::core::Result<HHOOK> {
    let module = unsafe { GetModuleHandleW(None) }?; //get's a handle to the current executable
    unsafe { SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), Some(HINSTANCE(module.0)), 0) }
}

pub fn unregister_mouse_hook(hook: HHOOK) -> windows::core::Result<()> {
    unsafe { UnhookWindowsHookEx(hook) }
}

// WH_MOUSE_LL -> selects lo-level mouse events
// Some(mouse_hook) supplies the address of callback to Windows
// Some(HINSTANCE(module.0)) => identifies teh executable containing it
// 0 -> requests event across the desktop.
