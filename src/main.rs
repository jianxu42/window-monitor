use std::env;
use std::thread;
use std::time::Duration;
use windows::core::PCSTR;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, SendMessageA, WM_CLOSE};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip the first argument which is the program name itself
    let window_names = &args[1..];

    println!("{} is monitoring...", args[0]);
    println!("Window names to close: {:?}", window_names);

    unsafe {
        loop {
            for window_name in window_names {
                // Convert the Rust string to a C-style string
                let window_name_cstr = PCSTR(window_name.as_ptr());

                let hwnd = FindWindowA(None, window_name_cstr);
                if hwnd.0 != 0 {
                    SendMessageA(hwnd, WM_CLOSE, WPARAM::default(), LPARAM::default());
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
