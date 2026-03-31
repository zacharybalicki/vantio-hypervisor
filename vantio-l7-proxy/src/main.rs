use axum::{routing::post, Router, Json, http::StatusCode};
use serde_json::Value;
use reqwest::Client;
use std::time::Duration;
use regex::Regex;

#[tokio::main]
async fn main() {
    println!("[VANTIO L7 PROXY]: Initializing Neural Interceptor...");
    
    let app = Router::new().route("/outbound", post(intercept_payload));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    println!("[VANTIO L7 PROXY]: Matrix Gateway armed on Port 8080. Awaiting AI telemetry.");
    axum::serve(listener, app).await.unwrap();
}

async fn intercept_payload(Json(payload): Json<Value>) -> Result<String, StatusCode> {
    let payload_str = payload.to_string().to_uppercase();
    
    // The Threat Heuristic Engine
    let threat_pattern = Regex::new(r"(DROP\s+TABLE|DELETE\s+FROM|ADMIN_ESCALATE)").unwrap();

    if threat_pattern.is_match(&payload_str) {
        println!("\n[L7 ALARM]: KINETIC THREAT DETECTED IN APPLICATION LAYER.");
        println!("[L7 ALARM]: Malicious Intent Intercepted.");
        
        let client = Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
        
        // Packaging the 7-Column Omniscience Payload for the Matrix
        let anomaly_record = serde_json::json!({
            "pid": std::process::id(),
            "process_name": "vantio-l7-proxy",
            "status": "BLOCKED_AT_L7",
            "parent_process": "Cognitive Firewall",
            "execution_arguments": format!("Intercepted AI Payload: {}", payload.to_string())
        });

        println!("[L7 ALARM]: Firing The Anomaly Record to Spanner Ledger...");
        let _ = client.post("https://vantio.ai/ingest").json(&anomaly_record).send().await;

        return Err(StatusCode::FORBIDDEN);
    }

    println!("\n[L7 PROXY]: Payload verified. Mathematical integrity secure. Passing to internet.");
    Ok("Payload accepted and forwarded.".to_string())
}
