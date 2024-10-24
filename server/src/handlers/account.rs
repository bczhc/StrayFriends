use crate::db::{change_password, Gender, GenderPg, Password, RowId, Uid, User};
use crate::handlers::{api_error, handle_errors};
use crate::jwt::{validate_token, Claims};
use crate::{api_ok, db, include_sql, ApiContext, ApiExtension, AuthHeader, DbPool};
use anyhow::anyhow;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{AppendHeaders, IntoResponse, Response};
use axum::{debug_handler, Form};
use axum_extra::extract::CookieJar;
use axum_extra::headers::authorization::Bearer;
use axum_extra::{headers, TypedHeader};
use serde::Deserialize;
use sqlx::{query, Executor, PgPool};
use std::sync::Arc;
use axum::extract::Path;
use log::debug;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserInfoForm {
    pub name: String,
    pub old_password: String,
    pub avatar_image_id: Option<String>,
    pub new_password: String,
    pub gender_type: String,
    pub gender_other: Option<String>,
    pub bio: Option<String>,
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
        
        let user = query_user(db, query.0).await?;
        let jwt = Claims::new(user, query.0).encode()?;
        return api_ok!(jwt);
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

#[debug_handler]
pub async fn my_email(auth: AuthHeader) -> impl IntoResponse {
    let claims = validate_token!(auth);
    api_ok!(claims.user.email)
}

async fn query_user(db: &PgPool, uid: RowId) -> anyhow::Result<User> {
    let user: User = sqlx::query_as(include_sql!("query-user"))
        .bind(uid)
        .fetch_one(db)
        .await?;
    Ok(user)
}

pub async fn query_user_api(ext: ApiExtension, path: Path<(RowId,)>) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let user = query_user(&ext.db, path.0.0).await?;
        return api_ok!(user);
    };
    handle_errors!(r)
}

#[debug_handler]
pub async fn my_info(ext: ApiExtension, auth: AuthHeader) -> impl IntoResponse {
    let claims = validate_token!(auth);
    let r: anyhow::Result<()> = try {
        let db = &ext.db;
        let uid = claims.uid;
        let user = query_user(db, uid).await?;
        return api_ok!(user);
    };
    handle_errors!(r)
}

#[debug_handler]
pub async fn update_info(
    ext: ApiExtension,
    auth: AuthHeader,
    Form(form): Form<UpdateUserInfoForm>,
) -> impl IntoResponse {
    let claims = validate_token!(auth);
    let db = &ext.db;
    let r: anyhow::Result<_> = try {
        debug!("Form: {:?}", form);
        
        if !form.new_password.is_empty() {
            // validate old password
            let (old_pass,): (Password,) = sqlx::query_as(include_sql!("query-password-by-uid"))
                .bind(claims.uid)
                .fetch_one(db)
                .await?;
            if !old_pass.validate(&form.old_password) {
                return api_error!("原密码错误");
            }
            let new_password = Password::generate(&form.new_password);
            change_password(db, &new_password).await?;
        }

        let gender_pg = GenderPg::from(
            Gender::from(&form.gender_type, form.gender_other)
                .ok_or_else(|| anyhow!("无效性别"))?,
        );
        sqlx::query(include_sql!("update-user-info"))
            .bind(claims.uid)
            .bind(form.name)
            .bind(form.avatar_image_id)
            .bind(&gender_pg.r#type)
            .bind(&gender_pg.other)
            .bind(form.bio)
            .execute(db)
            .await?;
        return api_ok!(());
    };
    handle_errors!(r)
}
