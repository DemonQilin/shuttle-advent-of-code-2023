use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::extract::FromRef;

pub type Timekeeper = Arc<Mutex<HashMap<String, Instant>>>;

#[derive(Clone)]
pub struct AppState {
    pub timekeeper: Timekeeper,
}

impl FromRef<AppState> for Timekeeper {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.timekeeper.clone()
    }
}
