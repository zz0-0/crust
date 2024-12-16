use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Logoot<K> {
    elements: Vec<(K, usize)>,
}

pub enum Operation {
    Insert { index: usize, element: () },
    Delete { index: usize },
}

impl<K> Logoot<K> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn insert() {}

    pub fn delete() {}

    // pub fn position() -> usize {}
}

impl<K> CmRDT for Logoot<K> {
    type Op = Operation;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Insert { index, element } => {}
            Operation::Delete { index } => {}
        }
    }
}

impl<K> CvRDT for Logoot<K>
where
    K: Clone + PartialEq + PartialOrd,
{
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
}

impl<K> Delta for Logoot<K>
where
    K: Clone + PartialEq + PartialOrd,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> Semilattice<Logoot<K>> for Logoot<K>
where
    K: Clone + PartialEq + PartialOrd,
{
    type Op = Operation;

    fn cmrdt_associative(a: Logoot<K>, b: Logoot<K>, c: Logoot<K>) -> bool
    where
        Logoot<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: Logoot<K>, b: Logoot<K>) -> bool
    where
        Logoot<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: Logoot<K>) -> bool
    where
        Logoot<K>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: Logoot<K>, b: Logoot<K>, c: Logoot<K>) -> bool
    where
        Logoot<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: Logoot<K>, b: Logoot<K>) -> bool
    where
        Logoot<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: Logoot<K>) -> bool
    where
        Logoot<K>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: Logoot<K>, b: Logoot<K>, c: Logoot<K>) -> bool
    where
        Logoot<K>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: Logoot<K>, b: Logoot<K>) -> bool
    where
        Logoot<K>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: Logoot<K>) -> bool
    where
        Logoot<K>: Delta,
    {
        todo!()
    }
}
