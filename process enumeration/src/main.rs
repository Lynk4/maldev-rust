use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
        PROCESSENTRY32W, TH32CS_SNAPPROCESS
    },
};

fn main() -> windows::core::Result<()> {
    unsafe {
        // Take snapshot of all processes
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        if snapshot == HANDLE(std::ptr::null_mut()) {
            return Err(windows::core::Error::from_win32());
        }

        let mut process_entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        // Get first process
        Process32FirstW(snapshot, &mut process_entry)?;

        // Iterate over all processes
        loop {
            let pid = process_entry.th32ProcessID;
            let name = String::from_utf16_lossy(
                &process_entry.szExeFile[..process_entry.szExeFile.iter()
                    .position(|&c| c == 0).unwrap_or(process_entry.szExeFile.len())]
            );

            println!("PID: {:5} | {}", pid, name);

            // Try to get next process — break on failure
            if Process32NextW(snapshot, &mut process_entry).is_err() {
                break;
            }
        }

        CloseHandle(snapshot)?;
    }
    Ok(())
}