use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn sled_id(Path(packed_ids): Path<String>) -> Result<String, StatusCode> {
    let mut packed_ids = packed_ids;

    if packed_ids.ends_with('/') {
        packed_ids = packed_ids[..packed_ids.len() - 1].to_string();
    }

    let packed_ids = packed_ids;

    if packed_ids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut id_sled: i32 = 0;

    for (index, packed_id) in packed_ids.split('/').enumerate() {
        if index > 19 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let id: i32 = packed_id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
        id_sled ^= id;
    }

    id_sled = id_sled.pow(3);

    Ok(id_sled.to_string())
}

async fn get_total_strength(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    reindeers
        .into_iter()
        .map(|reindeer| reindeer.strength)
        .sum::<i32>()
        .to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/*packed_ids", get(sled_id))
        .route("/4/strength", post(get_total_strength));

    Ok(router.into())
}
