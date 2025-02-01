use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    get_current_timestamp,
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PNCounter<K>
where
    K: Eq + Hash,
{
    pub p: HashMap<K, u64>,
    pub n: HashMap<K, u64>,
    pub previous_p: HashMap<K, u64>,
    pub previous_n: HashMap<K, u64>,
    pub applied_ops: HashSet<u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Increment { key: K, timestamp: u128 },
    Decrement { key: K, timestamp: u128 },
}

impl<K> PNCounter<K>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            p: HashMap::new(),
            n: HashMap::new(),
            previous_p: HashMap::new(),
            previous_n: HashMap::new(),
            applied_ops: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<(HashMap<K, u64>, HashMap<K, u64>), serde_json::Error> {
        serde_json::from_str(&str)
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

    pub fn name(&self) -> String {
        "pncounter".to_string()
    }
}

impl<K> CmRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Self::Op::Increment { ref key, timestamp } => {
                if !self.applied_ops.contains(&timestamp) {
                    *self.p.entry(key.clone()).or_insert(0) += 1;
                    self.applied_ops.insert(timestamp);
                }
            }
            Self::Op::Decrement { ref key, timestamp } => {
                if !self.applied_ops.contains(&timestamp) {
                    *self.n.entry(key.clone()).or_insert(0) += 1;
                    self.applied_ops.insert(timestamp);
                }
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => {
                vec![Self::Op::Increment {
                    key: value,
                    timestamp: get_current_timestamp(),
                }]
            }
            TextOperation::Delete { position: _, value } => {
                vec![Self::Op::Decrement {
                    key: value,
                    timestamp: get_current_timestamp(),
                }]
            }
        }
    }
}

impl<K> CvRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.p {
            let current_p = self.p.entry(k.clone()).or_insert(0);
            *current_p = (*current_p).max(*v);
        }
        for (k, v) in &other.n {
            let current_n = self.n.entry(k.clone()).or_insert(0);
            *current_n = (*current_n).max(*v);
        }
        self.applied_ops.extend(other.applied_ops.iter().copied());
    }
}

impl<K> Delta for PNCounter<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = (HashMap<K, u64>, HashMap<K, u64>);

    fn generate_delta(&self) -> Self::De {
        let mut p_delta = HashMap::new();
        let mut n_delta = HashMap::new();

        for (k, v) in &self.p {
            let since_v = self.previous_p.get(k).unwrap_or(&0);
            if v > since_v {
                p_delta.insert(k.clone(), *v);
            }
        }

        for (k, v) in &self.n {
            let since_v = self.previous_n.get(k).unwrap_or(&0);
            if v > since_v {
                n_delta.insert(k.clone(), *v);
            }
        }

        (p_delta, n_delta)
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        for (k, v) in delta.clone().0 {
            let current_p = self.p.entry(k).or_insert(0);
            *current_p = (*current_p).max(v);
        }
        for (k, v) in delta.clone().1 {
            let current_n = self.n.entry(k).or_insert(0);
            *current_n = (*current_n).max(v);
        }
    }
}
