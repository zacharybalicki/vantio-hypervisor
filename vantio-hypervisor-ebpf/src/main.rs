#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{tracepoint, map},
    maps::HashMap,
    programs::TracePointContext,
    helpers::{bpf_probe_read_user_str_bytes, bpf_send_signal, bpf_get_current_pid_tgid, bpf_get_current_uid_gid, bpf_get_current_comm},
};

#[map]
static THREAT_REGISTRY: HashMap<[u8; 32], u32> = HashMap::with_max_entries(1024, 0);

#[map]
static ASSASSINATED_CONTEXT: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[map]
static ANCESTRY_MAP: HashMap<u32, [u8; 16]> = HashMap::with_max_entries(1024, 0);

#[tracepoint]
pub fn vantio_phantom_fork(ctx: TracePointContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    let uid = bpf_get_current_uid_gid() as u32;

    let filename_ptr: *const u8 = unsafe { ctx.read_at::<*const u8>(16) }.unwrap_or(core::ptr::null());
    if filename_ptr.is_null() { return 0; }

    let mut buf = [0u8; 32];
    let read_result = unsafe { bpf_probe_read_user_str_bytes(filename_ptr, &mut buf) };
    
    if read_result.is_ok() {
        if unsafe { THREAT_REGISTRY.get(&buf).is_some() } {
            
            // 1. Drop the hammer
            let _ = unsafe { bpf_send_signal(9) };
            
            // 2. Extract the Ghost Ancestor using the modern Rust method
            let parent_comm = bpf_get_current_comm().unwrap_or([0u8; 16]);

            // 3. Log all context to user-space
            let _ = unsafe { ASSASSINATED_CONTEXT.insert(&pid, &uid, 0) };
            let _ = unsafe { ANCESTRY_MAP.insert(&pid, &parent_comm, 0) };
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