use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};

use counter::{gcounter::GCounter, pncounter::PNCounter};
use crdt_type::{CmRDT, CvRDT, Delta};
use graph::{awgraph::AWGraph, ggraph::GGraph, orgraph::ORGraph, tpgraph::TPGraph};
use map::{cmmap::CMMap, gmap::GMap, lwwmap::LWWMap, ormap::ORMap, rmap::RMap};
use register::{lwwregister::LWWRegister, mvregister::MVRegister};
use reqwest::{Client, Response};
use sequence::{logoot::Logoot, lseq::LSeq, rga::RGA};
use serde_json::{json, Value};
use set::{awset::AWSet, gset::GSet, orset::ORSet, rwset::RWSet, tpset::TPSet};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
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
        .route("/crdt/:type/peer/state/:state", post(send_state_to_peers))
        .route("/crdt/:type/peer/delta/:delta", post(send_delta_to_peers))
        .route("/crdt/:type/operation/:operation", get(sync_operation))
        .route("/crdt/:type/state/:state", get(sync_state))
        .route("/crdt/:type/delta/:delta", get(sync_delta));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn send_operation_to_peers(
    Path((crdt_type, operation)): Path<(String, String)>,
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
        "gcounter" => {
            let mut crdt = GCounter::<String>::new();
            let ops = GCounter::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GCounter::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "pncounter" => {
            let mut crdt = PNCounter::<String>::new();
            let ops = PNCounter::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                PNCounter::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "awgraph" => {
            let mut crdt = AWGraph::<String>::new();
            let ops = AWGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                AWGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "ggraph" => {
            let mut crdt = GGraph::<String>::new();
            let ops = GGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "orgraph" => {
            let mut crdt = ORGraph::<String>::new();
            let ops = ORGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "tpgraph" => {
            let mut crdt = TPGraph::<String>::new();
            let ops = TPGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                TPGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "cmmap" => {
            let mut crdt = CMMap::<String, String>::new();
            let ops = CMMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                CMMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "gmap" => {
            let mut crdt = GMap::<String, String>::new();
            let ops = GMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "lwwmap" => {
            let mut crdt = LWWMap::<String, String>::new();
            let ops = LWWMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LWWMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "ormap" => {
            let mut crdt = ORMap::<String, String>::new();
            let ops = ORMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "rmap" => {
            let mut crdt = RMap::<String, String>::new();
            let ops = RMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "lwwregister" => {
            let mut crdt = LWWRegister::<String>::new();
            let ops = LWWRegister::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LWWRegister::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "mvregister" => {
            let mut crdt = MVRegister::<String>::new();
            let ops = MVRegister::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                MVRegister::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "logoot" => {
            let mut crdt = Logoot::<String>::new();
            let ops = Logoot::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                Logoot::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "lseq" => {
            let mut crdt = LSeq::<String>::new();
            let ops = LSeq::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LSeq::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "rga" => {
            let mut crdt = RGA::<String>::new();
            let ops = RGA::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RGA::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "awset" => {
            let mut crdt = AWSet::<String>::new();
            let ops = AWSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                AWSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "gset" => {
            let mut crdt = GSet::<String>::new();
            let ops = GSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "orset" => {
            let mut crdt = ORSet::<String>::new();
            let ops = ORSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "rwset" => {
            let mut crdt = RWSet::<String>::new();
            let ops = RWSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RWSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "tpset" => {
            let mut crdt = TPSet::<String>::new();
            let ops = TPSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                TPSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "merkeldagtree" => {
            let mut crdt = MerkleDAGTree::<String>::new();
            let ops = MerkleDAGTree::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                MerkleDAGTree::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        _ => { /* handle unknown type */ }
    };
    let client = Client::new();
    let mut responses = Vec::new();
    let pod_count = env::var("REPLICA_COUNT").unwrap().parse::<i32>().unwrap();
    let current_pod_name = env::var("POD_NAME").unwrap_or_default();
    for i in 0..pod_count {
        let target_pod_name = format!("crust-{}", i);
        let host = format!("{}.crdt-service.default.svc.cluster.local", target_pod_name);
        let addr = format!("http://{}:3000", host);
        if target_pod_name == current_pod_name {
            continue;
        }
        let url = format!(
            "{}/crdt/{}/operation/{}",
            addr,
            crdt_type,
            operation.clone()
        );
        let response = match client.get(&url).send().await {
            Ok(response) => Some(response),
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        };
        if let Some(r) = response {
            responses.push(r);
        }
    }
    if responses.is_empty() {
        return Json(json!({
            "status": "error",
            "message": "Failed to communicate with any peers"
        }));
    }
    let merge_result = verify_merged_result(result, responses);
    println!(
        "{:?}-{:?}-{:?}",
        crdt_type,
        text_operation.clone(),
        merge_result.await
    );
    Json(json!({"message": "Operation sent to peers successfully"}))
}

async fn send_state_to_peers() {}

async fn send_delta_to_peers() {}

async fn sync_operation(Path((crdt_type, operation)): Path<(String, String)>) -> Json<Value> {
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
        "gcounter" => {
            let mut crdt = GCounter::<String>::new();
            let ops = GCounter::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GCounter::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "pncounter" => {
            let mut crdt = PNCounter::<String>::new();
            let ops = PNCounter::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                PNCounter::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "awgraph" => {
            let mut crdt = AWGraph::<String>::new();
            let ops = AWGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                AWGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "ggraph" => {
            let mut crdt = GGraph::<String>::new();
            let ops = GGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "orgraph" => {
            let mut crdt = ORGraph::<String>::new();
            let ops = ORGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "tpgraph" => {
            let mut crdt = TPGraph::<String>::new();
            let ops = TPGraph::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                TPGraph::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "cmmap" => {
            let mut crdt = CMMap::<String, String>::new();
            let ops = CMMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                CMMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "gmap" => {
            let mut crdt = GMap::<String, String>::new();
            let ops = GMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "lwwmap" => {
            let mut crdt = LWWMap::<String, String>::new();
            let ops = LWWMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LWWMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "ormap" => {
            let mut crdt = ORMap::<String, String>::new();
            let ops = ORMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "rmap" => {
            let mut crdt = RMap::<String, String>::new();
            let ops = RMap::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RMap::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "lwwregister" => {
            let mut crdt = LWWRegister::<String>::new();
            let ops = LWWRegister::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LWWRegister::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "mvregister" => {
            let mut crdt = MVRegister::<String>::new();
            let ops = MVRegister::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                MVRegister::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "logoot" => {
            let mut crdt = Logoot::<String>::new();
            let ops = Logoot::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                Logoot::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "lseq" => {
            let mut crdt = LSeq::<String>::new();
            let ops = LSeq::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                LSeq::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "rga" => {
            let mut crdt = RGA::<String>::new();
            let ops = RGA::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RGA::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "awset" => {
            let mut crdt = AWSet::<String>::new();
            let ops = AWSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                AWSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "gset" => {
            let mut crdt = GSet::<String>::new();
            let ops = GSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                GSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "orset" => {
            let mut crdt = ORSet::<String>::new();
            let ops = ORSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                ORSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "rwset" => {
            let mut crdt = RWSet::<String>::new();
            let ops = RWSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                RWSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "tpset" => {
            let mut crdt = TPSet::<String>::new();
            let ops = TPSet::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                TPSet::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        "merkeldagtree" => {
            let mut crdt = MerkleDAGTree::<String>::new();
            let ops = MerkleDAGTree::convert_operation(&crdt, text_operation.clone());
            for op in ops {
                MerkleDAGTree::apply(&mut crdt, op);
            }
            result = crdt.to_string();
        }
        _ => { /* handle unknown type */ }
    };
    Json(json!(result))
}

async fn verify_merged_result(result: String, responses: Vec<Response>) -> bool {
    let mut states = Vec::new();
    states.push(result);

    for response in responses {
        match response.text().await {
            Ok(remote_state) => {
                let parsed: Value = serde_json::from_str(&remote_state).unwrap();
                let clean_json_str = serde_json::to_string(&parsed);
                match clean_json_str {
                    Ok(clean_json_str) => {
                        states.push(clean_json_str);
                    }
                    Err(e) => {
                        println!("Failed to clean JSON: {:?}", e);
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("Failed to get response text: {:?}", e);
                continue;
            }
        }
    }

    // If we have less than 2 states (including local), verification failed
    if states.len() < 2 {
        println!("Not enough responses for verification");
        return false;
    }

    // Compare all states with the first one
    let reference_state = &states[0];
    let all_match = states.iter().all(|state| state == reference_state);

    if !all_match {
        println!("State mismatch detected:");
        for (i, state) in states.iter().enumerate() {
            println!("Replica {}: {}", i, state);
        }
        return false;
    }

    println!("All states consistent: {}", reference_state);
    true
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
