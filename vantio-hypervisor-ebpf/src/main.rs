#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{tracepoint, map},
    maps::HashMap,
    programs::TracePointContext,
    helpers::{bpf_probe_read_user_str_bytes, bpf_send_signal, bpf_get_current_pid_tgid},
};

// The Global Threat Registry. 
// Key: A 32-byte string array containing the exact path of the forbidden command.
// Value: A simple u32 (1 = block).
#[map]
static THREAT_REGISTRY: HashMap<[u8; 32], u32> = HashMap::with_max_entries(1024, 0);

#[map]
static ASSASSINATED_PIDS: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[tracepoint]
pub fn vantio_phantom_fork(ctx: TracePointContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;

    let filename_ptr: *const u8 = unsafe { ctx.read_at::<*const u8>(16) }.unwrap_or(core::ptr::null());
    if filename_ptr.is_null() { return 0; }

    // Read the execution path into a 32-byte buffer
    let mut buf = [0u8; 32];
    let read_result = unsafe { bpf_probe_read_user_str_bytes(filename_ptr, &mut buf) };
    
    if read_result.is_ok() {
        // GOD-MODE: O(1) Instantaneous Hash Map Lookup. 
        // If the execution path exists in our registry, drop the hammer immediately.
        if unsafe { THREAT_REGISTRY.get(&buf).is_some() } {
            
            let _ = unsafe { bpf_send_signal(9) };
            
            let val: u32 = 1;
            let _ = unsafe { ASSASSINATED_PIDS.insert(&pid, &val, 0) };
        }
    }
    
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";