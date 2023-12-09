use axum::{http::HeaderMap, routing::get, Router};
use base64::prelude::*;

async fn get_encoded_cookies_recipe(headers: HeaderMap) -> Result<String, String> {
    let cookie = headers.get("cookie").ok_or("A cookie is expected")?;
    let recipe = cookie.as_bytes().splitn(2, |&v| v == b'=').last().unwrap();

    let decode_recipe = BASE64_STANDARD
        .decode(recipe)
        .map_err(|_| "The recipe cookie is not valid")?;
    let decode_recipe = String::from_utf8(decode_recipe).unwrap();

    Ok(decode_recipe)
}

pub fn get_cookies_recipe_routes() -> Router {
    Router::new().route("/decode", get(get_encoded_cookies_recipe))
}
