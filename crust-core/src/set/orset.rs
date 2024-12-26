use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ORSet<K>
where
    K: Ord,
{
    added: BTreeSet<K>,
    removed: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> ORSet<K>
where
    K: Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            added: BTreeSet::new(),
            removed: BTreeSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_crdt(str: String) -> Self {
        serde_json::from_str(&str).unwrap()
    }

    pub fn insert(&mut self, value: K) {
        self.added.insert(value);
    }

    pub fn remove(&mut self, value: K) {
        self.removed.insert(value.clone());
        self.added.remove(&value.clone());
    }
}

impl<K> CmRDT for ORSet<K>
where
    K: Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Add(value) => {
                self.insert(value);
            }
            Operation::Remove(value) => {
                self.remove(value);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for ORSet<K>
where
    K: Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.added.extend(other.added.clone());
        self.removed.extend(other.removed.clone());
        self.added.retain(|k| !self.removed.contains(k));
    }
}

impl<K> Delta for ORSet<K>
where
    K: Ord + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            added: self.added.difference(&since.added).cloned().collect(),
            removed: self.removed.difference(&since.removed).cloned().collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
