use axum::{routing::get, Router};
use crdt_request::{info, send_delta, send_operation, send_state};

mod crdt_request;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/crust/:type/operation/:operation", get(send_operation))
        .route("/crust/:type/state/:state", get(send_state))
        .route("/crust/:type/delta/:delta", get(send_delta))
        .route("/crust/info", get(info));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
