// Thread Safety and Data Corruption
// The implementation lacks inherent thread safety.
// To address this, consider using:
// Arc<Mutex<GCounter<K>>> for safe concurrent access.
// dashmap for concurrent HashMap operations.

use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    //
    get_current_timestamp,
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GCounter<K>
where
    K: Eq + Hash,
{
    pub counter: HashMap<K, u64>,
    pub previous: HashMap<K, u64>,
    pub applied_ops: HashSet<u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Increment { key: K, timestamp: u128 },
}

impl<K> GCounter<K>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            counter: HashMap::new(),
            previous: HashMap::new(),
            applied_ops: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<HashMap<K, u64>, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn value(&self) -> u64 {
        self.counter.values().sum()
    }

    pub fn increment(&mut self, key: K) {
        *self.counter.entry(key).or_insert(0) += 1;
    }

    pub fn name(&self) -> String {
        "gcounter".to_string()
    }
}

impl<K> CmRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Increment { ref key, timestamp } => {
                if !self.applied_ops.contains(&timestamp) {
                    *self.counter.entry(key.clone()).or_insert(0) += 1;
                    self.applied_ops.insert(timestamp);
                }
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => {
                vec![Operation::Increment {
                    key: value,
                    timestamp: get_current_timestamp(),
                }]
            }
            TextOperation::Delete { .. } => vec![],
        }
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.counter {
            let current_count = self.counter.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
    }
}

impl<K> Delta for GCounter<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = HashMap<K, u64>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = HashMap::new();
        for (k, v) in &self.counter {
            if let Some(&since_v) = self.previous.get(k) {
                if *v > since_v {
                    delta.insert(k.clone(), *v);
                }
            } else {
                delta.insert(k.clone(), *v);
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        for (k, v) in delta {
            let current = self.counter.entry(k.clone()).or_insert(0);
            *current = (*current).max(v.clone());
        }
    }
}
