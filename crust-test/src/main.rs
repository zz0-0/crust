use axum::{routing::get, Router};

pub mod crdt_test;

#[tokio::main]
async fn main() {
    let test_controller = crdt_test::TestController::new().await;
    let app = Router::new()
    // .route(
    //     "/crdt/cmrdt/test",
    //     get(test_controller.test_cmrdt_semilattice(_)),
    // )
    ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
