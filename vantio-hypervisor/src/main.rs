use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Ebpf};
use std::collections::HashSet;
use std::time::Duration;
use std::thread::sleep;

// Helper function to format our strings into exactly 32 zero-padded bytes
// so they perfectly match the kernel's memory map keys.
fn format_rule(cmd: &str) -> [u8; 32] {
    let mut buf = [0u8; 32];
    let bytes = cmd.as_bytes();
    let len = bytes.len().min(31); 
    buf[..len].copy_from_slice(&bytes[..len]);
    buf
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let rlim = libc::rlimit { rlim_cur: libc::RLIM_INFINITY, rlim_max: libc::RLIM_INFINITY };
    unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };

    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/vantio-hypervisor")))?;
    
    let program: &mut TracePoint = bpf.program_mut("vantio_phantom_fork").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_execve")?;

    let mut registry: aya::maps::HashMap<_, [u8; 32], u32> = aya::maps::HashMap::try_from(bpf.take_map("THREAT_REGISTRY").unwrap())?;
    let assassinated_pids: aya::maps::HashMap<_, u32, u32> = aya::maps::HashMap::try_from(bpf.take_map("ASSASSINATED_PIDS").unwrap())?;
    let mut seen_pids = HashSet::new();

    // ==========================================
    // THE THREAT REGISTRY PAYLOAD
    // Execve usually resolves the absolute path of the binary, so we map the paths.
    // ==========================================
    let threats = vec![
        "/usr/bin/wget", 
        "/usr/bin/curl", 
        "/usr/bin/nmap",
        "/bin/nc" // Netcat (hacker reverse-shell tool)
    ];

    // Inject all rules into the kernel instantly
    for threat in &threats {
        registry.insert(format_rule(threat), 1, 0)?;
    }

    println!("====================================================");
    println!("[ ∅ VANTIO VECTOR ] O(1) MULTI-THREAT ARRAY ONLINE.");
    println!("Currently enforcing {} Zero-Trust execution rules:", threats.len());
    for t in &threats {
        println!("  -> [BLOCKED] {}", t);
    }
    println!("====================================================");

    loop {
        for item in assassinated_pids.iter() {
            if let Ok((pid, _)) = item {
                if !seen_pids.contains(&pid) {
                    println!("[ ∅ VANTIO EDR ] TARGET ASSASSINATED | LETHAL FORCE ON PID: {}", pid);
                    seen_pids.insert(pid);
                }
            }
        }
        sleep(Duration::from_millis(500)); 
    }
}