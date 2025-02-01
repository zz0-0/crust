use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GSet<K>
where
    K: Eq + Ord + Clone,
{
    pub elements: BTreeMap<K, u128>,
    pub previous_elements: BTreeMap<K, u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
}

impl<K> GSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            elements: BTreeMap::new(),
            previous_elements: BTreeMap::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn insert(&mut self, value: K, timestamp: u128) {
        match self.elements.get(&value) {
            Some(&ts) if ts >= timestamp => return,
            _ => {
                self.elements.insert(value, timestamp);
            }
        }
    }

    pub fn name(&self) -> String {
        "gset".to_string()
    }
}

impl<K> CmRDT for GSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Add(ref value, timestamp) => {
                self.insert(value.clone(), timestamp);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert {
                position: _,
                value: _,
            } => vec![],
            TextOperation::Delete {
                position: _,
                value: _,
            } => vec![],
        }
    }
}

impl<K> CvRDT for GSet<K>
where
    K: Eq + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, timestamp) in other.elements.iter() {
            match self.elements.get(key) {
                Some(&ts) if ts >= *timestamp => continue,
                _ => {
                    self.elements.insert(key.clone(), *timestamp);
                }
            }
        }
    }
}

impl<K> Delta for GSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = GSet<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = GSet::new();
        for (key, timestamp) in self.elements.iter() {
            match self.previous_elements.get(key) {
                Some(&ts) if ts >= *timestamp => continue,
                _ => {
                    delta.elements.insert(key.clone(), *timestamp);
                }
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        for (key, timestamp) in delta.elements.iter() {
            match self.elements.get(key) {
                Some(&ts) if ts >= *timestamp => continue,
                _ => {
                    self.elements.insert(key.clone(), *timestamp);
                }
            }
        }
    }
}
