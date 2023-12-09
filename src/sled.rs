use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn sled_id(Path(packed_ids): Path<String>) -> axum::response::Result<String, StatusCode> {
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

pub fn get_sled_routes() -> Router {
    Router::new().route("/*packed_ids", get(sled_id))
}
