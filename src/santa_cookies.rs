use std::collections::HashMap;

use axum::{
    http::{header::COOKIE, HeaderMap, HeaderValue},
    routing::get,
    Router,
};
use base64::prelude::*;

fn get_cookies_map(headers: &HeaderMap) -> Result<HashMap<String, HeaderValue>, String> {
    let cookies = headers.get_all(COOKIE);
    if cookies.iter().count() == 0 {
        return Err("Cookies are empty".into());
    }

    let mut map = HashMap::new();

    cookies
        .into_iter()
        .flat_map(|value| value.as_bytes().split(|v| v == &b';').collect::<Vec<_>>())
        .map(|cookie| cookie.splitn(2, |&v| v == b'=').collect::<Vec<_>>())
        .for_each(|cookie| {
            map.insert(
                String::from_utf8(cookie[0].into()).unwrap(),
                HeaderValue::from_bytes(cookie[1]).unwrap(),
            );
        });

    Ok(map)
}

async fn get_encoded_cookies_recipe(headers: HeaderMap) -> Result<String, String> {
    let cookies = get_cookies_map(&headers)?;
    let encode_recipe = cookies.get("recipe").ok_or("Missing cookie")?;

    let raw_recipe = BASE64_STANDARD
        .decode(encode_recipe)
        .map_err(|_| "The recipe cookie is not valid")?;

    let decode_recipe = String::from_utf8(raw_recipe).unwrap();

    Ok(decode_recipe)
}

pub fn get_cookies_recipe_routes() -> Router {
    Router::new().route("/decode", get(get_encoded_cookies_recipe))
}
