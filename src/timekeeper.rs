use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::{
    extract::{Path, State},
    routing::{get, post},
};
use reqwest::StatusCode;

type Timekeeper = Arc<Mutex<HashMap<String, Instant>>>;

async fn get_elapsed_time(
    State(timekeeper): State<Timekeeper>,
    Path(packet_key): Path<String>,
) -> (StatusCode, String) {
    let timekeeper = timekeeper.lock().unwrap();
    let packet_instant = timekeeper.get(&packet_key);
    if packet_instant.is_none() {
        return (
            StatusCode::NOT_FOUND,
            format!("The packet \"{packet_key}\" was not founded"),
        );
    }

    let elapsed_time = packet_instant.unwrap().elapsed().as_secs();

    (StatusCode::OK, elapsed_time.to_string())
}

async fn save_packet(State(timekeeper): State<Timekeeper>, Path(packet_key): Path<String>) {
    let mut timekeeper = timekeeper.lock().unwrap();
    let now = Instant::now();

    timekeeper.insert(packet_key, now);
}
