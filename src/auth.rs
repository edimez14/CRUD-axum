use axum::{
    extract::{Json, State},
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[allow(unused_imports)]
use sqlx::Row;

use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    jwt::{generate_jwt, validation_jwt},
    model::User,
};


#[derive(Deserialize)]
pub struct NewUser {
    user_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> Result<Json<TokenResponse>, Json<serde_json::Value>> {
    let user = sqlx::query_as::<_, User>(
        r#"INSERT INTO users (user_name, email, password) VALUES ($1, $2, $3) RETURNING id_user, user_name, email, password"#
    ).bind(&payload.user_name).bind(&payload.email).bind(&payload.password)
    .fetch_one(&state.db).await;

    let token = generate_jwt(&user.unwrap().id_user.to_string());

    Ok(Json(TokenResponse { token }))
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE email = $1"#
    ).bind(&payload.email)
    .fetch_optional(&state.db).await;

    match result {
        Ok(Some(user)) => {
            if user.password == payload.password {
                let token = generate_jwt(&user.id_user.to_string());
                Ok(Json(TokenResponse { token }))
            } else {
                Err(Json(json!({ "error": "Invalid password" })))
            }
        }
        Ok(_) => Err(Json(json!({ "error": "User not found" }))),
        Err(_) => Err(Json(json!({ "error": "Database error" }))),  
    }
}

pub async fn auth_middleware(req: Request<axum::body::Body>, next: Next) -> Result<Response, Response> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match validation_jwt(token) {
                    Ok(_) => return Ok(next.run(req).await),
                    Err(_) => return Err(unauthorized_response()),
                }
            }
        }
    }
    Err(unauthorized_response())
}

fn unauthorized_response() -> Response {
    (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}