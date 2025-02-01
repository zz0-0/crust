use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RWSet<K>
where
    K: Eq + Ord + Clone,
{
    pub elements: BTreeMap<K, (u128, bool)>,
    pub previous_elements: BTreeMap<K, (u128, bool)>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
    Remove(K, u128),
}

impl<K> RWSet<K>
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
        if let Some((existing_ts, _)) = self.elements.get(&value) {
            if timestamp > *existing_ts {
                self.elements.insert(value, (timestamp, true));
            }
        } else {
            self.elements.insert(value, (timestamp, true));
        }
    }

    pub fn remove(&mut self, value: K, timestamp: u128) {
        if let Some((existing_ts, _)) = self.elements.get(&value) {
            if timestamp >= *existing_ts {
                self.elements.insert(value, (timestamp, false));
            }
        } else {
            self.elements.insert(value, (timestamp, false));
        }
    }

    pub fn name(&self) -> String {
        "rwset".to_string()
    }
}

impl<K> CmRDT for RWSet<K>
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

impl<K> CvRDT for RWSet<K>
where
    K: Eq + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, (other_ts, other_is_add)) in &other.elements {
            match self.elements.get(key) {
                Some((existing_ts, _)) => {
                    if other_ts > existing_ts || (other_ts == existing_ts && !other_is_add) {
                        // Remove wins on equal timestamps
                        self.elements
                            .insert(key.clone(), (*other_ts, *other_is_add));
                    }
                }
                None => {
                    self.elements
                        .insert(key.clone(), (*other_ts, *other_is_add));
                }
            }
        }
    }
}

impl<K> Delta for RWSet<K>
where
    K: Eq + Ord + Clone,
{
    type De = RWSet<K>;

    fn generate_delta(&self) -> Self::De {
        todo!()
    }
    fn apply_delta(&mut self, delta: &Self::De) {
        todo!()
    }
}
