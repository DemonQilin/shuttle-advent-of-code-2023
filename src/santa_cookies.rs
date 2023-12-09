use axum::{
    http::{header::COOKIE, HeaderMap, HeaderValue},
    routing::get,
    Router,
};
use base64::prelude::*;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    ops::{Mul, Sub},
};

#[derive(Debug, Deserialize, Serialize)]
struct Ingredients {
    flour: u32,
    sugar: u32,
    butter: u32,
    baking_powder: u32,
    chocolate_chips: u32,
}

#[derive(Debug, Serialize)]
struct RecipeResult {
    cookies: u32,
    pantry: Ingredients,
}

#[derive(Debug, Deserialize)]
struct Recipe {
    recipe: Ingredients,
    pantry: Ingredients,
}

impl Sub for Ingredients {
    type Output = Ingredients;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            flour: self.flour - rhs.flour,
            sugar: self.sugar - rhs.sugar,
            butter: self.butter - rhs.butter,
            baking_powder: self.baking_powder - rhs.baking_powder,
            chocolate_chips: self.chocolate_chips - rhs.chocolate_chips,
        }
    }
}

impl Mul<u32> for Ingredients {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            flour: self.flour * rhs,
            sugar: self.sugar * rhs,
            butter: self.butter * rhs,
            baking_powder: self.baking_powder * rhs,
            chocolate_chips: self.chocolate_chips * rhs,
        }
    }
}

impl Recipe {
    fn bake(self) -> RecipeResult {
        let total_cookies = *[
            self.pantry.flour / self.recipe.flour,
            self.pantry.sugar / self.recipe.sugar,
            self.pantry.butter / self.recipe.butter,
            self.pantry.baking_powder / self.recipe.baking_powder,
            self.pantry.chocolate_chips / self.recipe.chocolate_chips,
        ]
        .iter()
        .min()
        .unwrap();

        let remain = self.pantry - (self.recipe * total_cookies);

        RecipeResult {
            cookies: total_cookies,
            pantry: remain,
        }
    }
}

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
