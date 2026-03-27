use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Ebpf};
use std::collections::HashSet;
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };

    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/vantio-hypervisor")))?;
    
    let program: &mut TracePoint = bpf.program_mut("vantio_phantom_fork").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_execve")?;

    // UPGRADE: Use take_map() to completely bypass Rust's Borrow Checker!
    let mut signature_map: aya::maps::HashMap<_, u32, [u8; 4]> = aya::maps::HashMap::try_from(bpf.take_map("DYNAMIC_SIGNATURE").unwrap())?;
    let assassinated_pids: aya::maps::HashMap<_, u32, u32> = aya::maps::HashMap::try_from(bpf.take_map("ASSASSINATED_PIDS").unwrap())?;
    let mut seen_pids = HashSet::new();

    // ==========================================
    // PHASE 1: INJECT 'wget' SIGNATURE
    // ==========================================
    signature_map.insert(0, *b"wget", 0)?;

    println!("====================================================");
    println!("[ ∅ VANTIO VECTOR ] HOT-SWAP INJECTOR ONLINE.");
    println!("Current Threat Signature: 'wget'");
    println!("--> Go test 'wget' in your other terminal NOW.");
    println!("--> Then wait. A hot-swap will occur in 15 seconds...");
    println!("====================================================");

    for _ in 0..30 {
        for item in assassinated_pids.iter() {
            if let Ok((pid, _)) = item {
                if !seen_pids.contains(&pid) {
                    println!("[ ∅ VANTIO EDR ] 'wget' NEUTRALIZED | PID: {}", pid);
                    seen_pids.insert(pid);
                }
            }
        }
        sleep(Duration::from_millis(500)); 
    }

    // ==========================================
    // PHASE 2: HOT-SWAP TO 'curl' (NO REBOOT REQUIRED)
    // ==========================================
    println!("\n[!] ALERT: ZERO-DAY DETECTED. INITIATING HOT-SWAP...");
    signature_map.insert(0, *b"curl", 0)?;
    println!("[!] NEW THREAT SIGNATURE INJECTED: 'curl'");
    println!("--> Try running 'wget' again (it will now LIVE).");
    println!("--> Now try running 'curl' (it will DIE).");

    loop {
        for item in assassinated_pids.iter() {
            if let Ok((pid, _)) = item {
                if !seen_pids.contains(&pid) {
                    println!("[ ∅ VANTIO EDR ] 'curl' NEUTRALIZED | PID: {}", pid);
                    seen_pids.insert(pid);
                }
            }
        }
        sleep(Duration::from_millis(500)); 
    }
}