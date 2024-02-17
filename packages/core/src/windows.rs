extern crate windows_sys;

use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM, PWSTR, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, MB_ICONINFORMATION, MB_OK, MB_TASKMODAL, MB_TOPMOST,
};

pub fn main() {
    unsafe {
        MessageBoxW(
            HWND::default(),
            PWSTR("Hello, Windows!\0".as_ptr() as _),
            PWSTR("Editify\0".as_ptr() as _),
            MB_OK | MB_ICONINFORMATION | MB_TASKMODAL | MB_TOPMOST,
        );
    }
}
