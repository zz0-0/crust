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
pub struct AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    added: BTreeSet<K>,
    removed: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    pub fn new() -> Self {
        Self {
            added: BTreeSet::new(),
            removed: BTreeSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn insert(&mut self, value: K) {
        self.added.insert(value);
    }

    pub fn remove(&mut self, value: K) {
        self.removed.insert(value);
    }

    pub fn read(&self) -> BTreeSet<K> {
        self.added.clone()
    }
}

impl<K> CmRDT for AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Add(value) => {
                self.insert(value);
            }
            Operation::Remove(value) => {
                self.remove(value);
            }
        }
    }
}

impl<K> CvRDT for AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        self.added.extend(other.added.clone());
        self.removed.extend(other.removed.clone());
        self.added.retain(|k| !self.removed.contains(k));
    }
}

impl<K> Delta for AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            added: self.added.difference(&since.added).cloned().collect(),
            removed: self.removed.difference(&since.removed).cloned().collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> TextOperationToCmRDT<AWSet<K>> for AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT<AWSet<K>> for AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta<AWSet<K>> for AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<AWSet<K>> for AWSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: AWSet<K>, b: AWSet<K>, c: AWSet<K>) -> bool
    where
        AWSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: AWSet<K>, b: AWSet<K>) -> bool
    where
        AWSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: AWSet<K>) -> bool
    where
        AWSet<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: AWSet<K>, b: AWSet<K>, c: AWSet<K>) -> bool
    where
        AWSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: AWSet<K>, b: AWSet<K>) -> bool
    where
        AWSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: AWSet<K>) -> bool
    where
        AWSet<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: AWSet<K>, b: AWSet<K>, c: AWSet<K>) -> bool
    where
        AWSet<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: AWSet<K>, b: AWSet<K>) -> bool
    where
        AWSet<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: AWSet<K>) -> bool
    where
        AWSet<K>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = AWSet::new();
        // let mut b = AWSet::new();
        // let mut c = AWSet::new();
        // assert!(AWSet::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(AWSet::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(AWSet::cmrdt_idempotent(a.clone()));
        // assert!(AWSet::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(AWSet::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(AWSet::cvrdt_idempotent(a.clone()));
        // assert!(AWSet::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(AWSet::delta_commutative(a.clone(), b.clone()));
        // assert!(AWSet::delta_idempotent(a.clone()));
    }
}
