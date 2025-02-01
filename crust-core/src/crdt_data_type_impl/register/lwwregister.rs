use crate::{
    crdt_sync_type::{CmRDT, CvRDT},
    get_current_timestamp,
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct LWWRegister<K> {
    pub value: Option<K>,
    pub timestamp: u128,
    pub replica_id: Uuid,
}

#[derive(Clone)]
pub enum Operation<K> {
    Set(K, u128, Uuid),
}

impl<K> LWWRegister<K>
where
    K: Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            value: None,
            timestamp: get_current_timestamp(),
            replica_id: Uuid::new_v4(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn update(&mut self, value: K) {
        self.value = value.into();
        self.timestamp = get_current_timestamp();
    }

    pub fn name(&self) -> String {
        "lwwregister".to_string()
    }
}

impl<K> CmRDT for LWWRegister<K>
where
    K: Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Set(ref value, timestamp, ref replica_id) => {
                self.merge(&LWWRegister {
                    value: value.clone().into(),
                    timestamp,
                    replica_id: *replica_id,
                });
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => vec![Operation::Set(
                value,
                get_current_timestamp(),
                Uuid::new_v4(),
            )],
            TextOperation::Delete {
                position: _,
                value: _,
            } => todo!(),
        }
    }
}

impl<K> CvRDT for LWWRegister<K>
where
    K: Clone,
{
    fn merge(&mut self, other: &Self) {
        if self.timestamp < other.timestamp
            || (other.timestamp == self.timestamp && other.replica_id > self.replica_id)
        {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
            self.replica_id = other.replica_id
        }
    }
}
