use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
struct Reindeer {
    strength: i32,
    #[allow(dead_code)]
    name: String,
}

#[derive(Deserialize)]
struct DetailReindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    candies_eaten: u32,
}

#[derive(Serialize)]
struct ContestWinnersResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl ContestWinnersResponse {
    fn new(
        fastest: &DetailReindeer,
        tallest: &DetailReindeer,
        magician: &DetailReindeer,
        consumer: &DetailReindeer,
    ) -> Self {
        Self {
            fastest: format!(
                "Speeding past the finish line with a strength of {} is {}",
                fastest.strength, fastest.name
            ),
            tallest: format!(
                "{} is standing tall with his {} cm wide antlers",
                tallest.name, tallest.antler_width
            ),
            magician: format!(
                "{} could blast you away with a snow magic power of {}",
                magician.name, magician.snow_magic_power
            ),
            consumer: format!(
                "{} ate lots of candies, but also some {}",
                consumer.name, consumer.favorite_food
            ),
        }
    }
}

async fn get_reideers_total_strength(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    reindeers
        .into_iter()
        .map(|reindeer| reindeer.strength)
        .sum::<i32>()
        .to_string()
}

async fn get_contest_winners(body: String) -> axum::response::Result<String, StatusCode> {
    let body = body.replace("cAnD13s_3ATeN-yesT3rdAy", "candies_eaten");
    let reindeers: Vec<DetailReindeer> =
        serde_json::from_str(&body).map_err(|_| StatusCode::BAD_REQUEST)?;

    let fastest = reindeers
        .iter()
        .max_by(|a, b| a.speed.total_cmp(&b.speed))
        .expect("A reindeer fastest than others was expected");

    let tallest = reindeers
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .expect("A reindeer tallest than others was expected");

    let magician = reindeers
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .expect("A reindeer more magician than others was expected");

    let consumer = reindeers
        .iter()
        .max_by(|a, b| a.candies_eaten.cmp(&b.candies_eaten))
        .expect("A reindeer more consumer than others was expected");

    let winners = ContestWinnersResponse::new(fastest, tallest, magician, consumer);

    serde_json::to_string(&winners).map_err(|_| StatusCode::BAD_REQUEST)
}

pub fn get_reindeer_routes() -> Router<AppState> {
    Router::new()
        .route("/strength", post(get_reideers_total_strength))
        .route("/contest", post(get_contest_winners))
}
