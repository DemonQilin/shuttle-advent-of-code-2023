use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::extract::FromRef;
use sqlx::PgPool;

pub type Timekeeper = Arc<Mutex<HashMap<String, Instant>>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub timekeeper: Timekeeper,
}

impl FromRef<AppState> for Timekeeper {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.timekeeper.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.pool.clone()
    }
}
