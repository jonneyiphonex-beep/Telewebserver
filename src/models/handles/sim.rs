use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::services::sim_manager::SimManager;
use crate::state::TelecomEngine;
use std::sync::Arc;

pub async fn switch_sim(
    State(engine): State<Arc<TelecomEngine>>,
    Path(slot_id): Path<u8>,
) -> Result<Json<String>, StatusCode> {
    if slot_id == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    SimManager::switch_active_slot(&engine, slot_id);
    Ok(Json(format!("Active SIM switched to slot {}", slot_id)))
}