use std::time::{SystemTime, UNIX_EPOCH};

pub mod crdt_benchmark;
pub mod crdt_data_type;
pub mod crdt_data_type_impl;
pub mod crdt_sync_type;
pub mod crdt_validation;
pub mod crdt_validation_impl;
pub mod text_operation;

pub fn get_current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}
