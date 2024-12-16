use axum::{
    extract::Path,
    http::Request,
    routing::{get, post},
    Router,
};

use crdt_type::{CmRDT, CvRDT, Delta};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

mod counter;
mod crdt_prop;
mod crdt_type;
mod graph;
mod map;
mod register;
mod sequence;
mod set;
mod text_operation;
mod tree;

pub fn get_current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/crdt/:type/state", get(receive_state_from_peers))
        .route("/crdt/:type/apply", post(send_operation_to_peers))
        .route("/crdt/:type/state", post(send_state_to_peers))
        .route("/crdt/:type/delta", post(send_delta_to_peers))
        .route("/crdt/instances", get(get_instances));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn send_operation_to_peers(Path(crdt_type): Path<String>) {
    match crdt_type.as_str() {
        "AWGraph" => { /* handle AWGraph */ }
        "AWSet" => { /* handle AWSet */ }
        "GCounter" => { /* handle GCounter */ }
        "GGraph" => { /* handle GGraph */ }
        "GSet" => { /* handle GSet */ }
        "LWWRegister" => { /* handle LWWRegister */ }
        "MVRegister" => { /* handle MVRegister */ }
        "ORGraph" => { /* handle ORGraph */ }
        _ => { /* handle unknown type */ }
    }

    let service_name = "crdt-pods";
    let namespace = "default"; // Use your actual namespace
    let pods_count = 3; // Assume you know how many pods are there (or fetch this dynamically)

    // List of pods (constructed dynamically from DNS)
    let mut pods = Vec::new();
    for i in 1..=pods_count {
        let pod_dns = format!(
            "crdt-pod-{}.{}.{}.svc.cluster.local",
            i, service_name, namespace
        );
        pods.push(pod_dns);
    }

    for pod in pods {
        let request = Request::builder()
            .method("POST")
            .uri("http://localhost:3000/crdt/:type/apply")
            .body(())
            .unwrap();
    }
}

async fn send_state_to_peers(Path(crdt_type): Path<String>) {
    match crdt_type.as_str() {
        "AWGraph" => { /* handle AWGraph */ }
        "AWSet" => { /* handle AWSet */ }
        "GCounter" => { /* handle GCounter */ }
        "GGraph" => { /* handle GGraph */ }
        "GSet" => { /* handle GSet */ }
        "LWWRegister" => { /* handle LWWRegister */ }
        "MVRegister" => { /* handle MVRegister */ }
        "ORGraph" => { /* handle ORGraph */ }
        _ => { /* handle unknown type */ }
    }
}

async fn send_delta_to_peers(Path(crdt_type): Path<String>) {
    match crdt_type.as_str() {
        "AWGraph" => { /* handle AWGraph */ }
        "AWSet" => { /* handle AWSet */ }
        "GCounter" => { /* handle GCounter */ }
        "GGraph" => { /* handle GGraph */ }
        "GSet" => { /* handle GSet */ }
        "LWWRegister" => { /* handle LWWRegister */ }
        "MVRegister" => { /* handle MVRegister */ }
        "ORGraph" => { /* handle ORGraph */ }
        _ => { /* handle unknown type */ }
    }
}

async fn receive_state_from_peers() {}

async fn get_instances() {}
