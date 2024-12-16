use std::collections::HashMap;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

struct MerkleDAGTree<K> {
    vertices: HashMap<K, Vec<u8>>,
    edges: HashMap<K, Vec<K>>,
}

pub enum Operation<K> {
    AddVertex { hash: K, data: Vec<u8> },
    AddEdge { from: K, to: K },
}

impl<K> MerkleDAGTree<K> {
    fn add_edge() {}

    fn add_vertex() {}

    fn verify() {}
}

impl<K> CmRDT for MerkleDAGTree<K> {
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::AddVertex { hash, data } => {}
            Operation::AddEdge { from, to } => {}
        }
    }
}

impl<K> CvRDT for MerkleDAGTree<K> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for MerkleDAGTree<K> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Semilattice<MerkleDAGTree<K>> for MerkleDAGTree<K> {
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
