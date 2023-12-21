mod hidden_elves;
mod imagery;
mod models;
mod pokemon;
mod reindeer;
mod santa_cookies;
mod santa_database;
mod sled;
mod timekeeper;

pub use hidden_elves::get_hidden_elves_routes;
pub use imagery::get_imagery_routes;
pub use models::AppState;
pub use pokemon::get_pokemon_routes;
pub use reindeer::get_reindeer_routes;
pub use santa_cookies::get_cookies_recipe_routes;
pub use santa_database::make_santa_database_api;
pub use sled::get_sled_routes;
pub use timekeeper::make_timekeeper_api;
