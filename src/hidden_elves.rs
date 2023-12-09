use axum::{routing::post, Router};

async fn count_elf_in_input(body: String) -> String {
    let body = body.to_lowercase();

    let total_elf = body.split("elf").count() - 1;
    let total_elf_in_shelf = body.split("elf on a shelf").count() - 1;
    let total_shelf_with_no_elf = body
        .split("elf on a shelf")
        .collect::<String>()
        .split("shelf")
        .count()
        - 1;

    format!(
        "{{\"elf\":{},\"elf on a shelf\": {},\"shelf with no elf on it\":{}}}",
        total_elf, total_elf_in_shelf, total_shelf_with_no_elf
    )
}

pub fn get_hidden_elves_routes() -> Router {
    Router::new().route("/", post(count_elf_in_input))
}
