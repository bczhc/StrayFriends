use crate::db::{AnimalInfoQueryRow, AnimalPostForm};
use crate::handlers::{handle_errors, PaginationQuery};
use crate::jwt::validate_token;
use crate::{api_ok, include_sql, ApiExtension, AuthHeader};
use anyhow::anyhow;
use axum::extract::{Query, RawQuery};
use axum::response::IntoResponse;
use axum::{debug_handler, Form};
use serde_with::serde_derive::{Serialize, Deserialize};

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
        return api_ok!(animals);
    };
    handle_errors!(r)
}
