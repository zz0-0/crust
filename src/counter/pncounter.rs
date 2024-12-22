use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PNCounter<K>
where
    K: Eq + Hash,
{
    p: HashMap<K, u64>,
    n: HashMap<K, u64>,
}

pub enum Operation<K> {
    Increment { key: K, value: u64 },
    Decrement { key: K, value: u64 },
}

impl<K> PNCounter<K>
where
    K: Eq + Hash + Serialize,
{
    pub fn new() -> Self {
        Self {
            p: HashMap::new(),
            n: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn value(&self) -> i64 {
        let p: u64 = self.p.values().sum();
        let n: u64 = self.n.values().sum();
        p as i64 - n as i64
    }

    pub fn increment(&mut self, key: K) {
        *self.p.entry(key).or_insert(0) += 1;
    }

    pub fn decrement(&mut self, key: K) {
        *self.n.entry(key).or_insert(0) += 1;
    }
}

impl<K> CmRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Self::Op::Increment { key, value } => {
                let current_count = self.p.entry(key.clone()).or_insert(0);
                *current_count = (*current_count).max(value);
            }
            Self::Op::Decrement { key, value } => {
                let current_count = self.n.entry(key.clone()).or_insert(0);
                *current_count = (*current_count).max(value);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.p {
            let current_count = self.p.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
        for (k, v) in &other.n {
            let current_count = self.n.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        todo!();
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
