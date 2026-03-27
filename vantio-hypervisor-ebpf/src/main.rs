#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{tracepoint, map},
    maps::HashMap,
    programs::TracePointContext,
    helpers::{bpf_probe_read_user_str_bytes, bpf_send_signal, bpf_get_current_pid_tgid},
};

// The Live Threat Feed. User-space will inject a 4-byte signature here at Key 0.
#[map]
static DYNAMIC_SIGNATURE: HashMap<u32, [u8; 4]> = HashMap::with_max_entries(1, 0);

#[map]
static ASSASSINATED_PIDS: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[tracepoint]
pub fn vantio_phantom_fork(ctx: TracePointContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;

    let filename_ptr: *const u8 = unsafe { ctx.read_at::<*const u8>(16) }.unwrap_or(core::ptr::null());
    if filename_ptr.is_null() { return 0; }

    let mut buf = [0u8; 64];
    let read_result = unsafe { bpf_probe_read_user_str_bytes(filename_ptr, &mut buf) };
    
    if read_result.is_ok() {
        // 1. Pull the LIVE signature from the Threat Feed map (Key 0)
        let key: u32 = 0;
        if let Some(sig) = unsafe { DYNAMIC_SIGNATURE.get(&key) } {
            let mut is_malicious = false;
            
            // 2. Scan the execution string for the dynamic signature
            for i in 0..60 {
                if buf[i] == sig[0] && buf[i+1] == sig[1] && buf[i+2] == sig[2] && buf[i+3] == sig[3] {
                    is_malicious = true;
                    break;
                }
            }

            // 3. Drop the hammer if we found a match
            if is_malicious {
                let _ = unsafe { bpf_send_signal(9) };
                
                let val: u32 = 1;
                let _ = unsafe { ASSASSINATED_PIDS.insert(&pid, &val, 0) };
            }
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