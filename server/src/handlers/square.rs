use crate::db::{PgCount, SquarePostRow};
use crate::handlers::{handle_errors, PaginationQuery};
use crate::jwt::validate_token;
use crate::{api_ok, include_sql, ApiExtension, AuthHeader};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{debug_handler, Form};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ListResponse {
    total: PgCount,
    list: Vec<SquarePostRow>,
}

#[debug_handler]
pub async fn list(ext: ApiExtension, pagination: Query<PaginationQuery>) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let fetch: Vec<SquarePostRow> = sqlx::query_as(include_sql!("list-square-posts"))
            .bind(pagination.offset)
            .bind(pagination.limit)
            .fetch_all(&ext.db)
            .await?;
        let (count,): (PgCount,) = sqlx::query_as(include_sql!("count-square-posts"))
            .fetch_one(&ext.db)
            .await?;
        let res = ListResponse {
            list: fetch,
            total: count,
        };
        return api_ok!(res);
    };
    handle_errors!(r)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostForm {
    content: String,
    /// String JSON array
    pub images: String,
}

#[debug_handler]
pub async fn post_new(
    ext: ApiExtension,
    auth: AuthHeader,
    Form(form): Form<PostForm>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claims = validate_token!(auth);

        let images = serde_json::from_str::<Vec<String>>(&form.images)?;

        sqlx::query(include_sql!("post-square"))
            .bind(claims.uid)
            .bind(form.content)
            .bind(images)
            .execute(&ext.db)
            .await?;

        return api_ok!(());
    };
    handle_errors!(r)
}
