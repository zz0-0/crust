use std::time::{SystemTime, UNIX_EPOCH};

pub mod counter;
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
