use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LSeq<K> {
    elements: Vec<(K, u64)>,
}

pub enum Operation {
    Insert { index: usize, element: () },
    Delete { index: usize },
}

impl<K> LSeq<K> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn insert() {}

    pub fn delete() {}
}

impl<K> CmRDT for LSeq<K> {
    type Op = Operation;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Insert { index, element } => {}
            Operation::Delete { index } => {}
        }
    }
}

impl<K> CvRDT for LSeq<K>
where
    K: Clone,
{
    fn merge(&mut self, other: &Self) {
        let mut merged = self.elements.clone();

        self.elements = merged;
    }
}

impl<K> Delta for LSeq<K>
where
    K: Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> Semilattice<LSeq<K>> for LSeq<K>
where
    K: Clone,
{
    type Op = Operation;

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
