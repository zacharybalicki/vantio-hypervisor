#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{bpf_send_signal, bpf_get_current_comm, bpf_get_current_pid_tgid, bpf_get_current_uid_gid},
    macros::{tracepoint, map},
    maps::{PerfEventArray, HashMap},
    programs::TracePointContext,
    EbpfContext,
};
use aya_log_ebpf::info;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VantioEvent {
    pub pid: u32,
    pub uid: u32,
    pub comm: [u8; 16],
}

// THE DYNAMIC THREAT MATRIX
#[map]
static THREAT_MAP: HashMap<[u8; 16], u32> = HashMap::with_max_entries(1024, 0);

#[map]
static VANTIO_EVENTS: PerfEventArray<VantioEvent> = PerfEventArray::new(0);

#[tracepoint]
pub fn vantio_exec(ctx: TracePointContext) -> u32 {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;

    if pid == 0 { return 0; }

    let comm = match bpf_get_current_comm() {
        Ok(c) => c,
        Err(_) => return 0,
    };

    // O(1) Mathematical lookup against the Threat Map
    if unsafe { THREAT_MAP.get(&comm).is_some() } {
        info!(&ctx, "[STASIS TRAP]: Dynamic Threat Detected! PID: {}", pid);
        
        let result = unsafe { bpf_send_signal(19) };
        if result == 0 {
            let uid_gid = bpf_get_current_uid_gid();
            let event = VantioEvent {
                pid: pid,
                uid: uid_gid as u32,
                comm: comm,
            };
            
            VANTIO_EVENTS.output(&ctx, &event, 0);
        }
    }
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
