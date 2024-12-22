use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TPSet<K>
where
    K: Ord,
{
    added: BTreeSet<K>,
    removed: BTreeSet<K>,
    removed_phase: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> TPSet<K>
where
    K: Ord + Clone + Serialize,
{
    pub fn new() -> Self {
        Self {
            added: BTreeSet::new(),
            removed: BTreeSet::new(),
            removed_phase: BTreeSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn insert(&mut self, value: K) {
        self.added.insert(value.clone());
        self.removed.remove(&value.clone());
    }

    pub fn remove(&mut self, value: &K) {
        self.added.remove(&value.clone());
        self.removed.insert(value.clone());
    }
}

impl<K> CmRDT for TPSet<K>
where
    K: Ord + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        todo!()
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for TPSet<K>
where
    K: Ord + Clone,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        todo!()
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for TPSet<K>
where
    K: Ord + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            added: self.added.difference(&since.added).cloned().collect(),
            removed: self.removed.difference(&since.removed).cloned().collect(),
            removed_phase: self
                .removed_phase
                .difference(&since.removed_phase)
                .cloned()
                .collect(),
        }
    }
    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
