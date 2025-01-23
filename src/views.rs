use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;
use serde_json::{json};
use serde::{Deserialize};
use sqlx;

use crate::AppState;
use crate::model::{
    User
};

#[derive(Deserialize)]
pub struct NewUser {
    user_name: String,
    email: String,
    password: String,
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn get_users(State(state): State<Arc<AppState>>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>(r#"SELECT * FROM users"#)
    .fetch_all(&state.db).await.unwrap();

    Json(users)
}

pub async fn view_users(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, Json<serde_json::Value>> {

    let result = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE id_user = $1"#,
    ).bind(&id).fetch_one(&state.db).await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Json(json!({ "error": "User not found" }))),
    }
}

pub async fn put_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, Json<serde_json::Value>> {

    let user = sqlx::query_as::<_, User>(
        r#"UPDATE users
           SET user_name = $1, email = $2, password = $3
           WHERE id_user = $4
           RETURNING id_user, user_name, email, password"#
    ).bind(&payload.user_name).bind(&payload.email).bind(&payload.password).bind(&id)
    .fetch_one(&state.db).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Json(json!({ "error": "User not found or update failed" }))),
    }
}

pub async fn delete_users(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Json<serde_json::Value> {
    let result = sqlx::query(
        r#"DELETE FROM users WHERE id_user = $1"#
    ).bind(&id).execute(&state.db).await.unwrap();

    if result.rows_affected() > 0 {
        Json(json!({ "reponse": "User delete" }))
    } else {
        Json(json!({ "error": "User not found" }))
    }
}