use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    get_current_timestamp,
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LWWRegister<K>
where
    K: Clone,
{
    value: Option<K>,
    timestamp: u128,
}

pub enum Operation<K> {
    Set(K, u128),
}

impl<K> LWWRegister<K>
where
    K: Clone + Serialize,
{
    pub fn new() -> Self {
        Self {
            value: None,
            timestamp: 0,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn read(&self) -> Option<K> {
        self.value.clone()
    }

    pub fn update(value: K) -> Self {
        LWWRegister {
            value: Some(value),
            timestamp: get_current_timestamp(),
        }
    }

    pub fn get() {}
}

impl<K> CmRDT for LWWRegister<K>
where
    K: Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Set(value, timestamp) => {
                self.merge(&LWWRegister {
                    value: Some(value),
                    timestamp,
                });
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for LWWRegister<K>
where
    K: Clone,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        match self.timestamp.cmp(&other.timestamp) {
            std::cmp::Ordering::Less => {
                self.value = other.value.clone();
                self.timestamp = other.timestamp;
            }
            std::cmp::Ordering::Equal => {
                self.value = other.value.clone();
            }
            _ => {}
        }
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for LWWRegister<K>
where
    K: Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
