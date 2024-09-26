use crate::handlers::{api_error, handle_errors};
use crate::jwt::{validate_user, Claims};
use crate::{api_ok, db, include_sql, ApiContext, ApiExtension};
use axum::debug_handler;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{AppendHeaders, IntoResponse, Response};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[debug_handler]
pub async fn login(ext: ApiExtension, form: axum::Form<LoginForm>) -> impl IntoResponse {
    let result: anyhow::Result<()> = try {
        let db = &ext.db;

        let email = &form.username;
        let password: Option<db::Password> = sqlx::query_as(include_sql!("query-password"))
            .bind(email)
            .fetch_optional(db)
            .await?;
        let Some(password) = password else {
            return api_error!("用户不存在");
        };

        if !password.validate(&form.password) {
            return api_error!("用户名或密码错误");
        }

        let jwt = Claims::new(email).encode()?;
        return ([(SET_COOKIE, format!("token={jwt}"))], api_ok!(jwt)).into_response();
    };
    handle_errors!(result)
}

macro validate_user($cookie:expr) {{
    let Some(claims) = validate_user($cookie) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    claims
}}

#[debug_handler]
pub async fn my_email(cookie: CookieJar) -> impl IntoResponse {
    let claims = validate_user!(&cookie);
    api_ok!(claims.email)
}
