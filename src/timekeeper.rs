use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::{
    extract::{Path, State},
    response,
    routing::{get, post},
    Json, Router,
};
use chrono::{prelude::*, LocalResult};
use reqwest::StatusCode;
use serde::Serialize;
use ulid::Ulid;
use uuid::Uuid;

type Timekeeper = Arc<Mutex<HashMap<String, Instant>>>;

#[derive(Debug, Serialize)]
struct UlidAnalysis {
    #[serde(rename(serialize = "christmas eve"))]
    christmas: usize,
    weekday: usize,
    #[serde(rename(serialize = "in the future"))]
    future: usize,
    #[serde(rename(serialize = "LSB is 1"))]
    lsb: usize,
}

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

async fn convert_ulids_to_uuids(Json(ulids): Json<Vec<Ulid>>) -> Json<Vec<Uuid>> {
    let uuids: Vec<Uuid> = ulids.into_iter().rev().map(|ulid| ulid.into()).collect();

    Json(uuids)
}

async fn analize_ulids(
    Path(weekday): Path<u8>,
    Json(ulids): Json<Vec<Ulid>>,
) -> response::Result<Json<UlidAnalysis>> {
    let dates = ulids
        .iter()
        .map(
            |ulid| match Utc.timestamp_millis_opt(ulid.timestamp_ms() as i64) {
                LocalResult::Single(date) => Ok(date),
                _ => Err(format!(
                    "The ulid \"{}\" could not be converted to a valid utc date",
                    ulid.to_string()
                )),
            },
        )
        .collect::<Result<Vec<_>, _>>()?;

    let christmas = dates
        .iter()
        .filter(|date| date.day() == 24 && date.month() == 12)
        .count();

    let weekday = dates
        .iter()
        .filter(|date| date.weekday().num_days_from_monday() == weekday as u32)
        .count();

    let current_date = Utc::now();
    let future = dates
        .iter()
        .filter(move |date| **date > current_date)
        .count();

    let lsb = ulids
        .into_iter()
        .filter(|ulid| ulid.random() % 2 != 0)
        .count();

    let analysis = UlidAnalysis {
        christmas,
        weekday,
        future,
        lsb,
    };

    Ok(Json(analysis))
}

pub fn make_timekeeper_api() -> Router {
    let timekeeper: Timekeeper = Arc::new(Mutex::new(HashMap::new()));

    Router::new()
        .route("/save/:packet_key", post(save_packet))
        .route("/load/:packet_key", get(get_elapsed_time))
        .route("/ulids", post(convert_ulids_to_uuids))
        .route("/ulids/:weekday", post(analize_ulids))
        .with_state(timekeeper)
}
