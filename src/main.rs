use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{http::StatusCode, routing::get, Router};

use cch23_demonqilin01::{
    get_cookies_recipe_routes, get_hidden_elves_routes, get_imagery_routes, get_pokemon_routes,
    get_reindeer_routes, get_sled_routes, make_timekeeper_api, AppState,
};
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let state = AppState {
        pool,
        timekeeper: Arc::new(Mutex::new(HashMap::new())),
    };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .nest("/1", get_sled_routes())
        .nest("/4", get_reindeer_routes())
        .nest("/6", get_hidden_elves_routes())
        .nest("/7", get_cookies_recipe_routes())
        .nest("/8", get_pokemon_routes())
        .nest("/11", get_imagery_routes())
        .nest("/12", make_timekeeper_api())
        .with_state(state);

    Ok(router.into())
}
