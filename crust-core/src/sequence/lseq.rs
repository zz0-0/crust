use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LSeq<K>
where
    K: Eq + Hash + Clone,
{
    elements: Vec<(K, u64)>,
}

pub enum Operation<K> {
    Insert { index: usize, element: K },
    Delete { index: usize },
}

impl<K> LSeq<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_crdt(str: String) -> Self {
        serde_json::from_str(&str).unwrap()
    }

    pub fn insert() {}

    pub fn delete() {}
}

impl<K> CmRDT for LSeq<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Insert { index, element } => {}
            Operation::Delete { index } => {}
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for LSeq<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        let mut merged = self.elements.clone();

        self.elements = merged;
    }
}

impl<K> Delta for LSeq<K>
where
    K: Eq + Hash + Clone,
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
