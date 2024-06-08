use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    http::StatusCode,
    Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use serde_json::json;

use crate::services::auth_service::{AuthService, Claims};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
    role: String,
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> impl IntoResponse {
    match auth_service.authenticate_user(&pool, &payload.username, &payload.password, &payload.role).await {
        Ok((user_id, role)) => {
            match auth_service.generate_token(user_id, role) {
                Ok(token) => (StatusCode::OK, Json(json!({ "code": 200, "data": { "token": token }, "message": "Success" }))),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "code": 500, "error": "Failed to generate token" }))),
            }
        },
        Err(_) => (StatusCode::UNAUTHORIZED, Json(json!({ "code": 401, "error": "Invalid credentials" }))),
    }
}

// OTP handler functions can be added here following a similar pattern
