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
pub struct RGA<K> {
    elements: Vec<(K, usize)>,
}

pub enum Operation<K> {
    Insert { index: usize, element: K },
    Delete { index: usize },
}

impl<K> RGA<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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

impl<K> CmRDT for RGA<K> {
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Insert { index, element } => {}
            Operation::Delete { index } => {}
        }
    }
}

impl<K> CvRDT for RGA<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        let mut merged = self.elements.clone();

        self.elements = merged;
    }
}

impl<K> Delta for RGA<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> TextOperationToCmRDT<RGA<K>> for RGA<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT<RGA<K>> for RGA<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta<RGA<K>> for RGA<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<RGA<K>> for RGA<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: RGA<K>, b: RGA<K>, c: RGA<K>) -> bool
    where
        RGA<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: RGA<K>, b: RGA<K>) -> bool
    where
        RGA<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: RGA<K>) -> bool
    where
        RGA<K>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: RGA<K>, b: RGA<K>, c: RGA<K>) -> bool
    where
        RGA<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: RGA<K>, b: RGA<K>) -> bool
    where
        RGA<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: RGA<K>) -> bool
    where
        RGA<K>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: RGA<K>, b: RGA<K>, c: RGA<K>) -> bool
    where
        RGA<K>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: RGA<K>, b: RGA<K>) -> bool
    where
        RGA<K>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: RGA<K>) -> bool
    where
        RGA<K>: Delta,
    {
        todo!()
    }
}
