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
pub struct LSeq<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    elements: Vec<(K, u64)>,
}

pub enum Operation<K> {
    Insert { index: usize, element: K },
    Delete { index: usize },
}

impl<K> LSeq<K>
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

impl<K> CmRDT for LSeq<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Insert { index, element } => {}
            Operation::Delete { index } => {}
        }
    }
}

impl<K> CvRDT for LSeq<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        let mut merged = self.elements.clone();

        self.elements = merged;
    }
}

impl<K> Delta for LSeq<K>
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

impl<K> TextOperationToCmRDT for LSeq<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT for LSeq<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta for LSeq<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<LSeq<K>> for LSeq<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: LSeq<K>, b: LSeq<K>, c: LSeq<K>) -> bool
    where
        LSeq<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: LSeq<K>, b: LSeq<K>) -> bool
    where
        LSeq<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: LSeq<K>) -> bool
    where
        LSeq<K>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: LSeq<K>, b: LSeq<K>, c: LSeq<K>) -> bool
    where
        LSeq<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: LSeq<K>, b: LSeq<K>) -> bool
    where
        LSeq<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: LSeq<K>) -> bool
    where
        LSeq<K>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: LSeq<K>, b: LSeq<K>, c: LSeq<K>) -> bool
    where
        LSeq<K>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: LSeq<K>, b: LSeq<K>) -> bool
    where
        LSeq<K>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: LSeq<K>) -> bool
    where
        LSeq<K>: Delta,
    {
        todo!()
    }
}
