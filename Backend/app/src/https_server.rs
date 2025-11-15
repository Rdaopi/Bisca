use axum::{
    extract::State,
    http::Method,
    response::Json,
    routing::{get, post},
    Router,
};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde_json::{json, Value};
use std::{
    fs::File,
    io::{BufReader, Error as IoError, ErrorKind},
    sync::Arc,
};
use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tokio_stream::wrappers::TcpListenerStream;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::auth;

// ===========================================
// APPLICATION STATE
// ===========================================

#[derive(Clone, Default)]
pub struct AppState;

// ===========================================
// HTTPS CONFIGURATION
// ===========================================

async fn load_tls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let cert_file = File::open("certs/server.crt")?;
    let key_file = File::open("certs/server.key")?;

    let mut cert_reader = BufReader::new(cert_file);
    let mut key_reader = BufReader::new(key_file);

    let cert_chain = certs(&mut cert_reader)?
        .into_iter()
        .map(Certificate)
        .collect();

    let mut keys = pkcs8_private_keys(&mut key_reader)?;
    let private_key = PrivateKey(keys.remove(0));

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)?;

    Ok(config)
}

// ===========================================
// ROUTE HANDLERS
// ===========================================

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Bisca API is running",
    }))
}

async fn create_game(State(_state): State<AppState>) -> Json<Value> {
    let game_id = uuid::Uuid::new_v4().to_string();

    Json(json!({
        "success": true,
        "game_id": game_id,
        "message": "Game created successfully"
    }))
}

async fn join_game(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Joined game successfully"
    }))
}

async fn play_card(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Card played successfully"
    }))
}

async fn make_prediction(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Prediction made successfully"
    }))
}

fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .route("/health", get(health_check))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .route("/games", post(create_game))
        .route("/games/:game_id/join", post(join_game))
        .route("/games/:game_id/play-card", post(play_card))
        .route("/games/:game_id/prediction", post(make_prediction))
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors))
}

// ===========================================
// HTTPS SERVER
// ===========================================

pub async fn run_https_server() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let tls_config = load_tls_config().await?;
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let state = AppState::default();
    let router = build_router(state);
    let service = router.into_make_service();

    let listener = TcpListener::bind("0.0.0.0:443").await?;
    let incoming = TcpListenerStream::new(listener);

    println!("[https] server starting on https://0.0.0.0:443");

    let tls_acceptor = tls_acceptor.clone();
    let tls_incoming = incoming
        .filter_map(move |conn| {
            let tls_acceptor = tls_acceptor.clone();
            async move {
                match conn {
                    Ok(stream) => match tls_acceptor.accept(stream).await {
                        Ok(tls_stream) => Some(Ok(tls_stream)),
                        Err(err) => {
                            eprintln!("[https] TLS handshake failed: {err}");
                            None
                        }
                    },
                    Err(err) => {
                        eprintln!("[https] TCP accept error: {err}");
                        None
                    }
                }
            }
        })
        .map(|stream| stream.map_err(|err| IoError::new(ErrorKind::Other, err)));

    axum::serve(tls_incoming, service)
        .await
        .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

    Ok(())
}

// ===========================================
// DEVELOPMENT SERVER (HTTP)
// ===========================================

pub async fn run_dev_server() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = build_router(AppState::default());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("[dev] server running on http://0.0.0.0:3000");
    println!("[dev] WARNING: development server uses HTTP only");

    axum::serve(listener, app)
        .await
        .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;

    Ok(())
}
