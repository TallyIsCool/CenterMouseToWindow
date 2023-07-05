use winapi::um::winuser::{GetForegroundWindow, SetForegroundWindow, GetWindowRect, SetCursorPos};
use winapi::shared::windef::{HWND, RECT};
use std::thread;
use std::time::Duration;
use toy_arms::VirtualKeyCode;

fn main() {
  println!("When an application is selected, press insert to toggle the mouse fix.");
  unsafe {
    loop {
      if toy_arms::detect_keypress(VirtualKeyCode::VK_INSERT) {
        let hwnd: HWND = GetForegroundWindow();
        let mut rect: RECT = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect);
        let mut isenabled:bool = false;
        let center_x = (rect.left + rect.right) / 2;
        let center_y = (rect.top + rect.bottom) / 2;
        SetForegroundWindow(hwnd);
        isenabled = !isenabled;
        print!("\x1B[2J\x1B[1;1H");
        println!("enabled");
        while isenabled == true {
        thread::sleep(Duration::from_millis(10));
        SetCursorPos(center_x, center_y);
          if toy_arms::detect_keypress(VirtualKeyCode::VK_INSERT) {
            print!("\x1B[2J\x1B[1;1H");
            println!("disabled");
            break;
          }   
        }
      }
    }
  }
}