static CRDT_INSTANCE: Lazy<Mutex<Option<StringDataType>>> = Lazy::new(|| Mutex::new(None));

type StringDataType = DataType<String>;
// type CharacterDataType = DataType<char>;
// type NumberDataType = DataType<u64>;

pub trait DataTypeExt {
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

use std::sync::Mutex;

use axum::{extract::Path, response::IntoResponse, Json};
use crust_core::{
    crdt_benchmark::{CRDTBenchmark, SingleInsertEnd},
    crdt_type::DataType,
    get_current_timestamp,
    text_operation::{Message, TextOperation},
};
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde_json::json;

pub async fn send_operation(
    Path((crdt_type, operation)): Path<(String, String)>,
    Json(message): Json<Message>,
) -> impl IntoResponse {
    let text_operation = match operation.as_str() {
        "insert" => TextOperation::Insert {
            position: message.position,
            value: message.text,
        },
        "delete" => TextOperation::Delete {
            position: message.position,
            value: message.text,
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
    (StatusCode::OK, Json(json!(data_type.to_string())))
}

pub async fn send_operation_with_timestamp(
    Path((crdt_type, operation)): Path<(String, String)>,
    Json(message): Json<Message>,
) -> impl IntoResponse {
    let text_operation = match operation.as_str() {
        "insert" => TextOperation::Insert {
            position: message.position,
            value: message.text,
        },
        "delete" => TextOperation::Delete {
            position: message.position,
            value: message.text,
        },
        _ => TextOperation::Insert {
            position: message.position,
            value: message.text,
        },
    };
    let mut data_type: DataType<String> = DataType::get_or_create(crdt_type);
    let ops = data_type.convert_operation(text_operation);
    let timestamp1 = get_current_timestamp();
    for op in ops {
        data_type.apply_operation(op);
    }
    let timestamp2 = get_current_timestamp();
    (
        StatusCode::OK,
        Json(json!({
            "state": data_type.to_string(),
            "timestamp": {
                "start": timestamp1,
                "end": timestamp2
            },
            "timespend": timestamp2 - timestamp1
        })),
    )
}

pub async fn send_state(Path((crdt_type, state)): Path<(String, String)>) -> impl IntoResponse {
    let mut data_type: DataType<String> = DataType::get_or_create(crdt_type.clone());
    let mut data_type2: DataType<String> = DataType::new(crdt_type.clone());
    data_type2.to_crdt(state);
    data_type.merge(&data_type2);
    (StatusCode::OK, Json(json!(data_type.to_string())))
}

pub async fn send_state_with_timestamp(
    Path((crdt_type, state)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut data_type: DataType<String> = DataType::get_or_create(crdt_type.clone());
    let mut data_type2: DataType<String> = DataType::new(crdt_type.clone());
    data_type2.to_crdt(state);
    let timestamp1 = get_current_timestamp();
    data_type.merge(&data_type2);
    let timestamp2 = get_current_timestamp();
    (
        StatusCode::OK,
        Json(json!({
            "state": data_type.to_string(),
            "timestamp": {
                "start": timestamp1,
                "end": timestamp2
            },
            "timespend": timestamp2 - timestamp1
        })),
    )
}

pub async fn send_delta(Path((crdt_type, delta)): Path<(String, String)>) -> impl IntoResponse {
    let mut data_type: DataType<String> = DataType::get_or_create(crdt_type.clone());
    let mut data_type2 = DataType::new(crdt_type.clone());
    let delta = data_type2.to_delta(delta);
    data_type.apply_delta(&delta);
    (StatusCode::OK, Json(json!(data_type.to_string())))
}

pub async fn send_delta_with_timestamp(
    Path((crdt_type, delta)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut data_type: DataType<String> = DataType::get_or_create(crdt_type.clone());
    let mut data_type2 = DataType::new(crdt_type.clone());
    let delta = data_type2.to_delta(delta);
    let timestamp1 = get_current_timestamp();
    data_type.apply_delta(&delta);
    let timestamp2 = get_current_timestamp();
    (
        StatusCode::OK,
        Json(json!({
            "state": data_type.to_string(),
            "timestamp": {
                "start": timestamp1,
                "end": timestamp2
            },
            "timespend": timestamp2 - timestamp1
        })),
    )
}

pub async fn info() -> impl IntoResponse {
    let instance = CRDT_INSTANCE.lock().unwrap();
    (
        StatusCode::OK,
        Json(json!(instance.as_ref().unwrap().to_string())),
    )
}

pub async fn send_benchmark(
    Path((crdt_type, iterations)): Path<(String, u32)>,
) -> impl IntoResponse {
    let mut data_type: DataType<String> = DataType::get_or_create(crdt_type);
    SingleInsertEnd::benchmark_cmrdt_result(
        &SingleInsertEnd,
        &mut data_type,
        "a".to_string(),
        iterations,
    );

    (StatusCode::OK)
}
