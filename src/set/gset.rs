use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::{
        TextOperation, TextOperationToCmRDT, TextOperationToCvRDT, TextOperationToDelta,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GSet<K: Ord + Clone> {
    added: BTreeSet<K>,
}

pub enum Operation<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    Add(K),
}

impl<K: Ord + Clone> GSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    pub fn new() -> Self {
        Self {
            added: BTreeSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn insert(&mut self, value: K) {
        self.added.insert(value);
    }

    pub fn contains(&self, value: &K) -> bool {
        self.added.contains(value)
    }

    pub fn read(&self) -> BTreeSet<K> {
        self.added.clone()
    }
}

impl<K: Ord + Clone> CmRDT for GSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Add(value) => {
                self.insert(value);
            }
        }
    }
}

impl<K: Ord + Clone> CvRDT for GSet<K> {
    fn merge(&mut self, other: &Self) {
        self.added.extend(other.added.iter().cloned());
    }
}

impl<K: Ord + Clone> Delta for GSet<K> {
    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            added: self.added.difference(&since.added).cloned().collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> TextOperationToCmRDT for GSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT for GSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta for GSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<GSet<K>> for GSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: GSet<K>, b: GSet<K>, c: GSet<K>) -> bool
    where
        GSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: GSet<K>, b: GSet<K>) -> bool
    where
        GSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: GSet<K>) -> bool
    where
        GSet<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: GSet<K>, b: GSet<K>, c: GSet<K>) -> bool
    where
        GSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: GSet<K>, b: GSet<K>) -> bool
    where
        GSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: GSet<K>) -> bool
    where
        GSet<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: GSet<K>, b: GSet<K>, c: GSet<K>) -> bool
    where
        GSet<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: GSet<K>, b: GSet<K>) -> bool
    where
        GSet<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: GSet<K>) -> bool
    where
        GSet<K>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = GSet::new();
        // let mut b = GSet::new();
        // let mut c = GSet::new();
        // assert!(GSet::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GSet::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(GSet::cmrdt_idempotent(a.clone()));
        // assert!(GSet::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GSet::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(GSet::cvrdt_idempotent(a.clone()));
        // assert!(GSet::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GSet::delta_commutative(a.clone(), b.clone()));
        // assert!(GSet::delta_idempotent(a.clone()));
    }
}
