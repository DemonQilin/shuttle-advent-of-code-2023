use axum::{extract::Path, http::StatusCode, routing::get, Router};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Pokemon {
    id: u16,
    name: String,
    weight: Hectogram,
}

#[derive(Debug, Deserialize)]
struct Hectogram(u16);
struct Kilogram(u16);

struct Newton {
    mass: Kilogram,
    acceleration: f32,
}

struct Joul {
    force: Newton,
    duration: f32,
}

impl Hectogram {
    fn to_kilogram(&self) -> Kilogram {
        Kilogram(self.0 / 10)
    }
}

impl Newton {
    fn value(&self) -> f32 {
        self.acceleration * self.mass.0 as f32
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

pub fn get_pokemon_routes() -> Router {
    Router::new().route("/weight/:pokemon_id", get(get_pokemon_weight))
}
