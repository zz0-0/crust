use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ORSet<K>
where
    K: Eq + Hash + Ord + Clone,
{
    pub elements: BTreeMap<K, HashSet<(u128, bool)>>,
    pub previous_elements: BTreeMap<K, HashSet<(u128, bool)>>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
    Remove(K, u128),
}

impl<K> ORSet<K>
where
    K: Eq + Hash + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
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
        let history = self.elements.entry(value).or_insert_with(HashSet::new);
        let is_active = history
            .iter()
            .max_by_key(|(ts, _)| ts)
            .map(|(_, active)| *active)
            .unwrap_or(false);
        if !is_active {
            history.insert((timestamp, true));
        }
    }

    pub fn remove(&mut self, value: K, timestamp: u128) {
        if let Some(history) = self.elements.get_mut(&value) {
            let is_active = history
                .iter()
                .max_by_key(|(ts, _)| ts)
                .map(|(_, active)| *active)
                .unwrap_or(false);
            if is_active {
                history.insert((timestamp, false));
            }
        }
    }

    pub fn name(&self) -> String {
        "orset".to_string()
    }
}

impl<K> CmRDT for ORSet<K>
where
    K: Eq + Hash + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
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

impl<K> CvRDT for ORSet<K>
where
    K: Eq + Hash + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (element, tags) in other.elements.iter() {
            let local_tags = self
                .elements
                .entry(element.clone())
                .or_insert_with(HashSet::new);
            local_tags.extend(tags.iter().cloned());
        }
    }
}

impl<K> Delta for ORSet<K>
where
    K: Eq + Hash + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = ORSet<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = ORSet::new();
        for (element, history) in self.elements.iter() {
            let since_history = self.previous_elements.get(element);
            let new_history: HashSet<_> = history
                .iter()
                .filter(|(ts, _)| {
                    if let Some(since_history) = since_history {
                        !since_history.contains(&(ts.clone(), true))
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
            if !new_history.is_empty() {
                delta.elements.insert(element.clone(), new_history);
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        for (element, history) in delta.elements.iter() {
            let local_history = self
                .elements
                .entry(element.clone())
                .or_insert_with(HashSet::new);
            local_history.extend(history.iter().cloned());
        }
    }
}
