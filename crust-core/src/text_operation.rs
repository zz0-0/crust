use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum TextOperation<K> {
    Insert { position: usize, value: K },
    Delete { position: usize },
}

#[derive(Serialize, Deserialize)]

pub struct Message {
    pub position: usize,
    pub text: String,
}

// pub trait TextOperationToCmRDT
// where
//     K: CmRDT,
// {
//     type Op;
//     fn convert_operation(&self, op: TextOperation) -> Vec<Self::Op>;
// }

// pub trait TextOperationToCvRDT
// where
//     K: CvRDT,
// {
//     fn convert_state(&self, op: TextOperation);
// }

// pub trait TextOperationToDelta
// where
//     K: Delta,
// {
//     fn convert_delta(&self, op: TextOperation);
// }
