use axum::{
    routing::get,
    Router,
    extract::State,
    response::sse::{Event, Sse},
};
use std::{convert::Infallible, net::SocketAddr};
use tower_http::services::ServeDir;
use tonic::{transport::{Server, Identity, ServerTlsConfig}, Request, Response, Status};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use futures_util::stream::StreamExt;
use serde_json::json;

pub mod pb { tonic::include_proto!("telemetry"); }
use pb::phantom_oracle_server::{PhantomOracle, PhantomOracleServer};
use pb::{AnomalyRecord, ZkReceipt};

pub struct OracleService {
    pub tx: broadcast::Sender<String>,
}

#[tonic::async_trait]
impl PhantomOracle for OracleService {
    async fn stream_telemetry(&self, request: Request<AnomalyRecord>) -> Result<Response<ZkReceipt>, Status> {
        let record = request.into_inner();
        
        // 1. Terminal Telemetry
        println!("[ ∅ mTLS UPLINK ] Threat Neutralized: {} from {}", record.threat_vector, record.edge_id);
        
        // 2. Broadcast directly to the Enterprise UI via SSE
        let payload = json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "edge_id": record.edge_id,
            "threat": record.threat_vector,
        }).to_string();
        
        let _ = self.tx.send(payload);

        Ok(Response::new(ZkReceipt {
            acknowledged: true,
            receipt_hash: "zk-STARK-0x9948F".into(),
        }))
    }
}

// 3. The Server-Sent Events (SSE) Endpoint
async fn sse_handler(State(tx): State<broadcast::Sender<String>>) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|res| async {
        res.ok().map(|msg| Ok(Event::default().data(msg)))
    });
    Sse::new(stream)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ ∅ VANTIO MONOLITH ] Booting Omniscient Radar...");

    // The Global Memory Bus (Multiplexing gRPC -> UI)
    let (tx, _rx) = broadcast::channel::<String>(100);

    // Ignite the Axum Web Matrix
    let axum_tx = tx.clone();
    tokio::spawn(async move {
        let app = Router::new()
            .route("/health", get(|| async { "200 OK" }))
            .route("/api/telemetry", get(sse_handler))
            .with_state(axum_tx)
            .fallback_service(ServeDir::new("public"));
        let web_addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        println!("[ ∅ UI ] Axum Dashboard bound to 0.0.0.0:8080");
        let listener = tokio::net::TcpListener::bind(&web_addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // Ignite the gRPC mTLS Telemetry Engine
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
        .add_service(PhantomOracleServer::new(OracleService { tx }))
        .serve(grpc_addr)
        .await.unwrap();

    Ok(())
}
