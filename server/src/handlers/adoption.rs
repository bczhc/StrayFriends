use crate::db::RowId;
use crate::handlers::handle_errors;
use crate::jwt::validate_token;
use crate::{api_ok, include_sql, ApiExtension, AuthHeader};
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
