use std::env;
use std::ffi::CString;
use std::thread;
use std::time::Duration;

use windows::core::PCSTR;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, SendMessageA, WM_CLOSE};

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Skip the first argument which is the program name itself
    let window_names = &args[1..];

    // Print program name and window names to monitor
    println!("{} is monitoring...", args[0]);
    println!("Window names to close: {:?}", window_names);

    // Ensure the unsafe block is used minimally
    unsafe {
        // Main loop to continuously check for windows to close
        loop {
            // Iterate over the provided window names
            for window_name in window_names {
                // Convert the Rust string to a C-style string
                let window_name_cstr =
                    CString::new(window_name.as_str()).expect("CString::new failed");
                let window_name_pcstr = PCSTR(window_name_cstr.as_ptr() as *const u8);

                // Find the window with the given name
                let hwnd = FindWindowA(None, window_name_pcstr);

                // If a valid window handle is found, send the WM_CLOSE message
                if hwnd.0 != 0 {
                    SendMessageA(hwnd, WM_CLOSE, WPARAM::default(), LPARAM::default());
                }
            }

            // Sleep for 500 milliseconds before checking again
            thread::sleep(Duration::from_millis(500));
        }
    }
}
