use axum::{
    http::{header::CONTENT_TYPE, Method},
};
use std::sync::Arc;
use tokio::net::TcpListener;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod model;
mod views;
mod jwt;
mod auth;

use crate::routes::route;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
            Ok(pool) => {
                println!("✅ Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("❌ Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = route(Arc::new(AppState { db: pool.clone() })).layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}