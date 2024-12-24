use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Logoot<K>
where
    K: Clone,
{
    elements: Vec<(K, usize)>,
}

pub enum Operation<K> {
    Insert { index: usize, element: K },
    Delete { index: usize },
}

impl<K> Logoot<K>
where
    K: Clone + Serialize,
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

    // pub fn position() -> usize {}
}

impl<K> CmRDT for Logoot<K>
where
    K: Clone,
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

impl<K> CvRDT for Logoot<K>
where
    K: Clone + Ord,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        let mut merged = self.elements.clone();

        for (pos, value) in other.elements.iter() {
            if !merged.contains(&(pos.clone(), value.clone())) {
                let index = merged
                    .iter()
                    .position(|(p, v)| p > pos)
                    .unwrap_or(merged.len());
                merged.insert(index, (pos.clone(), value.clone()));
            }
        }

        self.elements = merged;
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for Logoot<K>
where
    K: Clone + Ord,
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
