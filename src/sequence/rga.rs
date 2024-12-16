use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RGA<K> {
    elements: Vec<(K, usize)>,
}

pub enum Operation {
    Insert { index: usize, element: () },
    Delete { index: usize },
}

impl<K> RGA<K> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn insert() {}

    pub fn delete() {}
}

impl<K> CmRDT for RGA<K> {
    type Op = Operation;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Insert { index, element } => {}
            Operation::Delete { index } => {}
        }
    }
}

impl<K> CvRDT for RGA<K>
where
    K: Clone,
{
    fn merge(&mut self, other: &Self) {
        let mut merged = self.elements.clone();

        self.elements = merged;
    }
}

impl<K> Delta for RGA<K>
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

impl<K> Semilattice<RGA<K>> for RGA<K>
where
    K: Clone,
{
    type Op = Operation;

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
