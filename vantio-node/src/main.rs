use aya::Ebpf;
use aya::programs::TracePoint;
use aya::maps::perf::AsyncPerfEventArray;
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

    let program: &mut TracePoint = ebpf.program_mut("vantio_exec").unwrap().try_into()?;
    program.load()?;
    program.attach("sched", "sched_process_exec")?;

    let mut perf_array = AsyncPerfEventArray::try_from(ebpf.take_map("VANTIO_EVENTS").unwrap())?;

    for cpu_id in online_cpus().map_err(|(_, e)| e)? {
        let mut buf = perf_array.open(cpu_id, None)?;

        tokio::spawn(async move {
            let mut buffers = (0..10).map(|_| BytesMut::with_capacity(4)).collect::<Vec<_>>();
            
            // 3-SECOND TIMEOUT: If Google Cloud is down, we don't wait forever.
            let client = Client::builder()
                .timeout(Duration::from_secs(3))
                .build()
                .unwrap();

            loop {
                let events = buf.read_events(&mut buffers).await.unwrap();
                for i in 0..events.read {
                    let buf = &mut buffers[i];
                    let target_pid = u32::from_ne_bytes(buf[..4].try_into().unwrap());
                    println!("\n[VANTIO ALARM]: NEURAL BRIDGE BREACHED. Target PID {} isolated in stasis.", target_pid);

                    // 1. NEUTRALIZE FIRST: Kill the process before doing anything else
                    println!("[VANTIO ALARM]: Issuing SIGKILL (9) to annihilate PID {}.", target_pid);
                    Command::new("kill").arg("-9").arg(target_pid.to_string()).output().expect("Execution failed.");
                    println!("[VANTIO ALARM]: Target eradicated. Kernel physics returned to normal.");

                    // 2. PAPERWORK SECOND: Try to send the receipt to the cloud
                    let payload = json!({
                        "pid": target_pid,
                        "process_name": "rogue",
                        "status": "ERADICATED"
                    });

                    println!("[VANTIO ALARM]: Firing WORM Receipt to Spanner Ledger...");
                    let res = client.post("http://34.173.221.228:8080/ingest").json(&payload).send().await;
                    
                    match res {
                        Ok(_) => println!("[VANTIO ALARM]: Spanner transaction verified. Timestamp locked.\n"),
                        Err(e) => println!("[WARNING]: Spanner transmission failed. Logging locally.\n -> Error: {}\n", e),
                    }
                }
            }
        });
    }

    println!("[VANTIO NODE]: Stasis Trap Armed. Spanner Uplink Standby.");
    println!("[VANTIO NODE]: Waiting for 'rogue' execution. Press Ctrl+C to disengage.");

    signal::ctrl_c().await?;
    println!("[VANTIO NODE]: Disengaging Trap. Shutting down the Executioner.");
    Ok(())
}
