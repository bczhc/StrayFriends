use crate::db::AnimalPostForm;
use crate::handlers::handle_errors;
use crate::jwt::validate_token;
use crate::{api_ok, include_sql, ApiExtension, AuthHeader};
use axum::response::IntoResponse;
use axum::{debug_handler, Form};

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
            .execute(db)
            .await?;

        return api_ok!(());
    };
    handle_errors!(r)
}
