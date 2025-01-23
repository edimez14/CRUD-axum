use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id_user: i32,
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}