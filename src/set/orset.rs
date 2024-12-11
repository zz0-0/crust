use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
pub struct ORSet<K>
where
    K: Ord,
{
    set: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> ORSet<K>
where
    K: Ord,
{
    pub fn new() -> Self {
        ORSet {
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: K) {
        self.set.insert(value);
    }

    pub fn remove(&mut self, value: &K) {
        self.set.remove(value);
    }
}

impl<K> CmRDT for ORSet<K>
where
    K: Ord,
{
    type Op = Operation<K>;
    fn apply(&mut self, op: Self::Op) {
        todo!()
    }
}

impl<K> CvRDT for ORSet<K>
where
    K: Ord,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for ORSet<K>
where
    K: Ord,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Semilattice<ORSet<K>> for ORSet<K>
where
    K: Ord + Clone,
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
