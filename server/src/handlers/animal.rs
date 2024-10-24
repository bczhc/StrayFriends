use crate::db::AnimalPostForm;
use crate::handlers::handle_errors;
use crate::{api_ok, ApiExtension, AuthHeader};
use axum::response::IntoResponse;
use axum::{debug_handler, Form};

#[debug_handler]
pub async fn post_animal(
    ext: ApiExtension,
    auth: AuthHeader,
    Form(form): Form<AnimalPostForm>,
) -> impl IntoResponse {
    let r: anyhow::Result<_> = try {
        return api_ok!(());
    };
    handle_errors!(r)
}
