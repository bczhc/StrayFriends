use crate::db::{AdoptionRequestQueryRow, PgCount, RowId};
use crate::handlers::{handle_errors, PaginationQuery};
use crate::jwt::{validate_token, validate_token_admin};
use crate::{api_ok, include_sql, ApiExtension, AuthHeader};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdoptionFilingForm {
    post_id: RowId,
    details: String,
    mobile_number: String,
}

pub async fn file_adoption_request(
    ext: ApiExtension,
    auth: AuthHeader,
    Form(form): Form<AdoptionFilingForm>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claims = validate_token!(auth);
        let uid = claims.uid;
        let db = &ext.db;

        sqlx::query(include_sql!("file-adoption"))
            .bind(uid)
            .bind(form.post_id)
            .bind(form.details)
            .bind(form.mobile_number)
            .execute(db)
            .await?;

        return api_ok!(());
    };
    handle_errors!(r)
}

pub async fn list_requests(
    ext: ApiExtension,
    auth: AuthHeader,
    pagination: Query<PaginationQuery>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claims = validate_token_admin!(auth);

        let request: Vec<AdoptionRequestQueryRow> =
            sqlx::query_as(include_sql!("list-adoption-requests"))
                .bind(pagination.offset)
                .bind(pagination.limit)
                .fetch_all(&ext.db)
                .await?;

        return api_ok!(request);
    };
    handle_errors!(r)
}

pub async fn count(ext: ApiExtension, auth: AuthHeader) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claims = validate_token_admin!(auth);

        let (count,): (PgCount,) = sqlx::query_as(include_sql!("count-adoption-requests"))
            .fetch_one(&ext.db)
            .await?;

        return api_ok!(count);
    };
    handle_errors!(r)
}

pub async fn delete(
    ext: ApiExtension,
    auth: AuthHeader,
    path: Path<(RowId,)>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        let claims = validate_token_admin!(auth);
        sqlx::query(include_sql!("delete-adoption-request"))
            .bind(path.0 .0)
            .execute(&ext.db)
            .await?;
        return api_ok!(());
    };
    handle_errors!(r)
}
