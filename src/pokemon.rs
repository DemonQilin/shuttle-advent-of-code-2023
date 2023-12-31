use axum::{extract::Path, http::StatusCode, routing::get, Router};
use serde::Deserialize;

use crate::AppState;

const GRAVITATIONAL_ACCELERATION: f32 = 9.825;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Pokemon {
    id: u16,
    name: String,
    weight: Hectogram,
}

#[derive(Debug, Deserialize)]
struct Hectogram(f32);
struct Kilogram(f32);

struct Newton {
    mass: Kilogram,
    acceleration: f32,
}

struct Joul {
    force: Newton,
    duration: f32,
}

impl Pokemon {
    fn momentum(&self, distance: f32) -> Joul {
        let kilo_weigth = self.weight.to_kilogram();
        let duration = (distance * 2.0 / GRAVITATIONAL_ACCELERATION).sqrt();
        let force = Newton {
            mass: kilo_weigth,
            acceleration: GRAVITATIONAL_ACCELERATION,
        };

        Joul { force, duration }
    }
}

impl Hectogram {
    fn to_kilogram(&self) -> Kilogram {
        Kilogram(self.0 / 10.0)
    }
}

impl Newton {
    fn value(&self) -> f32 {
        self.acceleration * self.mass.0
    }
}

impl Joul {
    fn value(&self) -> f32 {
        self.force.value() * self.duration
    }
}

async fn get_pokemon_weight(Path(pokemon_id): Path<u16>) -> (StatusCode, String) {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{pokemon_id}");
    let res = reqwest::get(url).await;
    if let Err(e) = res {
        if e.is_request() {
            let msg = format!("The request for pokemon {pokemon_id} failed");
            return (StatusCode::NOT_FOUND, msg);
        }

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".into(),
        );
    }

    let pokemon = res.unwrap().json::<Pokemon>().await.unwrap();
    let kilo_weight = pokemon.weight.to_kilogram().0;

    (StatusCode::OK, kilo_weight.to_string())
}

async fn get_drop_momentum(Path(pokemon_id): Path<u16>) -> (StatusCode, String) {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{pokemon_id}");
    let res = reqwest::get(url).await;
    if let Err(e) = res {
        if e.is_request() {
            let msg = format!("The request for pokemon {pokemon_id} failed");
            return (StatusCode::NOT_FOUND, msg);
        }

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".into(),
        );
    }

    let pokemon = res.unwrap().json::<Pokemon>().await.unwrap();
    let momentum = pokemon.momentum(10.0).value();

    (StatusCode::OK, momentum.to_string())
}

pub fn get_pokemon_routes() -> Router<AppState> {
    Router::new()
        .route("/weight/:pokemon_id", get(get_pokemon_weight))
        .route("/drop/:pokemon_id", get(get_drop_momentum))
}
