#![windows_subsystem = "windows"]  // HIDE CONSOLE

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONINFORMATION};
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;

#[cfg(target_os = "windows")]
fn main() -> windows::core::Result<()> {
    unsafe {
        MessageBoxW(
            None,
            PCWSTR::from(windows::core::w!("hey baby!!!....")),
            PCWSTR::from(windows::core::w!("Lynk4")),
            MB_OK | MB_ICONINFORMATION,
        );
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This payload runs only on Windows.");
}