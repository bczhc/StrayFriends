use crate::{random_string, BYPASS_USER_VALIDATION, DEBUG_MODE};
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::db::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: User,
    pub uid: i64,
    pub exp: u64,
    pub iat: u64,
}

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| random_string(16));

impl Claims {
    pub fn new(user: User, uid: i64) -> Claims {
        let issued_at = jsonwebtoken::get_current_timestamp();
        let expiry = issued_at + 24 * 3600 /* 24h */;
        Self {
            user,
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
            // Only for test purposes. No need to check the signature. Just extract
            // the claims.
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

pub macro axum_return_unauthorized() {{
    use ::axum::response::IntoResponse;
    return ::axum::http::status::StatusCode::UNAUTHORIZED.into_response()
}}

pub macro validate_token($auth_header:expr) {{
    let Some(claims) = validate_token($auth_header.token()) else {
        crate::jwt::axum_return_unauthorized!()
    };
    claims
}}

pub macro validate_token_admin($auth_header:expr) {{
    let claims = crate::jwt::validate_token!($auth_header);
    if !claims.user.admin {
        crate::jwt::axum_return_unauthorized!();
    }
    claims
}}
