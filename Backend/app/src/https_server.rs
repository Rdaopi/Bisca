use axum::{
    extract::State,
    http::Method,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use rustls::{ServerConfig, Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::io::BufReader;
use std::fs::File;

mod models;
mod sse;

use sse::SseManager;

// ===========================================
// APPLICATION STATE
// ===========================================

#[derive(Clone)]
pub struct AppState {
    pub sse_manager: Arc<SseManager>,
    // Add your database connection and other state here
}

// ===========================================
// HTTPS CONFIGURATION
// ===========================================

async fn load_tls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    // Load certificate and private key
    let cert_file = File::open("certs/server.crt")?;
    let key_file = File::open("certs/server.key")?;
    
    let mut cert_reader = BufReader::new(cert_file);
    let mut key_reader = BufReader::new(key_file);
    
    // Parse certificates
    let cert_chain = certs(&mut cert_reader)?
        .into_iter()
        .map(Certificate)
        .collect();
    
    // Parse private key
    let mut keys = pkcs8_private_keys(&mut key_reader)?;
    let private_key = PrivateKey(keys.remove(0));
    
    // Create server config
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)?;
    
    Ok(config)
}

// ===========================================
// ROUTE HANDLERS (Same as before)
// ===========================================

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Rate Your Friends API is running (HTTPS)",
        "secure": true
    }))
}

async fn create_game(State(state): State<AppState>) -> Json<Value> {
    let game_id = uuid::Uuid::new_v4().to_string();
    
    Json(json!({
        "success": true,
        "game_id": game_id,
        "message": "Game created successfully",
        "secure": true
    }))
}

async fn join_game(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Joined game successfully",
        "secure": true
    }))
}

async fn play_card(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Card played successfully",
        "secure": true
    }))
}

async fn make_prediction(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Prediction made successfully",
        "secure": true
    }))
}

// ===========================================
// HTTPS SERVER
// ===========================================

pub async fn run_https_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load TLS configuration
    let tls_config = load_tls_config().await?;
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));

    // Initialize SSE manager
    let sse_manager = Arc::new(SseManager::new());

    // Create application state
    let app_state = AppState {
        sse_manager,
    };

    // Configure CORS for HTTPS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_credentials(true); // Important for HTTPS

    // Create the router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Game management
        .route("/games", post(create_game))
        .route("/games/:game_id/join", post(join_game))
        
        // Game actions
        .route("/games/:game_id/play-card", post(play_card))
        .route("/games/:game_id/prediction", post(make_prediction))
        
        // SSE endpoint for real-time updates
        .route("/games/:game_id/events", get(sse::game_events_sse))
        
        // Add state and middleware
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(cors));

    // Start the HTTPS server
    let listener = TcpListener::bind("0.0.0.0:443").await?;
    
    println!("🔐 HTTPS Server starting...");
    println!("📡 SSE endpoint: https://localhost:443/games/{{game_id}}/events");
    println!("🔒 All connections are encrypted with TLS");
    
    // Accept connections and handle TLS
    loop {
        let (stream, addr) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();
        let app = app.clone();
        
        tokio::spawn(async move {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => {
                    println!("🔐 Secure connection from: {}", addr);
                    axum::serve::IncomingStream::from_stream(tls_stream)
                        .serve(app.into_make_service())
                        .await
                        .unwrap_or_else(|e| eprintln!("❌ Connection error: {}", e));
                }
                Err(e) => eprintln!("❌ TLS handshake failed: {}", e),
            }
        });
    }
}

// ===========================================
// DEVELOPMENT SERVER (HTTP)
// ===========================================

pub async fn run_dev_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize SSE manager
    let sse_manager = Arc::new(SseManager::new());

    // Create application state
    let app_state = AppState {
        sse_manager,
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    // Create the router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Game management
        .route("/games", post(create_game))
        .route("/games/:game_id/join", post(join_game))
        
        // Game actions
        .route("/games/:game_id/play-card", post(play_card))
        .route("/games/:game_id/prediction", post(make_prediction))
        
        // SSE endpoint for real-time updates
        .route("/games/:game_id/events", get(sse::game_events_sse))
        
        // Add state and middleware
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(cors));

    // Start the development server (HTTP)
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("🚀 Development server running on http://0.0.0.0:3000");
    println!("⚠️  WARNING: This is HTTP only - not secure for production!");
    println!("📡 SSE endpoint: http://localhost:3000/games/{{game_id}}/events");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
    
    Ok(())
}
