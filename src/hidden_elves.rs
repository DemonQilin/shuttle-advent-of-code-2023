use axum::{routing::post, Router};

async fn count_elf_in_input(body: String) -> String {
    let total_elf = body.matches("elf").count();
    let total_shelf = body.matches("shelf").count();
    let total_elf_in_shelf = body
        .chars()
        .collect::<Vec<_>>()
        .windows("elf on a shelf".len())
        .filter(|slice| slice.iter().collect::<String>() == "elf on a shelf")
        .count();

    format!(
        "{{\"elf\":{},\"elf on a shelf\":{},\"shelf with no elf on it\":{}}}",
        total_elf,
        total_elf_in_shelf,
        total_shelf - total_elf_in_shelf
    )
}

pub fn get_hidden_elves_routes() -> Router {
    Router::new().route("/", post(count_elf_in_input))
}
