use std::collections::HashMap;

use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MerkleDAGTree<K>
where
    K: Eq + Hash,
{
    vertices: HashMap<K, Vec<u8>>,
    edges: HashMap<K, Vec<K>>,
}

pub enum Operation<K> {
    AddVertex { hash: K, data: Vec<u8> },
    AddEdge { from: K, to: K },
}

impl<K> MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Serialize,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn add_edge() {}

    pub fn add_vertex() {}

    pub fn verify() {}
}

impl<K> CmRDT for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::AddVertex { hash, data } => {}
            Operation::AddEdge { from, to } => {}
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        todo!()
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
