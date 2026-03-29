use anyhow::Context;
use aya::programs::KProbe;
use aya::Ebpf;
use aya_log::EbpfLogger;
use log::{info, warn};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the telemetry matrix
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("[ ∅ VANTIO ] Initializing Hypervisor Daemon...");

    // Target the compiled Kernel Anchor artifact via ABSOLUTE vector
    let bpf_path = "/home/zachary/src/vantio-hypervisor/target/bpfel-unknown-none/release/vantio-hypervisor";
    info!("[ ∅ VANTIO ] Locating Kernel Anchor at: {}", bpf_path);
    
    let mut bpf = Ebpf::load_file(bpf_path)
        .context("[ ∅ VANTIO FATAL ] Failed to load the eBPF bytecode. Is the artifact compiled?")?;

    // Initialize the kernel-to-user log pipe
    if let Err(e) = EbpfLogger::init(&mut bpf) {
        warn!("[ ∅ VANTIO ] Failed to initialize eBPF logger: {}", e);
    }

    // Extract the kprobe and attach it to the Linux process creation syscall
    let program: &mut KProbe = bpf.program_mut("vantio_clone").unwrap().try_into()?;
    program.load()?;
    
    // Modern x86_64 Linux kernels prefix syscalls with __x64_sys_
    program.attach("__x64_sys_clone", 0)
        .context("[ ∅ VANTIO FATAL ] Failed to attach to __x64_sys_clone. Ensure WSL2 kernel supports kprobes.")?;

    info!("[ ∅ VANTIO DAEMON ] PHANTOM ANCHOR DEPLOYED. Intercepting sys_clone matrix.");
    info!("[ ∅ VANTIO DAEMON ] Awaiting entity spawn events. Press Ctrl-C to abort timeline.");

    // Hold the dimension open until the Operator commands a halt
    signal::ctrl_c().await?;
    info!("[ ∅ VANTIO DAEMON ] Timeline aborted. Detaching Anchor...");

    Ok(())
}
