use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
pub struct AWSet<K>
where
    K: Ord + Clone,
{
    set: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> AWSet<K>
where
    K: Ord + Clone,
{
    pub fn new() -> Self {
        AWSet {
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: K) {
        self.set.insert(value);
    }

    pub fn remove(&mut self, value: &K) {
        self.set.remove(value);
    }

    pub fn read(&self) -> BTreeSet<K> {
        self.set.clone()
    }
}

impl<K> CmRDT for AWSet<K>
where
    K: Ord + Clone,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        todo!()
    }
}

impl<K> CvRDT for AWSet<K>
where
    K: Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for AWSet<K>
where
    K: Ord + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Semilattice<AWSet<K>> for AWSet<K>
where
    K: Ord + Clone,
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
