#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{tracepoint, map},
    maps::HashMap,
    programs::TracePointContext,
    helpers::{bpf_probe_read_user_str_bytes, bpf_send_signal, bpf_get_current_pid_tgid},
};

#[map]
static ASSASSINATED_PIDS: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[tracepoint]
pub fn vantio_phantom_fork(ctx: TracePointContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;

    // Read the string pointer from offset 16 of the sys_enter_execve tracepoint
    let filename_ptr: *const u8 = unsafe { ctx.read_at::<*const u8>(16) }.unwrap_or(core::ptr::null());
    
    if filename_ptr.is_null() {
        return 0;
    }

    let mut buf = [0u8; 64];
    let read_result = unsafe { bpf_probe_read_user_str_bytes(filename_ptr, &mut buf) };
    
    if read_result.is_ok() {
        let mut is_malicious = false;
        
        // Scan the extracted user-space string for 'w' 'g' 'e' 't'
        for i in 0..60 {
            if buf[i] == b'w' && buf[i+1] == b'g' && buf[i+2] == b'e' && buf[i+3] == b't' {
                is_malicious = true;
                break;
            }
        }

        if is_malicious {
            // Drop the hammer
            let _ = unsafe { bpf_send_signal(9) };
            
            // Log the kill to the orchestrator map
            let val: u32 = 1;
            let _ = unsafe { ASSASSINATED_PIDS.insert(&pid, &val, 0) };
        }
    }
    
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";