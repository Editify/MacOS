use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null;
use std::ptr::null_mut;
use winapi::shared::basetsd::INT32;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::wingdi::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;
use winapi_gui::*;

fn on_render(hwnd: HWND) {
    unsafe {
        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(hwnd, &mut rect);
        // background
        draw_rect(
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            RGBTRIPLE {
                rgbtBlue: 0x1F,
                rgbtGreen: 0x1F,
                rgbtRed: 0x1F,
            },
        );
        // titlebar
        draw_rect(
            rect.left,
            rect.top,
            rect.right - rect.left,
            20,
            RGBTRIPLE {
                rgbtBlue: 0x00,
                rgbtGreen: 0xFF,
                rgbtRed: 0x00,
            },
        );

        draw_text(
            "Epic Text".to_string(),
            rect.left,
            rect.top,
            rect.right - rect.left,
            20,
            RGBTRIPLE {
                rgbtBlue: 0xFF,
                rgbtGreen: 0x00,
                rgbtRed: 0x00,
            },
            DT_CENTER,
        );

        draw_button(
            20,
            20,
            200,
            200,
            RGBTRIPLE {
                rgbtRed: 0,
                rgbtBlue: 0,
                rgbtGreen: 20,
            },
            RGBTRIPLE {
                rgbtRed: 20,
                rgbtBlue: 0,
                rgbtGreen: 0,
            },
            Box::new(|| {
                print!("OMG BUTTON CLICKED");
            }),
        );
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            DestroyWindow(hwnd);
            PostQuitMessage(0);
            0
        }
        WM_MOUSEMOVE => {
            let x = LOWORD(lparam as DWORD) as INT32;
            let y = HIWORD(lparam as DWORD) as INT32;
            // println!("x {} y {}", x, y);
            handle_mouse_move(x, y);
            0
        }
        WM_LBUTTONDOWN => {
            let x = LOWORD(lparam as DWORD) as INT32;
            let y = HIWORD(lparam as DWORD) as INT32;
            handle_left_mouse_down(x, y);
            0
        }
        WM_GETMINMAXINFO => {
            let min_max_info = lparam as *mut MINMAXINFO;
            unsafe {
                (*min_max_info).ptMinTrackSize.x = 400; // Minimum width
                (*min_max_info).ptMinTrackSize.y = 260; // Minimum height
            }
            0
        }
        WM_TIMER => {
            InvalidateRect(hwnd, null_mut(), TRUE);
            0
        }
        WM_PAINT => {
            on_render(hwnd);
            render(hwnd);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub fn main() {
    unsafe {
        let class_name = OsStr::new("MyWindowClass")
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<_>>();

        let hinstance = GetModuleHandleA(null());

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance,
            lpszClassName: class_name.as_ptr() as LPCWSTR,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        RegisterClassW(&wnd_class);

        let window_name = OsStr::new("Editify")
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<_>>();

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr() as LPCWSTR,
            window_name.as_ptr() as LPCWSTR,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        );
        let mut dev_mode: DEVMODEW = std::mem::zeroed();
        dev_mode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
        SetTimer(hwnd, 1, 1000 / 60, None);
        // Get the current window style
        let style = GetWindowLongW(hwnd, GWL_STYLE);

        // Cast WS_CAPTION to i32 before bitwise operation
        let new_style = style & !(WS_CAPTION as i32);

        // Set the new window style
        SetWindowLongW(hwnd, GWL_STYLE, new_style);

        // Redraw the window to reflect the style change
        SetWindowPos(
            hwnd,
            null_mut(),
            0,
            0,
            0,
            0,
            SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        );

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        while GetMessageW(&mut msg, hwnd, 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
