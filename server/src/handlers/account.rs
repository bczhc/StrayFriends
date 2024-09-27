use crate::db::{Password, Uid, User};
use crate::handlers::{api_error, handle_errors};
use crate::jwt::{validate_user, Claims};
use crate::{api_ok, db, include_sql, ApiContext, ApiExtension, DbPool};
use axum::debug_handler;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{AppendHeaders, IntoResponse, Response};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use sqlx::{query, Executor};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct SignupForm {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[debug_handler]
pub async fn login(ext: ApiExtension, form: axum::Form<LoginForm>) -> impl IntoResponse {
    let result: anyhow::Result<()> = try {
        let db = &ext.db;

        let email = &form.username;
        let query: Option<(i64, String, String)> = sqlx::query_as(include_sql!("login-query"))
            .bind(email)
            .fetch_optional(db)
            .await?;
        let Some(query) = query else {
            return api_error!("用户不存在");
        };

        if !Password::new(query.1, query.2).validate(&form.password) {
            return api_error!("用户名或密码错误");
        }

        let jwt = Claims::new(email, query.0).encode()?;
        return ([(SET_COOKIE, format!("token={jwt}"))], api_ok!(jwt)).into_response();
    };
    handle_errors!(result)
}

#[debug_handler]
pub async fn signup(ext: ApiExtension, form: axum::Form<SignupForm>) -> impl IntoResponse {
    let r: anyhow::Result<()> = try {
        let db = &ext.db;

        if query_id_from_email(db, &form.email).await?.is_some() {
            return api_error!("用户名已经注册");
        }

        let password = Password::generate(&form.password);
        sqlx::query(include_sql!("user-signup"))
            .bind(&form.name)
            .bind(&form.email)
            .bind(&password.blake3)
            .bind(&password.salt)
            .execute(db)
            .await?;

        return api_ok!(());
    };
    handle_errors!(r)
}

async fn query_id_from_email(db: &DbPool, email: &str) -> anyhow::Result<Option<Uid>> {
    let uid: Option<(Uid,)> = sqlx::query_as(r#"SELECT id FROM "user" WHERE email = $1"#)
        .bind(email)
        .fetch_optional(db)
        .await?;
    Ok(uid.map(|x| x.0))
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

#[debug_handler]
pub async fn my_info(ext: ApiExtension, cookie: CookieJar) -> impl IntoResponse {
    let claims = validate_user!(&cookie);
    let r: anyhow::Result<()> = try {
        let db = &ext.db;
        let uid = claims.uid;
        let user: User = sqlx::query_as(include_sql!("query-user"))
            .bind(uid)
            .fetch_one(db)
            .await?;
        return api_ok!(user);
    };
    handle_errors!(r)
}
