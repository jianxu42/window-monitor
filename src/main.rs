/// This program monitors the specified processes and closes their windows if they are found.
/// The process names to be monitored are hard-coded in the function.
/// To monitor a process, use Spy++ to get the window name.
///
use std::env;
use std::thread;
use std::time::Duration;
use windows::core::s;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, SendMessageA, WM_CLOSE};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{} is monitoring...", args[0]);

    unsafe {
        loop {
            // to monitor the process, you can use Spy++ to get the window name
            let h_wnd = FindWindowA(None, s!("Message"));
            let h_wnd_agent = FindWindowA(None, s!("Microsoft.Flow.RPA.Agent.exe - System Error"));
            let h_wnd_pad = FindWindowA(None, s!("PAD.AutomationServer.exe - System Error"));
            if h_wnd.0 != 0 {
                SendMessageA(h_wnd, WM_CLOSE, WPARAM::default(), LPARAM::default());
            }
            if h_wnd_agent.0 != 0 {
                SendMessageA(h_wnd_agent, WM_CLOSE, WPARAM::default(), LPARAM::default());
            }
            if h_wnd_pad.0 != 0 {
                SendMessageA(h_wnd_pad, WM_CLOSE, WPARAM::default(), LPARAM::default());
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
