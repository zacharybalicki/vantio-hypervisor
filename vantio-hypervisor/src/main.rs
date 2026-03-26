use aya::programs::KProbe;
use aya::{include_bytes_aligned, Ebpf};
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Lift the memory limits
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };

    // Load the eBPF bytecode
    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/vantio-hypervisor")))?;
    
    let program: &mut KProbe = bpf.program_mut("vantio_phantom_fork").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_execve", 0)?;

    // Connect to the Kernel Map
    let mut lockdown_map: aya::maps::HashMap<_, u32, u32> = aya::maps::HashMap::try_from(bpf.map_mut("LOCKDOWN_MODE").unwrap())?;

    // Ensure the system starts UNLOCKED (0)
    lockdown_map.insert(0, 0, 0)?;

    println!("====================================================");
    println!("[ ∅ VANTIO VECTOR ] HYPERVISOR ONLINE.");
    println!("System is currently UNLOCKED. You can run commands.");
    println!("====================================================");

    // Give you a few seconds to get ready
    sleep(Duration::from_secs(5));
    println!("\n[!] INITIATING GLOBAL LOCKDOWN IN 3 SECONDS...");
    sleep(Duration::from_secs(3));
    
    // THE KILL SWITCH: Flip the map to 1
    lockdown_map.insert(0, 1, 0)?;
    println!("\n[X] ZERO-TRUST LOCKDOWN ACTIVE.");
    println!("[X] THE KERNEL IS NOW OBLITERATING ALL NEW PROCESSES.");
    println!("--> Go to your other terminal and try to run 'ls' or 'pwd' NOW!");

    // Hold the lockdown for 15 seconds so you can test it
    sleep(Duration::from_secs(15));

    // STAND DOWN: Flip the map back to 0
    lockdown_map.insert(0, 0, 0)?;
    println!("\n[O] LOCKDOWN LIFTED. System operations restored.");
    println!("Exiting Vantio Hypervisor...");

    Ok(())
}