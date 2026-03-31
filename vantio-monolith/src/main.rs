use axum::{
    routing::{get, post}, 
    Router, 
    response::IntoResponse,
    extract::DefaultBodyLimit,
};
use axum::http::header::{CONTENT_SECURITY_POLICY, STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    set_header::SetResponseHeaderLayer,
    services::ServeDir,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("[ ∅ VANTIO MONOLITH ] Initializing Planetary Ingestion Matrix...");
    
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ingest", post(ingest_telemetry))
        // Serves the Vantio HTML UI directly to the open internet
        .fallback_service(ServeDir::new("public"))
        // [ ∅ VANTIO AEGIS SHIELD ENFORCEMENT ]
        .layer(DefaultBodyLimit::max(1024 * 1024 * 2))
        .layer(RequestBodyLimitLayer::new(1024 * 1024 * 2))
        .layer(SetResponseHeaderLayer::overriding(X_FRAME_OPTIONS, "DENY".parse().unwrap()))
        .layer(SetResponseHeaderLayer::overriding(X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap()))
        .layer(SetResponseHeaderLayer::overriding(STRICT_TRANSPORT_SECURITY, "max-age=31536000; includeSubDomains; preload".parse().unwrap()))
        .layer(SetResponseHeaderLayer::overriding(CONTENT_SECURITY_POLICY, "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline' fonts.googleapis.com; font-src 'self' fonts.gstatic.com; img-src 'self' data:;".parse().unwrap()))
        .layer(cors);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("[ ∅ VANTIO MONOLITH ] Neural link established. Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    "[ ∅ VANTIO MONOLITH ] SYN-ACK. TrueTime matrix operational.\n"
}

async fn ingest_telemetry(payload: String) -> impl IntoResponse {
    println!("\n[ ∅ VANTIO MONOLITH ] INCOMING TELEMETRY DETECTED:\n>> {}", payload);
    println!("[ ∅ VANTIO MONOLITH ] Preparing for TrueTime Spanner ledger commit...");
    "RECEIPT_ACKNOWLEDGED\n"
}
