use crust_core::{delta::CrdtDelta, operation::CrdtOperation, r#type::CrdtType};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage<K>
where
    K: Eq + Hash,
{
    Operation {
        payload: CrdtOperation<K>,
        sender_pod_name: String,
    },
    Delta {
        payload: CrdtDelta<K>,
        sender_pod_name: String,
    },
    State {
        payload: CrdtType<K>,
        sender_pod_name: String,
    },
}
