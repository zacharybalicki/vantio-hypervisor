use aya::Ebpf;
use aya::programs::TracePoint;
use aya::maps::{perf::AsyncPerfEventArray, HashMap};
use aya::util::online_cpus;
use aya_log::EbpfLogger;
use bytes::BytesMut;
use reqwest::Client;
use serde_json::json;
use std::process::Command;
use std::time::Duration;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    println!("[VANTIO NODE]: Booting the Executioner Matrix...");

    let mut ebpf = Ebpf::load_file("./vantio-ebpf-payload.elf")?;
    if let Err(e) = EbpfLogger::init(&mut ebpf) {
        println!("[WARNING]: Logger init failed: {}", e);
    }

    // 1. POPULATE THE KERNEL THREAT MATRIX
    let mut threat_map: HashMap<_, [u8; 16], u32> = HashMap::try_from(ebpf.map_mut("THREAT_MAP").unwrap())?;
    
    let mut rogue_bytes = [0u8; 16];
    let target_str = b"rogue";
    rogue_bytes[..target_str.len()].copy_from_slice(target_str);
    threat_map.insert(rogue_bytes, 1, 0)?;

    let mut nmap_bytes = [0u8; 16];
    let nmap_str = b"nmap";
    nmap_bytes[..nmap_str.len()].copy_from_slice(nmap_str);
    threat_map.insert(nmap_bytes, 1, 0)?;

    println!("[VANTIO NODE]: Threat Matrix pushed to Ring-0 (Targets: 'rogue', 'nmap').");

    let program: &mut TracePoint = ebpf.program_mut("vantio_exec").unwrap().try_into()?;
    program.load()?;
    program.attach("sched", "sched_process_exec")?;

    let mut perf_array = AsyncPerfEventArray::try_from(ebpf.take_map("VANTIO_EVENTS").unwrap())?;

    for cpu_id in online_cpus().map_err(|(_, e)| e)? {
        let mut buf = perf_array.open(cpu_id, None)?;

        tokio::spawn(async move {
            let mut buffers = (0..10).map(|_| BytesMut::with_capacity(24)).collect::<Vec<_>>();
            let client = Client::builder().timeout(Duration::from_secs(3)).build().unwrap();

            loop {
                let events = buf.read_events(&mut buffers).await.unwrap();
                for i in 0..events.read {
                    let buf = &mut buffers[i];
                    
                    let target_pid = u32::from_ne_bytes(buf[0..4].try_into().unwrap());
                    let target_uid = u32::from_ne_bytes(buf[4..8].try_into().unwrap());
                    let process_name = String::from_utf8_lossy(&buf[8..24]).trim_matches('\0').to_string();

                    println!("\n[VANTIO ALARM]: NEURAL BRIDGE BREACHED. Target PID {} isolated.", target_pid);
                    println!("[VANTIO ALARM]: Issuing SIGKILL (9) to annihilate PID {}.", target_pid);
                    Command::new("kill").arg("-9").arg(target_pid.to_string()).output().expect("Execution failed.");
                    
                    let payload = json!({
                        "pid": target_pid,
                        "process_name": process_name,
                        "status": "ERADICATED",
                        "parent_process": format!("Context UID: {}", target_uid),
                        "execution_arguments": format!("Dynamic eBPF Target: {}", process_name)
                    });

                    println!("[VANTIO ALARM]: Firing WORM Receipt to Spanner Ledger...");
                    let _ = client.post("https://vantio.ai/ingest").json(&payload).send().await;
                }
            }
        });
    }

    println!("[VANTIO NODE]: Stasis Trap Armed. Spanner Uplink Standby.");
    signal::ctrl_c().await?;
    println!("[VANTIO NODE]: Disengaging Trap. Shutting down the Executioner.");
    Ok(())
}
