use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RGA<K>
where
    K: Eq + Hash + Clone,
{
    elements: Vec<(K, usize)>,
}

pub enum Operation<K> {
    Insert { index: usize, element: K },
    Delete { index: usize },
}

impl<K> RGA<K>
where
    K: Eq + Hash + Clone + Serialize,
{
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn insert() {}

    pub fn delete() {}
}

impl<K> CmRDT for RGA<K>
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

impl<K> CvRDT for RGA<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        let mut merged = self.elements.clone();

        self.elements = merged;
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for RGA<K>
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
