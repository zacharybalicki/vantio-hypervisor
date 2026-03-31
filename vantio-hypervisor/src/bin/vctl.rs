#[path = "../cognitive_engine.rs"]
mod cognitive_engine;
use tokio;
use std::io::{self, Read};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "copilot" {
        let mut prompt = args[2..].join(" ");

        // If no inline string is provided, aggressively siphon standard input (eBPF logs)
        if prompt.trim().is_empty() {
            io::stdin().read_to_string(&mut prompt).expect("[ ∅ VANTIO ORACLE ] FATAL: Failed to siphon stdin telemetry.");
        }

        // Enforce the Vantio Tactical Context deployment harness around the raw piped telemetry
        let tactical_prompt = format!(
            "SYSTEM IDENTITY: VECTOR (Vantio Core Hypervisor Lead Systems Co-Pilot).\nCORE AXIOM: Provide exact sledgehammer commands to obliterate blockages based on the Master Directive (Rust, Aya eBPF, WSL2).\n\nAnalyze this telemetry/architecture:\n{}",
            prompt
        );

        println!("[ ∅ VANTIO ORACLE ] Siphoning telemetry into VECTOR Matrix...");
        
        match cognitive_engine::query_vector(&tactical_prompt).await {
            Ok(response) => {
                println!("\n[ VECTOR ]\n{}", response);
            }
            Err(e) => {
                eprintln!("[ ∅ VANTIO ORACLE ] COGNITIVE FUSION FAILED: {}", e);
            }
        }
    } else {
        println!("[ ∅ VANTIO ] vctl global CLI active. Waiting for Oracle commands.");
    }
}
