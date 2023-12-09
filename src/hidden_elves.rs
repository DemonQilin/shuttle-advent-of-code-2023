use axum::{routing::post, Router};

async fn count_elf_in_input(body: String) -> String {
    let total = body.split("elf").count() - 1;

    total.to_string()
}

pub fn get_hidden_elves_routes() -> Router {
    Router::new().route("/", post(count_elf_in_input))
}
