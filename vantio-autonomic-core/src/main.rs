use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Fatal: Failed to forge Redpanda producer matrix.");
    let anomaly_payload = "[ ∅ VANTIO ] ANOMALY DETECTED: Rogue Agent contained in Phantom Dimension. zk-SNARK Receipt: 0x8F9B2C...";
    let delivery_status = producer.send(
        FutureRecord::to("vantio-anomaly-stream").payload(anomaly_payload).key("edge-node-alpha"),
        Duration::from_secs(0),
    ).await;
    match delivery_status {
        Ok(_) => println!("[ ∅ VANTIO ] Telemetry successfully committed to the Autonomic Data Core."),
        Err((e, _)) => eprintln!("[ ∅ VANTIO ] FATAL: Streaming timeline severed: {:?}", e),
    }
}
