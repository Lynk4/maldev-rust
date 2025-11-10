use std::ptr;
use winapi::um::processthreadsapi::{CreateProcessW, STARTUPINFOW, PROCESS_INFORMATION};
use winapi::um::winbase::{CREATE_NO_WINDOW, DETACHED_PROCESS};
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::shared::ntdef::NULL;

fn main() {
    unsafe {
        let mut si: STARTUPINFOW = std::mem::zeroed();
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

        // UTF-16 string: "calc.exe\0"
        let mut cmd_line: Vec<u16> = "calc.exe\0".encode_utf16().collect();

        let success = CreateProcessW(
            NULL as *mut _,                    // lpApplicationName
            cmd_line.as_mut_ptr(),             // lpCommandLine
            ptr::null_mut(),                   // lpProcessAttributes
            ptr::null_mut(),                   // lpThreadAttributes
            0,                                 // bInheritHandles
            CREATE_NO_WINDOW | DETACHED_PROCESS,
            NULL as *mut _,                    // lpEnvironment
            NULL as *mut _,                    // lpCurrentDirectory
            &mut si,                           // lpStartupInfo
            &mut pi,                           // lpProcessInformation
        );

        if success == 0 {
            let err = std::io::Error::last_os_error();
            eprintln!("CreateProcessW failed: {}", err);
            return;
        }

        // CORRECT: No dereference needed
        println!("Launched PID: {}", pi.dwProcessId);
    }
}