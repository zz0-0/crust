use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    get_current_timestamp,
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    values: HashSet<(K, u128)>,
}

pub enum Operation<K> {
    Write { value: K },
}

impl<K> MVRegister<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            values: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_crdt(str: String) -> Self {
        serde_json::from_str(&str).unwrap()
    }

    pub fn update(&mut self, value: K) {
        self.values.insert((value, get_current_timestamp()));
    }

    pub fn values() {}
}

impl<K> CmRDT for MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Write { value } => {}
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for entry in other.values.iter() {
            if !self.values.contains(entry) {
                self.values.insert(entry.clone());
            }
        }
    }
}

impl<K> Delta for MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        todo!();
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
