use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn cube_bits(Path((num1, num2)): Path<(u32, u32)>) -> String {
    let cube = (num1 ^ num2).pow(3);
    cube.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/:num1/:num2", get(cube_bits));

    Ok(router.into())
}
