use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
pub struct RWSet<K>
where
    K: Ord,
{
    set: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> RWSet<K>
where
    K: Ord,
{
    pub fn new() -> Self {
        RWSet {
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

impl<K> CmRDT for RWSet<K>
where
    K: Ord,
{
    type Op = Operation<K>;
    fn apply(&mut self, op: Self::Op) {
        todo!()
    }
}

impl<K> CvRDT for RWSet<K>
where
    K: Ord,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for RWSet<K>
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

impl<K> Semilattice<RWSet<K>> for RWSet<K>
where
    K: Ord + Clone,
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
