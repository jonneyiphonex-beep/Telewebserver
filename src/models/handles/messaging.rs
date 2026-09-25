use axum::Json;
use crate::models::messaging::MessagePayload;
use crate::services::rcs_engine::RcsEngine;

pub async fn send_message(Json(msg): Json<MessagePayload>) -> Json<String> {
    let status = RcsEngine::route_message(&msg);
    Json(status)
}