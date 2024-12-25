use std::sync::Mutex;

use axum::{extract::Path, Json};

use once_cell::sync::Lazy;
use serde_json::{json, Value};

use crust_core::{
    crdt_type::DataType,
    text_operation::{Message, TextOperation},
};

static CRDT_INSTANCE: Lazy<Mutex<Option<StringDataType>>> = Lazy::new(|| Mutex::new(None));

type StringDataType = DataType<String>;
type CharacterDataType = DataType<char>;
type NumberDataType = DataType<u64>;

trait DataTypeExt {
    fn get_or_create(crdt_type: String) -> Self;
}

impl DataTypeExt for StringDataType {
    fn get_or_create(crdt_type: String) -> Self {
        let mut instance = CRDT_INSTANCE.lock().unwrap();
        if instance.is_none() {
            *instance = Some(DataType::new(crdt_type));
        }
        instance.as_ref().unwrap().clone()
    }
}

pub async fn send_operation(
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
    let mut data_type: DataType<String> = DataType::get_or_create(crdt_type);
    let ops = data_type.convert_operation(text_operation);
    for op in ops {
        data_type.apply_operation(op);
    }
    Json(json!(data_type.to_string()))
}

pub async fn send_state() {}

pub async fn send_delta() {}

pub async fn info() -> Json<Value> {
    let instance = CRDT_INSTANCE.lock().unwrap();
    Json(json!(instance.as_ref().unwrap().to_string()))
}
