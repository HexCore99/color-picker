use crate::windows_sampling::sample_color_at;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetWindowThreadProcessId, HC_ACTION, HHOOK, MSLLHOOKSTRUCT, SetWindowsHookExW,
    UnhookWindowsHookEx, WH_MOUSE_LL, WM_LBUTTONDOWN, WindowFromPoint,
};

use crate::color::Rgb;
use async_channel::{Receiver, Sender};
use std::sync::OnceLock;

// AtomicBool - safely read/change values across different threads
static PICKING: AtomicBool = AtomicBool::new(false);
static COLOR_SENDER: OnceLock<Sender<Rgb>> = OnceLock::new();

pub fn start_picking() {
    PICKING.store(true, Ordering::Relaxed);
}

pub fn cancel_picking() {
    PICKING.store(false, Ordering::Relaxed);
}
pub fn current_picking_state() -> bool {
    PICKING.load(Ordering::Relaxed)
}

pub fn create_color_channel() -> Result<Receiver<Rgb>, &'static str> {
    let (sender, receiver) = async_channel::unbounded::<Rgb>(); //unbound -> A channel has a queue: it holds sent values until the receiver takes them.
    COLOR_SENDER
        .set(sender)
        .map_err(|_| "color channel already initialized")?;
    Ok(receiver)
}

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32
        && wparam.0 == WM_LBUTTONDOWN as usize
        && PICKING.load(Ordering::Relaxed)
    {
        let event = lparam.0 as *const MSLLHOOKSTRUCT;
        let point = unsafe { (*event).pt };

        // find the process id
        let mut process_id = 0;

        unsafe {
            let window = WindowFromPoint(point);
            GetWindowThreadProcessId(window, Some(&mut process_id));
        }

        //Ignore our ap so clicking Cancel doesnt sample the Picker button.

        if process_id != std::process::id() && PICKING.load(Ordering::Relaxed) {
            match sample_color_at(point.x, point.y) {
                Ok(color) => match COLOR_SENDER.get() {
                    Some(sender) => match sender.try_send(color) {
                        Ok(()) => {}
                        Err(error) => eprintln!("Could not send color: {error}"),
                    },
                    None => {}
                },
                Err(error) => eprintln!("Sampling failed: {error}"),
            }
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
