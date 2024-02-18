use once_cell::sync::Lazy;
use std::sync::Mutex;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

pub struct Button<'a> {
    pub x: INT32,
    pub y: INT32,
    pub width: INT32,
    pub height: INT32,
    pub background_color: RGBTRIPLE,
    pub hover_color: RGBTRIPLE,
    pub hovered: bool,
    pub callback: Box<dyn Fn() + Send + 'a>,
}
pub static BUTTONS: Lazy<Mutex<Vec<Button>>> = Lazy::new(|| Mutex::new(vec![]));
pub fn draw_button(
    x: INT32,
    y: INT32,
    width: INT32,
    height: INT32,
    background_color: RGBTRIPLE,
    hover_color: RGBTRIPLE,
    callback: Box<dyn Fn() + Send + 'static>,
) {
    let mut buttons = BUTTONS.lock().unwrap();
    buttons.push(Button {
        x,
        y,
        width,
        height,
        background_color,
        hover_color,
        hovered: false,
        callback,
    })
}

pub struct Rectangle {
    pub x: INT32,
    pub y: INT32,
    pub width: INT32,
    pub height: INT32,
    pub color: RGBTRIPLE,
}
pub static RECTANGLES: Lazy<Mutex<Vec<Rectangle>>> = Lazy::new(|| Mutex::new(vec![]));
pub fn draw_rect(x: INT32, y: INT32, width: INT32, height: INT32, color: RGBTRIPLE) {
    let mut rectangles = RECTANGLES.lock().unwrap();
    rectangles.push(Rectangle {
        x,
        y,
        width,
        height,
        color,
    });
}

// pub struct FONT {
//     pub size: INT32,
//     pub italic: BOOLEAN,
//     pub bold: BOOLEAN,
//     pub family: String,     // Font family name
//     pub style: FontStyle,   // Font style (normal, bold, italic, etc.)
//     pub kerning: bool,      // Whether kerning is applied
//     pub outline_width: u16, // Width of the font's outline
//     pub hinting: Hinting,   // Font hinting mode
// }

pub struct TextBox {
    pub text: String,
    pub x: INT32,
    pub y: INT32,
    pub width: INT32,
    pub height: INT32,
    pub color: RGBTRIPLE,
    pub format: UINT,
}
pub static TEXT: Lazy<Mutex<Vec<TextBox>>> = Lazy::new(|| Mutex::new(vec![]));
pub fn draw_text(
    text: String,
    x: INT32,
    y: INT32,
    width: INT32,
    height: INT32,
    color: RGBTRIPLE,
    format: UINT,
) {
    let mut text_v = TEXT.lock().unwrap();
    text_v.push(TextBox {
        text,
        x,
        y,
        width,
        height,
        color,
        format,
    });
}
// Add a function to handle mouse move events
pub fn handle_mouse_move(x: INT32, y: INT32) {
    let mut buttons = BUTTONS.lock().unwrap();
    for button in buttons.iter_mut() {
        println!(
            "x >= {} && x <= {} && y >= {} && y <= {}",
            button.x,
            button.x + button.width,
            button.y,
            button.y + button.height
        );
        if x >= button.x
            && x <= button.x + button.width
            && y >= button.y
            && y <= button.y + button.height
        {
            // The mouse is over the button
            println!("HOVER SET TO TRUE");
            button.hovered = true;
        } else {
            button.hovered = false;
        }
    }
}

// Add a function to handle left mouse button down events
pub fn handle_left_mouse_down(x: INT32, y: INT32) {
    let buttons = BUTTONS.lock().unwrap();
    for button in buttons.iter() {
        if x >= button.x
            && x <= button.x + button.width
            && y >= button.y
            && y <= button.y + button.height
            && button.hovered
        {
            println!("CALLBACK CALLED");
            // The button was clicked and it was hovered over
            (button.callback)();
        }
    }
}
pub fn render(hwnd: HWND) {
    unsafe {
        let mut ps: PAINTSTRUCT = std::mem::zeroed();
        let hdc = BeginPaint(hwnd, &mut ps);

        let mut rectangles = RECTANGLES.lock().unwrap();
        while let Some((index, rect)) = rectangles.iter().enumerate().next() {
            let hbrush = CreateSolidBrush(RGB(
                rect.color.rgbtRed,
                rect.color.rgbtGreen,
                rect.color.rgbtBlue,
            ));
            let rect = RECT {
                left: rect.x,
                top: rect.y,
                right: rect.x + rect.width,
                bottom: rect.y + rect.height,
            };
            FillRect(hdc, &rect, hbrush as HBRUSH);
            DeleteObject(hbrush as HGDIOBJ);

            // Remove the rectangle from the vector
            rectangles.remove(index);
        }

        let mut buttons = BUTTONS.lock().unwrap();
        while let Some((index, button)) = buttons.iter().enumerate().next() {
            let mut hbrush = CreateSolidBrush(RGB(
                button.background_color.rgbtRed,
                button.background_color.rgbtGreen,
                button.background_color.rgbtBlue,
            ));

            if button.hovered {
                hbrush = CreateSolidBrush(RGB(
                    button.hover_color.rgbtRed,
                    button.hover_color.rgbtGreen,
                    button.hover_color.rgbtBlue,
                ));
            }

            let rect: RECT = RECT {
                left: button.x,
                top: button.y,
                right: button.x + button.width,
                bottom: button.y + button.height,
            };

            FillRect(hdc, &rect, hbrush as HBRUSH);
            DeleteObject(hbrush as HGDIOBJ);
            // Remove the button from the vector
            buttons.remove(index);
        }

        let mut text_boxes = TEXT.lock().unwrap();
        while let Some((index, text_box)) = text_boxes.iter().enumerate().next() {
            // Remove the rectangle from the vector
            let rect = RECT {
                left: text_box.x,
                top: text_box.y,
                right: text_box.x + text_box.width,
                bottom: text_box.y + text_box.height,
            };
            let wide_string: Vec<u16> = text_box.text.encode_utf16().collect();
            SetBkMode(hdc, winapi::um::wingdi::TRANSPARENT as i32); // Convert DWORD to i32
            SetTextColor(
                hdc,
                RGB(
                    text_box.color.rgbtRed,
                    text_box.color.rgbtGreen,
                    text_box.color.rgbtBlue,
                ),
            );
            DrawTextExW(
                hdc,
                wide_string.as_ptr(),
                wide_string.len() as i32,
                &rect as *const RECT as *mut RECT,
                text_box.format,
                std::ptr::null_mut(),
            );
            text_boxes.remove(index);
        }

        ReleaseDC(hwnd, hdc);
    }
}
