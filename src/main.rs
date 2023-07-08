use winapi::um::winuser::{GetForegroundWindow, SetForegroundWindow, GetWindowRect, SetCursorPos};
use winapi::shared::windef::{HWND, RECT};
use std::thread;
use std::time::Duration;
use toy_arms::VirtualKeyCode;

fn main() {
  println!("press INS to toggle");
  unsafe {
    loop {
      if toy_arms::detect_keypress(VirtualKeyCode::VK_INSERT) {
        let hwnd: HWND = GetForegroundWindow();
        SetForegroundWindow(hwnd);
        let mut rect: RECT = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect);
        let center_x = (rect.left + rect.right) / 2;
        let center_y = (rect.top + rect.bottom) / 2;
        loop {
          thread::sleep(Duration::from_millis(50));
          SetCursorPos(center_x, center_y);   
          if toy_arms::detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
          }
        }
      }   
    }
  }
}
