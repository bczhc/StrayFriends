/// TODO: Axum `async fn` has lots of boilerplate. A neat way to eliminate them
///  is use Rust procedural macros.

mod account;
mod animal;
mod upload;
mod adoption;

use crate::mutex_lock;
use axum::response::IntoResponse;
use axum::{debug_handler, Router};
use log::info;
use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Mutex;

use serde_with::{serde_as, DisplayFromStr};
use serde_with::serde_derive::{Serialize, Deserialize};

static COLLECTED_ROUTES: Lazy<Mutex<Vec<&'static str>>> =
    Lazy::new(|| Mutex::new(Default::default()));

macro add_route($router:expr, $t:tt $path:literal, $f:expr) {
    paste::paste! {
        $router = $router.route($path, ::axum::routing::[<$t:lower>]($f));
        mutex_lock!(COLLECTED_ROUTES).push(concat!(stringify!([<$t:upper>]), " ", $path));
    }
}

pub fn router() -> Router {
    let mut router = Router::new();
    add_route!(router, GET "/test", test_api);
    add_route!(router, GET "/routes", list_routes);
    add_route!(router, POST "/login", account::login);
    add_route!(router, POST "/signup", account::signup);
    add_route!(router, GET "/me/email", account::my_email);
    add_route!(router, GET "/me", account::my_info);
    add_route!(router, POST "/image/upload", upload::upload_image);
    add_route!(router, GET "/image/:id", upload::image);
    add_route!(router, PUT "/me", account::update_info);
    add_route!(router, POST "/animal", animal::post_animal);
    add_route!(router, GET "/animals", animal::list);
    add_route!(router, GET "/animal/:id", animal::query_animal_post);
    add_route!(router, POST "/adoption", adoption::file_adoption_request);
    add_route!(router, GET "/user/:id", account::query_user_api);
    /* ====================== OWNER OR ADMIN ====================== */
    add_route!(router, PATCH "/animal/:id/adopt", animal::set_adopted);
    add_route!(router, DELETE "/animal/:id", animal::delete);
    /* ====================== ADMIN ====================== */
    add_route!(router, GET "/adoptions/count", adoption::count);
    add_route!(router, GET "/adoptions/list", adoption::list_requests);
    add_route!(router, DELETE "/adoption/:id", adoption::delete);
    /* =================================================== */
    router
}

pub async fn list_routes() -> impl IntoResponse {
    info!("Route: /routes");
    let mut content = String::new();
    use fmt::Write;
    for &line in &*mutex_lock!(COLLECTED_ROUTES) {
        writeln!(&mut content, "{}", line).unwrap();
    }
    content
}

pub macro api_error {
() => {
        crate::ResponseJson::<()>::error().into_response()
    },
($message:expr) => {{
        log::debug!("Error message:\n{}", $message);
        crate::ResponseJson::<()>::error_msg($message).into_response()
    }}
}

pub macro handle_errors($r:expr) {{
    log::debug!("Result: {:?}", &$r);
    let err = $r.err().unwrap();
    api_error!(format!("{}", err))
}}

#[debug_handler]
pub async fn test_api() -> impl IntoResponse {
    "hello, world"
}

/// The common pagination query parameters
///
/// With `serde(flatten)`, query deserialization can't be handled well,
/// I have to use this ugly workaround.
///
/// When using the workaround, note `Serialize` and `Deserialize` traits
/// are from decent `serde_with::serde_derive::{Serialize, Deserialize}` instead,
/// not from `serde`.
///
/// https://github.com/nox/serde_urlencoded/issues/33#issuecomment-629803582
#[serde_as]
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct PaginationQuery {
    #[serde_as(as = "DisplayFromStr")]
    offset: i32,
    #[serde_as(as = "DisplayFromStr")]
    limit: i32,
}

#[cfg(test)]
mod test {
    use crate::handlers::animal::ListQuery;
    use crate::handlers::PaginationQuery;

    #[test]
    fn test() {
        let a: Result<ListQuery, _> = serde_qs::from_str("offset=0&limit=20");
        let query = ListQuery {
            pagination: PaginationQuery { offset: 0, limit: 20 }
        };
        assert_eq!(a.ok(), Some(query));
    }
}
