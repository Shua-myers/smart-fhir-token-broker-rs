use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

pub async fn get_token_validate() -> &'static str {
    "Your token has been validated."
}

#[derive(Serialize, Deserialize)]
pub struct Healthcheck {
    status: String,
    version: String,
    uptime: String,
}

pub async fn get_health(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Healthcheck>) {
    let uptime_duration = Utc::now().signed_duration_since(state.start_time);
    let uptime_seconds = uptime_duration.num_seconds();
    let uptime = format!("{} seconds", uptime_seconds);

    (
        StatusCode::OK,
        Json(Healthcheck {
            status: "Healthy!".to_string(),
            // TODO: use a function that gets semver from somewhere else
            version: "0.0.1".to_string(),
            uptime,
        }),
    )
}

pub async fn post_token_exchange() -> &'static str {
    "Here's the access token: BLAH!"
}
