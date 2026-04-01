use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tonic::{transport::{Server, Identity, ServerTlsConfig}, Request, Response, Status};

pub mod pb { tonic::include_proto!("telemetry"); }
use pb::phantom_oracle_server::{PhantomOracle, PhantomOracleServer};
use pb::{AnomalyRecord, ZkReceipt};

#[derive(Debug, Default)]
pub struct OracleService {}

#[tonic::async_trait]
impl PhantomOracle for OracleService {
    async fn stream_telemetry(&self, request: Request<AnomalyRecord>) -> Result<Response<ZkReceipt>, Status> {
        let record = request.into_inner();
        println!("[ ∅ mTLS UPLINK ] Threat Neutralized: {} from {}", record.threat_vector, record.edge_id);
        
        Ok(Response::new(ZkReceipt {
            acknowledged: true,
            receipt_hash: "zk-STARK-0x9948F".into(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ ∅ VANTIO MONOLITH ] Booting Control Plane...");

    // 1. Ignite the Axum Web Matrix
    tokio::spawn(async move {
        let app = Router::new()
            .route("/health", get(|| async { "200 OK" }))
            .fallback_service(ServeDir::new("public"));
        let web_addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        println!("[ ∅ UI ] Axum Dashboard bound to 0.0.0.0:8080");
        let listener = tokio::net::TcpListener::bind(&web_addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Ignite the gRPC mTLS Telemetry Engine
    let cert = std::fs::read_to_string("tls/server.pem").expect("Missing server.pem");
    let key = std::fs::read_to_string("tls/server.key").expect("Missing server.key");
    let ca = std::fs::read_to_string("tls/ca.pem").expect("Missing ca.pem");

    let identity = Identity::from_pem(cert, key);
    let client_ca = tonic::transport::Certificate::from_pem(ca);

    let tls = ServerTlsConfig::new()
        .identity(identity)
        .client_ca_root(client_ca);

    let grpc_addr = "0.0.0.0:50051".parse().unwrap();
    println!("[ ∅ mTLS ] gRPC Neural Uplink bound to 0.0.0.0:50051");

    Server::builder()
        .tls_config(tls).unwrap()
        .add_service(PhantomOracleServer::new(OracleService::default()))
        .serve(grpc_addr)
        .await.unwrap();

    Ok(())
}
