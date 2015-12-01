extern crate winapi;
extern crate user32;

use std::ffi::CString;

fn main() {
    let text = CString::new("Hello from Rust!").unwrap();
    let caption = CString::new("winapi-rs").unwrap();

    unsafe {
        user32::MessageBoxA(std::ptr::null_mut(), text.as_ptr(), caption.as_ptr(), winapi::winuser::MB_OK);
    }
}
