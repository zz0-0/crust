use serde::{Deserialize, Serialize};

use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ORMap<K, V>
where
    K: Eq + Hash,
{
    entries: HashMap<K, (V, u128)>,
    tombstone: HashSet<(K, u128)>,
}

pub enum Operation<K, V> {
    Put { key: K, value: (V, u128) },
    Remove { key: K },
}

impl<K, V> ORMap<K, V>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
    V: Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            tombstone: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_crdt(str: String) -> Self {
        serde_json::from_str(&str).unwrap()
    }

    pub fn value() {}

    pub fn put() {}

    pub fn remove() {}

    pub fn get() {}
}

impl<K, V> CmRDT for ORMap<K, V>
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

impl<K, V> CvRDT for ORMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, (value, timestamp)) in other.entries.iter() {
            match self.entries.get(key) {
                Some((_, current_timestamp)) => {
                    if timestamp > current_timestamp {
                        self.entries
                            .insert(key.clone(), (value.clone(), *timestamp));
                    }
                }
                None => {
                    self.entries
                        .insert(key.clone(), (value.clone(), *timestamp));
                }
            }
        }

        for tombstone in other.tombstone.iter() {
            if !self.tombstone.contains(tombstone) {
                self.tombstone.insert(tombstone.clone());
            }
        }
    }
}

impl<K, V> Delta for ORMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
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
