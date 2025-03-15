use crust_core::{delta::CrdtDelta, operation::CrdtOperation, r#type::CrdtType};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage<K>
where
    K: Eq + Hash,
{
    Operation {
        operation: CrdtOperation<K>,
        sender_pod_name: String,
    },
    Delta {
        delta: CrdtDelta<K>,
        sender_pod_name: String,
    },
    State {
        state: CrdtType<K>,
        sender_pod_name: String,
    },
}
