use axum::{
    routing::{get, post},
    Router,
};
use receiver::{
    get_state, receive_message_from_internal, receive_message_from_other_instances, AppState,
};

pub mod message;
pub mod receiver;
pub mod sender;

static PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = Router::new()
        .route(
            "/receive/{type}",
            post(receive_message_from_other_instances),
        )
        .route(
            "/receive/internal/{crdt_type}/{sync_type}/{sync_mode}",
            post(receive_message_from_internal),
        )
        .route("/state/{type}", get(get_state))
        .with_state(state);
    let url = format!("0.0.0.0:{PORT}");
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_current_pod_name() -> String {
    std::env::var("POD_NAME").unwrap_or_else(|_| "none".to_string())
}

fn get_current_service_name() -> String {
    std::env::var("SERVICE_NAME").unwrap_or_else(|_| "none".to_string())
}
