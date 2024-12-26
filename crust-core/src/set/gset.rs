use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GSet<K>
where
    K: Ord,
{
    added: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
}

impl<K> GSet<K>
where
    K: Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            added: BTreeSet::new(),
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

    pub fn contains(&self, value: &K) -> bool {
        self.added.contains(value)
    }

    pub fn read(&self) -> BTreeSet<K> {
        self.added.clone()
    }
}

impl<K> CmRDT for GSet<K>
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
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for GSet<K>
where
    K: Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.added.extend(other.added.iter().cloned());
    }
}

impl<K> Delta for GSet<K>
where
    K: Ord + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            added: self.added.difference(&since.added).cloned().collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
