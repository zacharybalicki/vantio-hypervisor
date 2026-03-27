use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Ebpf};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread::sleep;
use std::fs::OpenOptions;
use std::io::Write;

fn format_rule(cmd: &str) -> [u8; 32] {
    let mut buf = [0u8; 32];
    let bytes = cmd.as_bytes();
    let len = bytes.len().min(31); 
    buf[..len].copy_from_slice(&bytes[..len]);
    buf
}

fn parse_comm(comm: &[u8; 16]) -> String {
    let len = comm.iter().position(|&c| c == 0).unwrap_or(16);
    String::from_utf8_lossy(&comm[..len]).into_owned()
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
    let assassinated_context: aya::maps::HashMap<_, u32, u32> = aya::maps::HashMap::try_from(bpf.take_map("ASSASSINATED_CONTEXT").unwrap())?;
    let ancestry_map: aya::maps::HashMap<_, u32, [u8; 16]> = aya::maps::HashMap::try_from(bpf.take_map("ANCESTRY_MAP").unwrap())?;
    let mut seen_pids = HashSet::new();

    let threats = vec!["/usr/bin/wget", "/usr/bin/curl", "/bin/nc", "/usr/bin/nc"];

    for threat in &threats {
        registry.insert(format_rule(threat), 1, 0)?;
    }

    let log_path = "/tmp/vantio-edr.json";
    println!("====================================================");
    println!("[ ∅ VANTIO VECTOR ] ANCESTRY ENGINE ONLINE.");
    println!("Extracting Ghost Ancestors and Telemetry to: {}", log_path);
    println!("====================================================");

    loop {
        for item in assassinated_context.iter() {
            if let Ok((pid, uid)) = item {
                if !seen_pids.contains(&pid) {
                    
                    let mut ancestor_name = String::from("UNKNOWN");
                    if let Ok(comm_bytes) = ancestry_map.get(&pid, 0) {
                        ancestor_name = parse_comm(&comm_bytes);
                    }

                    let user_context = if uid == 0 { "ROOT" } else { "STANDARD" };
                    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                    println!("[ ∅ VANTIO EDR ] THREAT ASSASSINATED!");
                    println!("    ├─ TRUE PID   : {}", pid);
                    println!("    ├─ ATTACKER   : UID {} [{}]", uid, user_context);
                    println!("    └─ SPAWNED BY : {} (The Ancestor)", ancestor_name);
                    println!("----------------------------------------------------");

                    let json_log = format!(
                        r#"{{"timestamp": {}, "event": "threat_assassinated", "pid": {}, "uid": {}, "context": "{}", "spawned_by": "{}"}}"#,
                        timestamp, pid, uid, user_context, ancestor_name
                    );

                    let mut file = OpenOptions::new().create(true).append(true).open(log_path).expect("Failed to open SIEM log file");
                    writeln!(file, "{}", json_log).expect("Failed to write JSON log");

                    seen_pids.insert(pid);
                }
            }
        }
        sleep(Duration::from_millis(500)); 
    }
}