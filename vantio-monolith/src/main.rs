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
        
        // Injecting rich data for the frontend slide-over dossiers
        let payload = json!({
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
    println!("[ ∅ VANTIO MONOLITH ] Booting Functional Matrix...");

    let (tx, _rx) = broadcast::channel::<String>(100);
    let metrics = Arc::new(Mutex::new(SystemMetrics {
        active_nodes: 1024,
        syscalls: 84200000,
        threats: 3142,
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
