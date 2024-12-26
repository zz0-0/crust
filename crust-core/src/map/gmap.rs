use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GMap<K, V>
where
    K: Eq + Hash,
{
    entries: HashMap<K, V>,
}

pub enum Operation<K, V> {
    Put { key: K, value: V },
    Remove { key: K },
}

impl<K, V> GMap<K, V>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
    V: Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
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

    pub fn get() {}
}

impl<K, V> CmRDT for GMap<K, V>
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

impl<K, V> CvRDT for GMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, value) in other.entries.iter() {
            self.entries
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
    }
}

impl<K, V> Delta for GMap<K, V>
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
