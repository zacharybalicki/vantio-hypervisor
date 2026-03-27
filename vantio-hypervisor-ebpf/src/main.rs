#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{tracepoint, map},
    maps::HashMap,
    programs::TracePointContext,
    helpers::{bpf_probe_read_user_str_bytes, bpf_send_signal, bpf_get_current_pid_tgid, bpf_get_current_uid_gid},
};

#[map]
static THREAT_REGISTRY: HashMap<[u8; 32], u32> = HashMap::with_max_entries(1024, 0);

// We upgrade our telemetry map. 
// Key: The True PID of the assassinated process.
// Value: The User ID (UID) of the person who tried to run it.
#[map]
static ASSASSINATED_CONTEXT: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[tracepoint]
pub fn vantio_phantom_fork(ctx: TracePointContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    // Extract the User ID from the kernel
    let uid = bpf_get_current_uid_gid() as u32;

    let filename_ptr: *const u8 = unsafe { ctx.read_at::<*const u8>(16) }.unwrap_or(core::ptr::null());
    if filename_ptr.is_null() { return 0; }

    let mut buf = [0u8; 32];
    let read_result = unsafe { bpf_probe_read_user_str_bytes(filename_ptr, &mut buf) };
    
    if read_result.is_ok() {
        if unsafe { THREAT_REGISTRY.get(&buf).is_some() } {
            
            // Drop the hammer
            let _ = unsafe { bpf_send_signal(9) };
            
            // Log the True PID and the User ID to the orchestrator
            let _ = unsafe { ASSASSINATED_CONTEXT.insert(&pid, &uid, 0) };
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