use axum::{http::StatusCode, routing::get, Router};
use cch23_demonqilin01::{get_reindeer_routes, get_sled_routes, hidden_elves};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .nest("/1", get_sled_routes())
        .nest("/4", get_reindeer_routes())
        .nest("/6", hidden_elves::get_hidden_elves_routes());

    Ok(router.into())
}
