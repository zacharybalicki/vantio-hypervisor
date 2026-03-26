#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{kprobe, map},
    maps::HashMap,
    programs::ProbeContext,
    helpers::bpf_send_signal,
};

// The control panel. Key 0 will hold our toggle (0 = Open, 1 = Lockdown)
#[map]
static LOCKDOWN_MODE: HashMap<u32, u32> = HashMap::with_max_entries(1, 0);

#[kprobe]
pub fn vantio_phantom_fork(_ctx: ProbeContext) -> u32 {
    let key: u32 = 0;
    
    // Read the switch from the map
    if let Some(&mode) = unsafe { LOCKDOWN_MODE.get(&key) } {
        if mode == 1 {
            // LOCKDOWN IS ACTIVE. 
            // 9 is the universal Linux code for SIGKILL (Instant Death).
            // Destroy the process before it can even spawn.
            let _ = unsafe { bpf_send_signal(9) };
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