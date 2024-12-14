use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
pub struct RWSet<K>
where
    K: Ord + Clone,
{
    added: BTreeSet<K>,
    removed: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> RWSet<K>
where
    K: Ord + Clone,
{
    pub fn new() -> Self {
        RWSet {
            added: BTreeSet::new(),
            removed: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: K) {
        self.added.insert(value.clone());
        self.removed.remove(&value.clone());
    }

    pub fn remove(&mut self, value: &K) {
        self.removed.insert(value.clone());
        self.added.remove(&value.clone());
    }
}

impl<K> CmRDT for RWSet<K>
where
    K: Ord + Clone,
{
    type Op = Operation<K>;
    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Add(value) => {
                self.insert(value);
            }
            Operation::Remove(value) => {
                self.remove(&value);
            }
        }
    }
}

impl<K> CvRDT for RWSet<K>
where
    K: Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.added.extend(other.added.clone());
        self.removed.extend(other.removed.clone());
        self.added.retain(|k| !self.removed.contains(k));
    }
}

impl<K> Delta for RWSet<K>
where
    K: Ord + Clone,
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

impl<K> Semilattice<RWSet<K>> for RWSet<K>
where
    K: Ord + Clone + Clone,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: RWSet<K>, b: RWSet<K>, c: RWSet<K>) -> bool
    where
        RWSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: RWSet<K>, b: RWSet<K>) -> bool
    where
        RWSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: RWSet<K>) -> bool
    where
        RWSet<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: RWSet<K>, b: RWSet<K>, c: RWSet<K>) -> bool
    where
        RWSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: RWSet<K>, b: RWSet<K>) -> bool
    where
        RWSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: RWSet<K>) -> bool
    where
        RWSet<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: RWSet<K>, b: RWSet<K>, c: RWSet<K>) -> bool
    where
        RWSet<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: RWSet<K>, b: RWSet<K>) -> bool
    where
        RWSet<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: RWSet<K>) -> bool
    where
        RWSet<K>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = RWSet::new();
        // let mut b = RWSet::new();
        // let mut c = RWSet::new();
        // assert!(RWSet::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(RWSet::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(RWSet::cmrdt_idempotent(a.clone()));
        // assert!(RWSet::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(RWSet::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(RWSet::cvrdt_idempotent(a.clone()));
        // assert!(RWSet::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(RWSet::delta_commutative(a.clone(), b.clone()));
        // assert!(RWSet::delta_idempotent(a.clone()));
    }
}
