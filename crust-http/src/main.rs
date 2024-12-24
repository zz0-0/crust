use axum::{routing::get, Router};
use crdt_request::send_operation;

mod crdt_request;

#[tokio::main]
async fn main() {
    let app = Router::new()
    //     .route("/crdt/cmrdt/test", get(test_cmrdt_semilattice))
    //     .route(
    //         "/crdt/:type/peer/operation/:operation",
    //         post(send_operation_to_peers),
    //     )
    //     .route("/crdt/:type/peer/state/:state", post(send_state_to_peers))
    //     .route("/crdt/:type/peer/delta/:delta", post(send_delta_to_peers))
        .route("/crdt/:type/operation/:operation", get(send_operation))
    //     .route("/crdt/:type/state/:state", get(sync_state))
    //     .route("/crdt/:type/delta/:delta", get(sync_delta));
    ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
