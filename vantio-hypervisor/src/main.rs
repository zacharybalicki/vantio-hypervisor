use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Ebpf};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::thread::sleep;

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
    
    // 1. Establish the Live Threat Matrix File
    let matrix_path = "./vantio-matrix.txt";
    if !std::path::Path::new(matrix_path).exists() {
        let mut file = std::fs::File::create(matrix_path)?;
        // Adding the WSL specific symlinks for absolute coverage
        writeln!(file, "/bin/nc\n/usr/bin/nc\n/bin/nc.openbsd\n/usr/bin/wget\n/usr/bin/curl")?;
    }

    println!("====================================================");
    println!("[ ∅ VANTIO VECTOR ] ACTIVE POLICY ENGINE ONLINE.");
    println!("Monitoring {} for real-time threat injection.", matrix_path);
    println!("====================================================");

    let mut last_modified = SystemTime::UNIX_EPOCH;
    let mut seen_pids = HashSet::new();
    let log_path = "./vantio-edr.json";

    loop {
        // 2. HOT-SWAP MODULE: Dynamically check the matrix file for updates
        if let Ok(metadata) = fs::metadata(matrix_path) {
            if let Ok(modified) = metadata.modified() {
                if modified > last_modified {
                    last_modified = modified;
                    if let Ok(contents) = fs::read_to_string(matrix_path) {
                        println!("[ + ] Matrix Update Detected. Synchronizing Kernel Ring-0...");
                        for line in contents.lines() {
                            let threat = line.trim();
                            if !threat.is_empty() {
                                let _ = registry.insert(format_rule(threat), 1, 0);
                            }
                        }
                        println!("[ + ] Ring-0 Synchronization Complete.");
                    }
                }
            }
        }

        // 3. TELEMETRY MODULE: Stream to the UI
        for item in assassinated_context.iter() {
            if let Ok((pid, uid)) = item {
                if !seen_pids.contains(&pid) {
                    let mut ancestor_name = String::from("UNKNOWN");
                    if let Ok(comm_bytes) = ancestry_map.get(&pid, 0) { ancestor_name = parse_comm(&comm_bytes); }

                    let user_context = if uid == 0 { "ROOT" } else { "STANDARD" };
                    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                    println!("[ ∅ VANTIO EDR ] THREAT NEUTRALIZED | PID: {} | ANCESTOR: {}", pid, ancestor_name);

                    let json_log = format!(
                        r#"{{"timestamp": {}, "event": "threat_assassinated", "pid": {}, "uid": {}, "context": "{}", "spawned_by": "{}"}}"#,
                        timestamp, pid, uid, user_context, ancestor_name
                    );

                    let mut file = OpenOptions::new().create(true).append(true).open(log_path).expect("Failed to open");
                    writeln!(file, "{}", json_log).expect("Failed to write");
                    
                    // FORCE FLUSH: Bypass RAM buffering and write directly to the hard drive for the UI
                    let _ = file.sync_all(); 

                    seen_pids.insert(pid);
                }
            }
        }
        sleep(Duration::from_millis(500));
    }
}