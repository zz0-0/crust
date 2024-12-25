use axum::{routing::get, Router};
use std::sync::Arc;

pub mod crdt_test;

#[tokio::main]
async fn main() {
    let test_controller = Arc::new(crdt_test::TestController::new().await);
    let app = Router::new()
        .route(
            "/crust/cmrdt/:type/test/:operation",
            get({
                let controller = Arc::clone(&test_controller);
                move |req| async move { controller.test_cmrdt_semilattice(req).await }
            }),
        )
        .route(
            "/crust/cvrdt/:type/test/:state",
            get({
                let controller = Arc::clone(&test_controller);
                move |req| async move { controller.test_cvrdt_semilattice(req).await }
            }),
        )
        .route(
            "/crust/delta/:type/test/:delta",
            get({
                let controller = Arc::clone(&test_controller);
                move |req| async move { controller.test_delta_semilattice(req).await }
            }),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
