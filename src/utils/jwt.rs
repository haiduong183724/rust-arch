use actix_web::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    iat: i64,
    exp: i64,
    email: String,
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::hours(24);
    let claim = Claim {
        iat: now.timestamp(),
        exp: (now + expire).timestamp() as i64,
        email,
    };
    let secret = std::env::var("SECRET_KEY").unwrap();
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(token: &str) -> Result<Claim, StatusCode> {
    let secret = std::env::var("SECRET_KEY").unwrap();
    jsonwebtoken::decode::<Claim>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| StatusCode::UNAUTHORIZED)
}
