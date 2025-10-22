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

// ===========================================
// APPLICATION STATE
// ===========================================

#[derive(Clone)]
pub struct AppState {
    pub mongodb_uri: String,
    pub jwt_secret: String,
}

// ===========================================
// ROUTE HANDLERS
// ===========================================

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Bisca API is running",
        "version": "1.0.0"
    }))
}

async fn create_game(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({
        "success": true,
        "game_id": "example-game-id",
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

// ===========================================
// SERVER FUNCTIONS
// ===========================================

pub async fn run_dev_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create application state
    let app_state = AppState {
        mongodb_uri: std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017/biscaDB".to_string()),
        jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-key".to_string()),
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
        
        // Add state and middleware
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(cors));

    // Start the development server (HTTP)
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("🚀 Bisca Server running on http://0.0.0.0:3000");
    println!("📡 Health check: http://localhost:3000/health");
    println!("🎮 Ready to play!");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
    
    Ok(())
}

pub async fn run_https_server() -> Result<(), Box<dyn std::error::Error>> {
    // For now, just run the dev server
    // HTTPS implementation can be added later
    run_dev_server().await
}