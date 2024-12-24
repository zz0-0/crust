use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GCounter<K>
where
    K: Eq + Hash,
{
    counter: HashMap<K, u64>,
}

pub enum Operation<K> {
    Increment { key: K },
}

impl<K> GCounter<K>
where
    K: Eq + Hash + Serialize,
{
    pub fn new() -> Self {
        Self {
            counter: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn value(&self) -> u64 {
        self.counter.values().sum()
    }

    pub fn increment(&mut self, key: K) {
        *self.counter.entry(key).or_insert(0) += 1;
    }
}

impl<K> CmRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Self::Op::Increment { key } => {
                let current_count = self.counter.entry(key.clone()).or_insert(0);
                *current_count += 1;
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => {
                vec![Self::Op::Increment { key: value }]
            }
            _ => vec![],
        }
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.counter {
            let current_count = self.counter.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        todo!();
    }

    fn apply_delta(&mut self, delta: &Self) {
        self.merge(delta);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
