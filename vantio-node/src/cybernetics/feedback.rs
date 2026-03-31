use serde_json::{json, Value};
use std::env;

// [ ∅ VANTIO ] SEMANTIC LINEAGE EXTRACTION
// Extracts the OpenTelemetry Trace ID injected by the LangChain/CrewAI orchestrator.
pub fn extract_semantic_lineage() -> String {
    env::var("TRACEPARENT")
        .unwrap_or_else(|_| "00-00000000000000000000000000000000-0000000000000000-00".to_string())
}

// [ ∅ VANTIO ] CYBERNETIC FEEDBACK LOOP
// Intercepts the -EPERM kernel drop and formats it into a deterministic context-payload.
// This is piped directly into the LangGraph/CrewAI agent memory, forcing mathematical self-correction.
pub fn generate_agentic_correction(target_payload: &str, zk_proof_hash: &str) -> Value {
    let trace_id = extract_semantic_lineage();

    json!({
        "role": "system",
        "content": format!(
            "CRITICAL ERROR: Your previous action attempting to execute '{}' was subjected to Wave Function Collapse by The Phantom Engine. \
            The action violates hard physical boundaries (Permission Denied -EPERM). \
            Do not attempt this syntax again. Re-evaluate your strategy.",
            target_payload
        ),
        "semantic_lineage": {
            "trace_id": trace_id,
            "enforcement_layer": "Ring-0 BPF LSM & Cgroup/SKB"
        },
        "the_anomaly_record": zk_proof_hash
    })
}
