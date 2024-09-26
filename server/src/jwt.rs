use axum_extra::extract::CookieJar;
use crate::random_string;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: u64,
    pub iat: u64,
}

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| random_string(16));

impl Claims {
    pub fn new(email: impl Into<String>) -> Claims {
        let issued_at = jsonwebtoken::get_current_timestamp();
        let expiry = issued_at + 24 * 3600 /* 24h */;
        Self {
            email: email.into(),
            exp: expiry,
            iat: issued_at,
        }
    }

    pub fn encode(&self) -> jsonwebtoken::errors::Result<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )
    }

    pub fn decode(token: &str) -> jsonwebtoken::errors::Result<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        ).map(|x| x.claims)
    }
}

pub fn validate_user(cookie: &CookieJar) -> Option<Claims> {
    let token = cookie.get("token")?;
    Claims::decode(token.value()).ok()
}
