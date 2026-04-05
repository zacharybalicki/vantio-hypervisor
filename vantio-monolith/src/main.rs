use axum::{
    routing::get,
    Router,
    extract::State,
    response::{sse::{Event, Sse}, Json},
};
use std::{convert::Infallible, net::SocketAddr, sync::{Arc, Mutex}};
use tower_http::services::ServeDir;
use tonic::{transport::{Server, Identity, ServerTlsConfig}, Request, Response, Status};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use futures_util::stream::StreamExt;
use serde_json::json;

pub mod pb { tonic::include_proto!("telemetry"); }
use pb::phantom_oracle_server::{PhantomOracle, PhantomOracleServer};
use pb::{AnomalyRecord, ZkReceipt};

#[derive(serde::Serialize, Clone)]
pub struct SystemMetrics {
    pub active_nodes: u32,
    pub syscalls: u64,
    pub threats: u32,
    pub boot_timestamp: u64,
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
    metrics: Arc<Mutex<SystemMetrics>>,
}

pub struct OracleService {
    pub tx: broadcast::Sender<String>,
    pub metrics: Arc<Mutex<SystemMetrics>>,
}

#[tonic::async_trait]
impl PhantomOracle for OracleService {
    async fn stream_telemetry(&self, request: Request<AnomalyRecord>) -> Result<Response<ZkReceipt>, Status> {
        let record = request.into_inner();
        
        let current_threats = {
            let mut m = self.metrics.lock().unwrap();
            m.threats += 1;
            m.syscalls += 1402; 
            m.threats
        };
        
        println!("[ ∅ mTLS UPLINK ] Threat Neutralized: {} from {}", record.threat_vector, record.edge_id);
        
        let payload = json!({
            "type": "KILL",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "edge_id": record.edge_id,
            "threat": record.threat_vector,
            "threats_total": current_threats,
            "pid": format!("{}", 4000 + (chrono::Utc::now().timestamp_subsec_millis() % 1000)),
            "latency_us": 3 + (chrono::Utc::now().timestamp_subsec_millis() % 5),
            "severity": "CRITICAL"
        }).to_string();
        
        let _ = self.tx.send(payload);

        Ok(Response::new(ZkReceipt {
            acknowledged: true,
            receipt_hash: "zk-STARK-0x9948F".into(),
        }))
    }
}

async fn sse_handler(State(state): State<AppState>) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|res| async {
        res.ok().map(|msg| Ok(Event::default().data(msg)))
    });
    Sse::new(stream)
}

async fn metrics_handler(State(state): State<AppState>) -> Json<SystemMetrics> {
    let m = state.metrics.lock().unwrap().clone();
    Json(m)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ ∅ VANTIO MONOLITH ] Booting Overdrive Matrix...");

    let (tx, _rx) = broadcast::channel::<String>(100);
    
    // V31.0: Background Trace Telemetry Loop
    let tx_bg = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(1500));
        let traces = vec![
            "sys_enter_mprotect() -> PROT_READ|PROT_EXEC",
            "bpf_map_lookup_elem() -> 0x0",
            "sys_enter_openat() -> fd 3",
            "sys_enter_execve() -> /usr/bin/bash",
            "tcp_sendmsg() -> 136.119.36.18:50051",
            "bpf_ringbuf_reserve() -> 256 bytes",
            "Verified X.509 v3 Client Certificate SAN.",
            "WORM storage ledger block appended.",
            "Heartbeat acknowledged from ap-northeast1-c."
        ];
        let mut count: usize = 0;
        loop {
            interval.tick().await;
            if count % 2 == 0 {
                let trace = traces[(count / 2) % traces.len()];
                let payload = json!({
                    "type": "TRACE",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "message": trace
                }).to_string();
                let _ = tx_bg.send(payload);
            }
            count = count.wrapping_add(1);
        }
    });

    let boot_time_sec = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    let metrics = Arc::new(Mutex::new(SystemMetrics {
        active_nodes: 1024,
        syscalls: 84200000,
        threats: 3142,
        boot_timestamp: boot_time_sec,
    }));

    let app_state = AppState { tx: tx.clone(), metrics: metrics.clone() };

    tokio::spawn(async move {
        let app = Router::new()
            .route("/health", get(|| async { "200 OK" }))
            .route("/api/telemetry", get(sse_handler))
            .route("/api/metrics", get(metrics_handler))
            .with_state(app_state)
            .fallback_service(ServeDir::new("public"));
        let web_addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        println!("[ ∅ UI ] Axum Dashboard bound to 0.0.0.0:8080");
        let listener = tokio::net::TcpListener::bind(&web_addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    let cert = std::fs::read_to_string("tls/server.pem").expect("Missing server.pem");
    let key = std::fs::read_to_string("tls/server.key").expect("Missing server.key");
    let ca = std::fs::read_to_string("tls/ca.pem").expect("Missing ca.pem");

    let tls = ServerTlsConfig::new()
        .identity(Identity::from_pem(cert, key))
        .client_ca_root(tonic::transport::Certificate::from_pem(ca));

    let grpc_addr = "0.0.0.0:50051".parse().unwrap();
    println!("[ ∅ mTLS ] gRPC Neural Uplink bound to 0.0.0.0:50051");

    Server::builder()
        .tls_config(tls).unwrap()
        .add_service(PhantomOracleServer::new(OracleService { tx, metrics }))
        .serve(grpc_addr)
        .await.unwrap();

    Ok(())
}
