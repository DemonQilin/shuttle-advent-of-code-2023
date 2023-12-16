use axum::Router;
use tower_http::services::ServeDir;

pub fn get_imagery_routes() -> Router {
    Router::new().nest_service("/assets", ServeDir::new("assets"))
}
