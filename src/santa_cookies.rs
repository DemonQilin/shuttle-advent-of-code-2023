use axum::{
    http::{header::COOKIE, HeaderMap, HeaderValue},
    routing::get,
    Json, Router,
};
use base64::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::AppState;

#[derive(Debug, Deserialize)]
struct Order {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

#[derive(Debug, Serialize)]
struct OrderResponse {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

impl Order {
    fn bake(self) -> OrderResponse {
        let cookies_total = self
            .recipe
            .iter()
            .filter(|(_, &recipe_quantity)| recipe_quantity != 0)
            .flat_map(|(ingredient, recipe_quantity)| {
                self.pantry
                    .get(ingredient)
                    .map(|pantry_quantity| pantry_quantity / recipe_quantity)
            })
            .min()
            .unwrap_or(0);

        let remain = self.pantry.into_iter().fold(
            HashMap::new(),
            move |mut acc, (ingredient, pantry_quantity)| {
                let recipe_quantity = self.recipe.get(&ingredient);

                if let Some(recipe_quantity) = recipe_quantity {
                    acc.insert(
                        ingredient,
                        pantry_quantity - cookies_total * recipe_quantity,
                    );
                } else {
                    acc.insert(ingredient, pantry_quantity);
                }

                acc
            },
        );

        OrderResponse {
            cookies: cookies_total,
            pantry: remain,
        }
    }
}

fn get_cookies_map(
    headers: &HeaderMap,
) -> axum::response::Result<HashMap<String, HeaderValue>, String> {
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

async fn get_encoded_cookies_recipe(headers: HeaderMap) -> axum::response::Result<String, String> {
    let cookies = get_cookies_map(&headers)?;
    let encode_recipe = cookies.get("recipe").ok_or("Missing cookie")?;

    let raw_recipe = BASE64_STANDARD
        .decode(encode_recipe)
        .map_err(|_| "The recipe cookie is not valid")?;

    let decode_recipe = String::from_utf8(raw_recipe).unwrap();

    Ok(decode_recipe)
}

async fn get_baked_cookies(
    headers: HeaderMap,
) -> axum::response::Result<Json<OrderResponse>, String> {
    let cookies = get_cookies_map(&headers)?;
    let encode_recipe = cookies.get("recipe").ok_or("Missing cookie")?;

    let raw_recipe = BASE64_STANDARD
        .decode(encode_recipe)
        .map_err(|_| "Invalid value for recipe cookie")?;

    let recipe = serde_json::from_slice::<Order>(&raw_recipe)
        .map_err(|_| "Order does not have the correct shape")?;

    let remain = recipe.bake();

    Ok(Json(remain))
}

pub fn get_cookies_recipe_routes() -> Router<AppState> {
    Router::new()
        .route("/decode", get(get_encoded_cookies_recipe))
        .route("/bake", get(get_baked_cookies))
}
