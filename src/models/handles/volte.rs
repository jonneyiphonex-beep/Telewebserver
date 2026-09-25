use axum::{extract::State, Json};
use crate::services::volte_switcher::VolteSwitcher;
use crate::state::TelecomEngine;
use std::sync::Arc;

pub async fn handle_volte_switch(
    State(engine): State<Arc<TelecomEngine>>,
    Json(is_in_call): Json<bool>,
) -> Json<String> {
    let result = VolteSwitcher::set_volte_state(&engine, is_in_call);
    Json(result.to_string())
}