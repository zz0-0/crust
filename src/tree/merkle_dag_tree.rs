use std::collections::HashMap;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::{
        TextOperation, TextOperationToCmRDT, TextOperationToCvRDT, TextOperationToDelta,
    },
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    vertices: HashMap<K, Vec<u8>>,
    edges: HashMap<K, Vec<K>>,
}

pub enum Operation<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    AddVertex { hash: K, data: Vec<u8> },
    AddEdge { from: K, to: K },
}

impl<K> MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::AddVertex { hash, data } => {}
            Operation::AddEdge { from, to } => {}
        }
    }
}

impl<K> CvRDT for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> TextOperationToCmRDT for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Delta for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Semilattice<MerkleDAGTree<K>> for MerkleDAGTree<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>, c: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>, c: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>, c: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: MerkleDAGTree<K>) -> bool
    where
        MerkleDAGTree<K>: Delta,
    {
        todo!()
    }
}
