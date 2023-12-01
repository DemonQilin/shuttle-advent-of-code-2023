use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn sled_id(Path(packed_ids): Path<String>) -> Result<String, StatusCode> {
    if packed_ids.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut id_sled: u32 = 0;

    for (index, packed_id) in packed_ids.split('/').enumerate() {
        if index > 19 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let id: u32 = packed_id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
        id_sled ^= id;
    }

    id_sled = id_sled.pow(3);

    Ok(id_sled.to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/*packed_ids", get(sled_id));

    Ok(router.into())
}
