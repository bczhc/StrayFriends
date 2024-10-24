use crate::db::{AnimalInfoQueryRow, AnimalPostForm, RowId};
use crate::handlers::{handle_errors, PaginationQuery};
use crate::jwt::{validate_token, validate_token_admin};
use crate::{api_ok, include_sql, jwt, ApiExtension, AuthHeader};
use anyhow::anyhow;
use axum::extract::{Path, Query, RawQuery};
use axum::response::IntoResponse;
use axum::{debug_handler, Form};
use serde_with::serde_derive::{Deserialize, Serialize};

#[debug_handler]
pub async fn post_animal(
    ext: ApiExtension,
    auth: AuthHeader,
    Form(form): Form<AnimalPostForm>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claim = validate_token!(auth);
        let uid = claim.uid;

        let image_ids: Vec<String> = serde_json::from_str(&form.image_id_list)?;

        let db = &ext.db;
        sqlx::query(include_sql!("post-animal"))
            .bind(uid)
            .bind(&form.name)
            .bind(image_ids)
            .bind(&form.content)
            .bind(false)
            .bind(&form.mobile_number)
            .bind(&form.description)
            .execute(db)
            .await?;

        return api_ok!(());
    };
    handle_errors!(r)
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct ListQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
}

#[derive(Serialize, Debug)]
pub struct ListResponse {
    total: i64,
    animals: Vec<AnimalInfoQueryRow>,
}

#[debug_handler]
pub async fn list(ext: ApiExtension, query: RawQuery) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let query: ListQuery =
            serde_qs::from_str(&query.0.ok_or_else(|| anyhow!("Query missing!"))?)?;
        let (offset, limit) = (query.pagination.offset, query.pagination.limit);
        let db = &ext.db;
        let animals: Vec<AnimalInfoQueryRow> = sqlx::query_as(include_sql!("list-animals"))
            .bind(offset)
            .bind(limit)
            .fetch_all(db)
            .await?;

        let count: (i64,) = sqlx::query_as(include_sql!("count-animals"))
            .fetch_one(db)
            .await?;

        let response = ListResponse {
            total: count.0,
            animals,
        };
        return api_ok!(response);
    };
    handle_errors!(r)
}

#[debug_handler]
pub async fn query_animal_post(ext: ApiExtension, Path(path): Path<(RowId,)>) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let post_id = path.0;
        let db = &ext.db;

        let animal_info: AnimalInfoQueryRow = sqlx::query_as(include_sql!("query-animal-post"))
            .bind(post_id)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| anyhow!("未找到记录"))?;

        return api_ok!(animal_info);
    };
    handle_errors!(r)
}

macro check_owned($claims:expr, $post_id:expr, $db:expr) {
    let (owned,): (bool,) = sqlx::query_as(include_sql!("check-animal-post-owner"))
        .bind($post_id)
        .bind($claims.uid)
        .fetch_optional($db)
        .await?
        .unwrap_or((false,));
    if !owned {
        jwt::axum_return_unauthorized!();
    }
}

#[debug_handler]
pub async fn set_adopted(
    ext: ApiExtension,
    auth: AuthHeader,
    path: Path<(RowId,)>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claims = validate_token_admin!(auth);
        check_owned!(claims, path.0 .0, &ext.db);
        sqlx::query(include_sql!("animal-set-adopted"))
            .bind(path.0 .0)
            .execute(&ext.db)
            .await?;
        return api_ok!(());
    };
    handle_errors!(r)
}

#[debug_handler]
pub async fn delete(
    ext: ApiExtension,
    auth: AuthHeader,
    path: Path<(RowId,)>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claims = validate_token_admin!(auth);
        check_owned!(claims, path.0 .0, &ext.db);
        sqlx::query(include_sql!("animal-delete"))
            .bind(path.0 .0)
            .execute(&ext.db)
            .await?;
        return api_ok!(());
    };
    handle_errors!(r)
}
