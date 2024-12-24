use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CMMap<K, V>
where
    K: Eq + Hash,
{
    entries: HashMap<K, Vec<(V, u128)>>,
}

pub enum Operation<K, V> {
    Put { key: K, value: (V, u128) },
    Remove { key: K },
}

impl<K, V> CMMap<K, V>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
{
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn value() {}

    pub fn put() {}

    pub fn remove() {}

    pub fn get() {}
}

impl<K, V> CmRDT for CMMap<K, V>
where
    K: Eq + Hash,
{
    type Op = Operation<K, V>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Put { key, value } => {}
            Operation::Remove { key } => {}
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K, V> CvRDT for CMMap<K, V>
where
    K: Eq + Hash,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        for (key, value) in other.entries.iter() {
            // if let Some(current_entry) = self.entries.
        }
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K, V> Delta for CMMap<K, V>
where
    K: Eq + Hash,
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