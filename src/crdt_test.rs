use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{crdt_type::DataType, text_operation::TextOperation};

#[derive(Serialize, Deserialize)]

pub struct Message {
    position: usize,
    text: String,
}

// cmrdt

pub fn test_cmrdt_semilattice() -> bool {
    test_cmrdt_associative() && test_cmrdt_commutative() && test_cmrdt_idempotent()
}

pub fn test_cmrdt_associative() -> bool {
    false
}

pub fn test_cmrdt_commutative() -> bool {
    false
}

pub fn test_cmrdt_idempotent() -> bool {
    false
}

pub fn send_operation_to_peers(
    Path((crdt_type, operation)): Path<(String, String)>,
    Json(message): Json<Message>,
) {
}

pub fn send_operation(
    Path((crdt_type, operation)): Path<(String, String)>,
    Json(message): Json<Message>,
) -> Json<Value> {
    let text_operation = match operation.as_str() {
        "insert" => TextOperation::Insert {
            position: message.position,
            value: message.text,
        },
        "delete" => TextOperation::Delete {
            position: message.position,
        },
        _ => TextOperation::Insert {
            position: message.position,
            value: message.text,
        },
    };
    let mut data_type: DataType<String> = DataType::new(crdt_type);
    let ops = data_type.convert_operation(text_operation);
    for op in ops {
        data_type.apply_operation(op);
    }
    Json(json!(data_type.to_string()))
}

// cvrdt

pub fn test_cvrdt_semilattice() -> bool {
    test_cvrdt_associative() && test_cvrdt_commutative() && test_cvrdt_idempotent()
}

pub fn test_cvrdt_associative() -> bool {
    false
}

pub fn test_cvrdt_commutative() -> bool {
    false
}

pub fn test_cvrdt_idempotent() -> bool {
    false
}

pub fn send_state_to_peers() {}

pub fn send_state() {}

// delta

pub fn test_delta_semilattice() -> bool {
    test_delta_associative() && test_delta_commutative() && test_delta_idempotent()
}

pub fn test_delta_associative() -> bool {
    false
}

pub fn test_delta_commutative() -> bool {
    false
}

pub fn test_delta_idempotent() -> bool {
    false
}

pub fn send_delta_to_peers() {}

pub fn send_delta() {}
