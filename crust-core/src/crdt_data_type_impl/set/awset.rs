use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct AWSet<K>
where
    K: Eq + Ord,
{
    pub elements: BTreeMap<K, u128>,
    pub removed_elements: BTreeMap<K, u128>,
    pub previous_elements: BTreeMap<K, u128>,
    pub previous_removed_elements: BTreeMap<K, u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
    Remove(K, u128),
}

impl<K> AWSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            elements: BTreeMap::new(),
            removed_elements: BTreeMap::new(),
            previous_elements: BTreeMap::new(),
            previous_removed_elements: BTreeMap::new(),
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
        if let Some(&remove_ts) = self.removed_elements.get(&value) {
            if timestamp > remove_ts {
                self.removed_elements.remove(&value);
                self.elements.insert(value, timestamp);
            }
        } else {
            self.elements.insert(value, timestamp);
        }
    }

    pub fn remove(&mut self, value: K, timestamp: u128) {
        if let Some(current_timestamp) = self.elements.get(&value) {
            if timestamp > *current_timestamp {
                self.elements.remove(&value);
                self.removed_elements.insert(value, timestamp);
            }
        }
    }

    pub fn name(&self) -> String {
        "awset".to_string()
    }
}

impl<K> CmRDT for AWSet<K>
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
            Operation::Remove(ref value, timestamp) => {
                self.remove(value.clone(), timestamp);
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

impl<K> CvRDT for AWSet<K>
where
    K: Eq + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, timestamp) in &other.elements {
            if let Some(ref remove_ts) = self.removed_elements.get(key) {
                if timestamp > remove_ts {
                    self.removed_elements.remove(key);
                    self.elements.insert(key.clone(), *timestamp);
                }
            } else {
                if let Some(current_timestamp) = self.elements.get(key) {
                    if timestamp > current_timestamp {
                        self.elements.insert(key.clone(), *timestamp);
                    }
                } else {
                    self.elements.insert(key.clone(), *timestamp);
                }
            }
        }

        for (key, timestamp) in &other.removed_elements {
            if let Some(current_timestamp) = self.elements.get(key) {
                if timestamp > current_timestamp {
                    self.elements.remove(key);
                    self.removed_elements.insert(key.clone(), *timestamp);
                }
            }
        }
    }
}

impl<K> Delta for AWSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = AWSet<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = AWSet::new();
        for (key, timestamp) in &self.elements {
            match self.previous_elements.get(key) {
                Some(since_timestamp) if timestamp > since_timestamp => {
                    delta.elements.insert(key.clone(), *timestamp);
                }
                None => {
                    delta.elements.insert(key.clone(), *timestamp);
                }
                _ => {}
            }
        }
        for (key, timestamp) in &self.removed_elements {
            match self.previous_removed_elements.get(key) {
                Some(since_timestamp) if timestamp > since_timestamp => {
                    delta.removed_elements.insert(key.clone(), *timestamp);
                }
                None => {
                    delta.removed_elements.insert(key.clone(), *timestamp);
                }
                _ => {}
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        for (key, timpstamp) in &delta.elements {
            if let Some(ref remove_ts) = self.removed_elements.get(key) {
                if timpstamp > remove_ts {
                    self.removed_elements.remove(key);
                    self.elements.insert(key.clone(), *timpstamp);
                }
            } else {
                if let Some(current_timestamp) = self.elements.get(key) {
                    if timpstamp > current_timestamp {
                        self.elements.insert(key.clone(), *timpstamp);
                    }
                } else {
                    self.elements.insert(key.clone(), *timpstamp);
                }
            }
        }

        self.removed_elements.extend(delta.removed_elements.clone());
    }
}
