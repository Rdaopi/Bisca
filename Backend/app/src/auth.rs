use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::https_server::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

pub async fn login(
    State(_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<AuthResponse>) {
    // TODO: replace with real authentication logic
    let response = AuthResponse {
        success: true,
        message: format!("Login effettuato per {}", payload.email),
        token: Some("token_fittizio".to_string()),
    };
    (StatusCode::OK, Json(response))
}

pub async fn register(
    State(_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<AuthResponse>) {
    // TODO: replace with real registration logic
    let response = AuthResponse {
        success: true,
        message: format!("Registrazione completata per {}", payload.username),
        token: None,
    };
    (StatusCode::CREATED, Json(response))
}
