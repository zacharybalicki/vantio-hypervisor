use nix::sched::{unshare, CloneFlags};
use nix::unistd::sethostname;
use nix::mount::{mount, MsFlags};
use std::process::{Command, exit};

fn main() {
    println!("[ ∅ VANTIO ORACLE ] Initiating Wave Function Collapse...");
    println!("[ ∅ VANTIO ORACLE ] Forging The Phantom Dimension...");

    // Define the absolute boundaries of the simulation
    let flags = CloneFlags::CLONE_NEWNS  // Isolated Mount points
              | CloneFlags::CLONE_NEWUTS // Isolated Hostname
              | CloneFlags::CLONE_NEWPID // Isolated Process IDs
              | CloneFlags::CLONE_NEWNET; // Isolated Network stack

    // Sever reality via the Kernel
    if let Err(e) = unshare(flags) {
        eprintln!("[ ∅ VANTIO ] FATAL: Reality fracture rejected by Kernel. Error: {}", e);
        exit(1);
    }

    // Mathematically prevent our localized mounts from bleeding into the host
    mount(None::<&str>, "/", None::<&str>, MsFlags::MS_PRIVATE | MsFlags::MS_REC, None::<&str>)
        .expect("Failed to seal the mount namespace");

    // Synthesize a pristine process filesystem for the isolated agent
    mount(Some("proc"), "/proc", Some("proc"), MsFlags::empty(), None::<&str>)
        .expect("Failed to construct the Phantom /proc matrix");

    // Mathematically rename the isolated matrix
    sethostname("phantom-dimension").expect("Failed to mathematically alter the UTS namespace");

    println!("[ ∅ VANTIO ] Reality fractured. Mount, UTS, PID, and Network namespaces severed.");
    println!("[ ∅ VANTIO ] Localized /proc matrix synthesized. Agent execution is now blind.");
    println!("-------------------------------------------------------------------------");
    
    // Spawn a test process inside the vacuum to prove isolation
    let mut child = Command::new("/bin/bash")
        .spawn()
        .expect("Failed to spawn phantom execution shell");

    child.wait().unwrap();
    
    println!("-------------------------------------------------------------------------");
    println!("[ ∅ VANTIO ORACLE ] Phantom execution terminated. Reality restored.");
}
