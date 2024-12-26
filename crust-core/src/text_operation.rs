use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum TextOperation<K> {
    Insert { position: usize, value: K },
    Delete { position: usize, value: K },
}

#[derive(Serialize, Deserialize)]

pub struct Message {
    pub position: usize,
    pub text: String,
}
