#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{bpf_send_signal, bpf_get_current_comm},
    macros::{tracepoint, map},
    maps::PerfEventArray,
    programs::TracePointContext,
    EbpfContext,
};
use aya_log_ebpf::info;

// THE BRIDGE: Updated to modern Aya syntax
#[map]
static VANTIO_EVENTS: PerfEventArray<u32> = PerfEventArray::new(0);

#[tracepoint]
pub fn vantio_exec(ctx: TracePointContext) -> u32 {
    let pid = ctx.pid();
    if pid == 0 { return 0; }

    let comm = match bpf_get_current_comm() {
        Ok(c) => c,
        Err(_) => return 0,
    };

    let target = b"rogue\0\0\0\0\0\0\0\0\0\0\0";
    
    if comm == *target {
        info!(&ctx, "[STASIS TRAP]: Rogue Execution Detected! PID: {}", pid);
        
        let result = unsafe { bpf_send_signal(19) };
        if result == 0 {
            info!(&ctx, "[STASIS TRAP]: PID {} isolated. Firing across Neural Bridge...", pid);
            VANTIO_EVENTS.output(&ctx, &pid, 0);
        }
    }
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
