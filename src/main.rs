mod config;
#[path = "models/handles/mod.rs"]
mod handlers;
mod models;
#[path = "models/services/mod.rs"]
mod services;
mod state;
#[path = "models/services/utils/mod.rs"]
mod utils;

use axum::{
    http::header,
    response::Html,
    routing::{get, post},
    Router,
};
use state::TelecomEngine;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // تهيئة حالة المحرك وتغليفها برابط ذكي غير متزامن
    let shared_state = Arc::new(TelecomEngine::new());

    let app = Router::new()
        .route("/", get(index))
        .route("/styles.css", get(stylesheet))
        .route("/app.js", get(script))
        .route("/sim/switch/:slot_id", post(handlers::sim::switch_sim))
        .route("/volte/data-switch", post(handlers::volte::handle_volte_switch))
        .route("/apn/config", post(handlers::apn::configure_apn))
        .route("/messaging/send", post(handlers::messaging::send_message))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Telecom Micro-Server running on port 8080...");
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../web/index.html"))
}

async fn stylesheet() -> impl axum::response::IntoResponse {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], include_str!("../web/styles.css"))
}

async fn script() -> impl axum::response::IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], include_str!("../web/app.js"))
}