use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Ebpf};
use std::collections::HashSet;
use std::time::Duration;
use std::thread::sleep;

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
    // Connect to our new Context map
    let assassinated_context: aya::maps::HashMap<_, u32, u32> = aya::maps::HashMap::try_from(bpf.take_map("ASSASSINATED_CONTEXT").unwrap())?;
    let mut seen_pids = HashSet::new();

    let threats = vec!["/usr/bin/wget", "/usr/bin/curl", "/bin/nc"];

    for threat in &threats {
        registry.insert(format_rule(threat), 1, 0)?;
    }

    println!("====================================================");
    println!("[ ∅ VANTIO VECTOR ] CONTEXT ENGINE ONLINE.");
    println!("Scanning for executions and extracting User Attribution...");
    println!("====================================================");

    loop {
        // Sweep the map for PID (Key) and UID (Value)
        for item in assassinated_context.iter() {
            if let Ok((pid, uid)) = item {
                if !seen_pids.contains(&pid) {
                    
                    // Format the UID to be human-readable
                    let user_context = if uid == 0 { "ROOT (CRITICAL THREAT)" } else { "STANDARD USER" };

                    println!("[ ∅ VANTIO EDR ] THREAT ASSASSINATED!");
                    println!("    ├─ TRUE PID : {}", pid);
                    println!("    └─ ATTACKER : UID {} [{}]", uid, user_context);
                    println!("----------------------------------------------------");
                    
                    seen_pids.insert(pid);
                }
            }
        }
        sleep(Duration::from_millis(500)); 
    }
}