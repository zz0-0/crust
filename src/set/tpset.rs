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
pub struct TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    added: BTreeSet<K>,
    removed: BTreeSet<K>,
    removed_phase: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    pub fn new() -> Self {
        Self {
            added: BTreeSet::new(),
            removed: BTreeSet::new(),
            removed_phase: BTreeSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn insert(&mut self, value: K) {
        self.added.insert(value.clone());
        self.removed.remove(&value.clone());
    }

    pub fn remove(&mut self, value: &K) {
        self.added.remove(&value.clone());
        self.removed.insert(value.clone());
    }
}

impl<K> CmRDT for TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;
    fn apply(&mut self, op: Self::Op) {
        todo!()
    }
}

impl<K> CvRDT for TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            added: self.added.difference(&since.added).cloned().collect(),
            removed: self.removed.difference(&since.removed).cloned().collect(),
            removed_phase: self
                .removed_phase
                .difference(&since.removed_phase)
                .cloned()
                .collect(),
        }
    }
    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> TextOperationToCmRDT for TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT for TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta for TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<TPSet<K>> for TPSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: TPSet<K>, b: TPSet<K>, c: TPSet<K>) -> bool
    where
        TPSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: TPSet<K>, b: TPSet<K>) -> bool
    where
        TPSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: TPSet<K>) -> bool
    where
        TPSet<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: TPSet<K>, b: TPSet<K>, c: TPSet<K>) -> bool
    where
        TPSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: TPSet<K>, b: TPSet<K>) -> bool
    where
        TPSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: TPSet<K>) -> bool
    where
        TPSet<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: TPSet<K>, b: TPSet<K>, c: TPSet<K>) -> bool
    where
        TPSet<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: TPSet<K>, b: TPSet<K>) -> bool
    where
        TPSet<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: TPSet<K>) -> bool
    where
        TPSet<K>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = TPSet::new();
        // let mut b = TPSet::new();
        // let mut c = TPSet::new();
        // assert!(TPSet::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(TPSet::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(TPSet::cmrdt_idempotent(a.clone()));
        // assert!(TPSet::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(TPSet::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(TPSet::cvrdt_idempotent(a.clone()));
        // assert!(TPSet::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(TPSet::delta_commutative(a.clone(), b.clone()));
        // assert!(TPSet::delta_idempotent(a.clone()));
    }
}
