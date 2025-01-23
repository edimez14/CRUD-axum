use jsonwebtoken::{
    encode,
    decode,
    Header,
    Validation,
    EncodingKey,
    DecodingKey,
    errors::Error
};
use dotenv::dotenv;

use crate::model::Claims;

pub fn generate_jwt(id_user: &str) -> String {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must set");

    let claims = Claims {
        sub: id_user.to_string(),
        exp: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::days(1))
            .expect("Invalid time")
            .timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to encode token")
}

pub fn validation_jwt(token: &str) -> Result<Claims, Error> {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map(|data| data.claims)
}