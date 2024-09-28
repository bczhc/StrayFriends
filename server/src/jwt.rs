use crate::{random_string, BYPASS_USER_VALIDATION, DEBUG_MODE};
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub uid: i64,
    pub exp: u64,
    pub iat: u64,
}

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| random_string(16));

impl Claims {
    pub fn new(email: impl Into<String>, uid: i64) -> Claims {
        let issued_at = jsonwebtoken::get_current_timestamp();
        let expiry = issued_at + 24 * 3600 /* 24h */;
        Self {
            email: email.into(),
            uid,
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
        if DEBUG_MODE && BYPASS_USER_VALIDATION {
            // only for test purposes. No need to handle errors here.
            let payload = token.split('.').nth(1).unwrap();
            let payload = BASE64_STANDARD_NO_PAD.decode(payload).unwrap();
            let claims: Claims =
                serde_json::from_str(std::str::from_utf8(&payload).unwrap()).unwrap();
            return Ok(claims);
        }
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map(|x| x.claims)
    }
}

pub fn validate_token(token: &str) -> Option<Claims> {
    Claims::decode(token).ok()
}

pub macro validate_token($auth_header:expr) {{
    let Some(claims) = validate_token($auth_header.token()) else {
        use ::axum::response::IntoResponse;
        return ::axum::http::status::StatusCode::UNAUTHORIZED.into_response()
    };
    claims
}}
