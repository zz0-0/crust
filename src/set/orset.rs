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
pub struct ORSet<K>
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

impl<K> ORSet<K>
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
        self.removed.insert(value.clone());
        self.added.remove(&value.clone());
    }
}

impl<K> CmRDT for ORSet<K>
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

impl<K> CvRDT for ORSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        self.added.extend(other.added.clone());
        self.removed.extend(other.removed.clone());
        self.added.retain(|k| !self.removed.contains(k));
    }
}

impl<K> Delta for ORSet<K>
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

impl<K> TextOperationToCmRDT<ORSet<K>> for ORSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT<ORSet<K>> for ORSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta<ORSet<K>> for ORSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<ORSet<K>> for ORSet<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: ORSet<K>, b: ORSet<K>, c: ORSet<K>) -> bool
    where
        ORSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: ORSet<K>, b: ORSet<K>) -> bool
    where
        ORSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: ORSet<K>) -> bool
    where
        ORSet<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: ORSet<K>, b: ORSet<K>, c: ORSet<K>) -> bool
    where
        ORSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: ORSet<K>, b: ORSet<K>) -> bool
    where
        ORSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: ORSet<K>) -> bool
    where
        ORSet<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: ORSet<K>, b: ORSet<K>, c: ORSet<K>) -> bool
    where
        ORSet<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: ORSet<K>, b: ORSet<K>) -> bool
    where
        ORSet<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: ORSet<K>) -> bool
    where
        ORSet<K>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = ORSet::new();
        // let mut b = ORSet::new();
        // let mut c = ORSet::new();
        // assert!(ORSet::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(ORSet::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(ORSet::cmrdt_idempotent(a.clone()));
        // assert!(ORSet::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(ORSet::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(ORSet::cvrdt_idempotent(a.clone()));
        // assert!(ORSet::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(ORSet::delta_commutative(a.clone(), b.clone()));
        // assert!(ORSet::delta_idempotent(a.clone()));
    }
}
