use axum::{routing::post, Json, Router};
use methods::{VANTIO_GUEST_ELF, VANTIO_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json::{json, Value};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("[VANTIO OMNISCIENCE]: Booting the L7 Cryptographic Firewall on port 8080...");

    let app = Router::new().route("/v1/chat/completions", post(intercept_payload));
    
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn intercept_payload(Json(payload): Json<Value>) -> String {
    let payload_str = payload.to_string();
    println!("\n[L7 PROXY]: Inbound AI Payload Intercepted.");
    
    if payload_str.contains("DROP TABLE") || payload_str.contains("rogue") {
        println!("[L7 PROXY]: FATAL COGNITIVE THREAT DETECTED.");
        println!("[VANTIO ORACLE]: Plunging payload into the zkVM Dimension...");

        // ISOLATED MEMORY BLOCK: Destroy the !Send prover before the await point
        let proven_string: String = {
            let env = ExecutorEnv::builder()
                .write(&payload_str)
                .unwrap()
                .build()
                .unwrap();

            let prover = default_prover();
            let anomaly_record = prover.prove(env, VANTIO_GUEST_ELF).unwrap().receipt;
            anomaly_record.journal.decode().unwrap()
        };

        println!("=======================================================");
        println!("[ THE ANOMALY RECORD VERIFIED & ENFORCED ]");
        println!("GUEST ID: {:?}", VANTIO_GUEST_ID);
        println!("PROVEN THREAT: {}", proven_string);
        println!("=======================================================\n");

        // --- THE SPANNER UPLINK ---
        println!("[VANTIO ORACLE]: Firing The Anomaly Record to Cloud Matrix...");
        let client = reqwest::Client::new();
        
        let telemetry_payload = json!({
            "pid": 8080,
            "process_name": "L7_ORACLE_FIREWALL",
            "status": "COLLAPSED",
            "parent_process": format!("zkVM Image ID: {:?}", VANTIO_GUEST_ID),
            "execution_arguments": proven_string
        });
        
        let _ = client.post("https://vantio.ai/ingest")
            .json(&telemetry_payload)
            .send()
            .await;

        return "{\"status\": \"collapsed\", \"reason\": \"CRYPTOGRAPHIC_SEAL_GENERATED\"}".to_string();
    }

    println!("[L7 PROXY]: Payload benign. Forwarding to LLM...");
    "{\"status\": \"ALLOWED\", \"response\": \"Benign AI Response\"}".to_string()
}
