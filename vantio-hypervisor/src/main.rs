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
    
    // Attach directly to the kernel's native execution tracepoint
    program.attach("syscalls", "sys_enter_execve")?;

    let assassinated_pids: aya::maps::HashMap<_, u32, u32> = aya::maps::HashMap::try_from(bpf.map_mut("ASSASSINATED_PIDS").unwrap())?;
    let mut seen_pids = HashSet::new();

    println!("====================================================");
    println!("[ ∅ VANTIO VECTOR ] ADVANCED TRACEPOINT SNIPER ONLINE.");
    println!("Rules of Engagement:");
    println!("- Standard commands ('ls', 'pwd') will PASS.");
    println!("- The forbidden payload ('wget') will be ASSASSINATED.");
    println!("====================================================");

    loop {
        for item in assassinated_pids.iter() {
            if let Ok((pid, _)) = item {
                if !seen_pids.contains(&pid) {
                    println!("[ ∅ VANTIO EDR ] THREAT NEUTRALIZED | LETHAL FORCE APPLIED TO PID: {}", pid);
                    seen_pids.insert(pid);
                }
            }
        }
        sleep(Duration::from_millis(500)); 
    }
}