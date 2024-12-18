use axum::{
    extract::Path,
    http::{response, Request},
    routing::{get, post},
    Json, Router,
};

use counter::{gcounter::GCounter, pncounter::PNCounter};
use crdt_type::{CmRDT, CvRDT, Delta};
use graph::{awgraph::AWGraph, ggraph::GGraph, orgraph::ORGraph, tpgraph::TPGraph};
use map::{cmmap::CMMap, gmap::GMap, lwwmap::LWWMap, ormap::ORMap, rmap::RMap};
use register::{lwwregister::LWWRegister, mvregister::MVRegister};
use reqwest::Client;
use sequence::{logoot::Logoot, lseq::LSeq, rga::RGA};
use serde_json::{json, Value};
use set::{awset::AWSet, gset::GSet, orset::ORSet, rwset::RWSet, tpset::TPSet};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, ops};
use text_operation::{TextOperation, TextOperationToCmRDT};
use tree::merkle_dag_tree::MerkleDAGTree;

pub mod counter;
pub mod crdt_prop;
pub mod crdt_type;
pub mod graph;
pub mod map;
pub mod register;
pub mod sequence;
pub mod set;
pub mod text_operation;
pub mod tree;

pub fn get_current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/crdt/:type/peer/operation/:operation",
            post(send_operation_to_peers),
        )
        .route("/crdt/:type/peer/state", post(send_state_to_peers))
        .route("/crdt/:type/peer/delta", post(send_delta_to_peers))
        .route("/crdt/:type/operation/:operation", get(sync_operation))
        .route("/crdt/:type/state", get(sync_state))
        .route("/crdt/:type/delta", get(sync_delta));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn send_operation_to_peers(
    Path(crdt_type): Path<String>,
    Path(operation): Path<String>,
) -> Json<Value> {
    let mut result = String::new();
    let text_operation = match operation.as_str() {
        "insert" => TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        },
        "delete" => TextOperation::Delete { position: 0 },
        _ => TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        },
    };
    match crdt_type.as_str() {
        "GCounter" => {
            let mut crdt = GCounter::<String>::new();
            let ops = GCounter::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GCounter::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "PNCounter" => {
            let mut crdt = PNCounter::<String>::new();
            let ops = PNCounter::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                PNCounter::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "AWGraph" => {
            let mut crdt = AWGraph::<String>::new();
            let ops = AWGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                AWGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "GGraph" => {
            let mut crdt = GGraph::<String>::new();
            let ops = GGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "ORGraph" => {
            let mut crdt = ORGraph::<String>::new();
            let ops = ORGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "TPGraph" => {
            let mut crdt = TPGraph::<String>::new();
            let ops = TPGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                TPGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "CMMap" => {
            let mut crdt = CMMap::<String, String>::new();
            let ops = CMMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                CMMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "GMap" => {
            let mut crdt = GMap::<String, String>::new();
            let ops = GMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "LWWMap" => {
            let mut crdt = LWWMap::<String, String>::new();
            let ops = LWWMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LWWMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "ORMap" => {
            let mut crdt = ORMap::<String, String>::new();
            let ops = ORMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "RMap" => {
            let mut crdt = RMap::<String, String>::new();
            let ops = RMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "LWWRegister" => {
            let mut crdt = LWWRegister::<String>::new();
            let ops = LWWRegister::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LWWRegister::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "MVRegister" => {
            let mut crdt = MVRegister::<String>::new();
            let ops = MVRegister::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                MVRegister::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "Logoot" => {
            let mut crdt = Logoot::<String>::new();
            let ops = Logoot::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                Logoot::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "LSeq" => {
            let mut crdt = LSeq::<String>::new();
            let ops = LSeq::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LSeq::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "RGA" => {
            let mut crdt = RGA::<String>::new();
            let ops = RGA::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RGA::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "AWSet" => {
            let mut crdt = AWSet::<String>::new();
            let ops = AWSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                AWSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "GSet" => {
            let mut crdt = GSet::<String>::new();
            let ops = GSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "ORSet" => {
            let mut crdt = ORSet::<String>::new();
            let ops = ORSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "RWSet" => {
            let mut crdt = RWSet::<String>::new();
            let ops = RWSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RWSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "TPSet" => {
            let mut crdt = TPSet::<String>::new();
            let ops = TPSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                TPSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "MerkleDAGTree" => {
            let mut crdt = MerkleDAGTree::<String>::new();
            let ops = MerkleDAGTree::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                MerkleDAGTree::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        _ => { /* handle unknown type */ }
    };
    let client = Client::new();
    let namespace = env::var("NAMESPACE").unwrap_or("default".to_string());
    let service_name = env::var("SERVICE_NAME").unwrap_or("crdt-service".to_string());
    let srv_address = format!("{}.{}.srv.cluster.local", service_name, namespace);
    let peers = tokio::net::lookup_host((srv_address.as_str(), 3000))
        .await
        .unwrap()
        .map(|addr| format!("http://{}:{}", addr.ip(), addr.port()))
        .collect::<Vec<_>>();
    let mut responses = Vec::new();
    for peer_url in peers {
        if peer_url.contains("localhost") {
            continue;
        }
        let url = format!(
            "{}/crdt/{}/operation/{}",
            peer_url,
            crdt_type,
            operation.clone()
        );
        let response = client.post(url).send().await.unwrap();
        responses.push(response);
    }
    for response in responses {
        if response.status().is_success() {
            let merge_result = verify_merged_result(result, response);
            println!(
                "{:?}-{:?}-{:?}",
                crdt_type,
                text_operation.clone(),
                merge_result
            );
        }
        return Json(json!({"error": "Failed to send operation to peers"}));
    }
    Json(json!({"message": "Operation sent to peers successfully"}))
}

async fn send_state_to_peers() {}

async fn send_delta_to_peers() {}

async fn sync_operation(
    Path(crdt_type): Path<String>,
    Path(operation): Path<String>,
) -> Json<Value> {
    let mut result = String::new();
    let operation = match operation.as_str() {
        "insert" => TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        },
        "delete" => TextOperation::Delete { position: 0 },
        _ => TextOperation::Insert {
            position: 0,
            text: "Hello".to_string(),
        },
    };
    match crdt_type.as_str() {
        "GCounter" => {
            let mut crdt = GCounter::<String>::new();
            let ops = GCounter::convert_operation(&crdt, operation);
            for op in ops {
                GCounter::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "PNCounter" => {
            let mut crdt = PNCounter::<String>::new();
            let ops = PNCounter::convert_operation(&crdt, operation);
            for op in ops {
                PNCounter::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "AWGraph" => {
            let mut crdt = AWGraph::<String>::new();
            let ops = AWGraph::convert_operation(&crdt, operation);
            for op in ops {
                AWGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "GGraph" => {
            let mut crdt = GGraph::<String>::new();
            let ops = GGraph::convert_operation(&crdt, operation);
            for op in ops {
                GGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "ORGraph" => {
            let mut crdt = ORGraph::<String>::new();
            let ops = ORGraph::convert_operation(&crdt, operation);
            for op in ops {
                ORGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "TPGraph" => {
            let mut crdt = TPGraph::<String>::new();
            let ops = TPGraph::convert_operation(&crdt, operation);
            for op in ops {
                TPGraph::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "CMMap" => {
            let mut crdt = CMMap::<String, String>::new();
            let ops = CMMap::convert_operation(&crdt, operation);
            for op in ops {
                CMMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "GMap" => {
            let mut crdt = GMap::<String, String>::new();
            let ops = GMap::convert_operation(&crdt, operation);
            for op in ops {
                GMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "LWWMap" => {
            let mut crdt = LWWMap::<String, String>::new();
            let ops = LWWMap::convert_operation(&crdt, operation);
            for op in ops {
                LWWMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "ORMap" => {
            let mut crdt = ORMap::<String, String>::new();
            let ops = ORMap::convert_operation(&crdt, operation);
            for op in ops {
                ORMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "RMap" => {
            let mut crdt = RMap::<String, String>::new();
            let ops = RMap::convert_operation(&crdt, operation);
            for op in ops {
                RMap::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "LWWRegister" => {
            let mut crdt = LWWRegister::<String>::new();
            let ops = LWWRegister::convert_operation(&crdt, operation);
            for op in ops {
                LWWRegister::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "MVRegister" => {
            let mut crdt = MVRegister::<String>::new();
            let ops = MVRegister::convert_operation(&crdt, operation);
            for op in ops {
                MVRegister::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "Logoot" => {
            let mut crdt = Logoot::<String>::new();
            let ops = Logoot::convert_operation(&crdt, operation);
            for op in ops {
                Logoot::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "LSeq" => {
            let mut crdt = LSeq::<String>::new();
            let ops = LSeq::convert_operation(&crdt, operation);
            for op in ops {
                LSeq::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "RGA" => {
            let mut crdt = RGA::<String>::new();
            let ops = RGA::convert_operation(&crdt, operation);
            for op in ops {
                RGA::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "AWSet" => {
            let mut crdt = AWSet::<String>::new();
            let ops = AWSet::convert_operation(&crdt, operation);
            for op in ops {
                AWSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "GSet" => {
            let mut crdt = GSet::<String>::new();
            let ops = GSet::convert_operation(&crdt, operation);
            for op in ops {
                GSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "ORSet" => {
            let mut crdt = ORSet::<String>::new();
            let ops = ORSet::convert_operation(&crdt, operation);
            for op in ops {
                ORSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "RWSet" => {
            let mut crdt = RWSet::<String>::new();
            let ops = RWSet::convert_operation(&crdt, operation);
            for op in ops {
                RWSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "TPSet" => {
            let mut crdt = TPSet::<String>::new();
            let ops = TPSet::convert_operation(&crdt, operation);
            for op in ops {
                TPSet::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        "MerkleDAGTree" => {
            let mut crdt = MerkleDAGTree::<String>::new();
            let ops = MerkleDAGTree::convert_operation(&crdt, operation);
            for op in ops {
                MerkleDAGTree::apply(&mut crdt, op);
                result = crdt.to_string();
            }
        }
        _ => { /* handle unknown type */ }
    };
    Json(json!({"result": result}))
}

fn verify_merged_result(result: String, response: reqwest::Response) -> bool {
    // let response_result = response.json().await.unwrap();
    // if result != response_result {
    //     panic!("Merged result is not same across all peers");
    // }

    false
}

async fn sync_state(Path(crdt_type): Path<String>) {
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

async fn sync_delta(Path(crdt_type): Path<String>) {
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
