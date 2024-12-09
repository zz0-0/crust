use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Clone, PartialEq)]
pub struct TPSet<K>
where
    K: Ord,
{
    set: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
    Remove(K),
}

impl<K> TPSet<K>
where
    K: Ord,
{
    pub fn new() -> Self {
        TPSet {
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

impl<K> CmRDT for TPSet<K>
where
    K: Ord,
{
    type Op = Operation<K>;
    fn apply(&mut self, op: Self::Op) {
        todo!()
    }
}

impl<K> CvRDT for TPSet<K>
where
    K: Ord,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for TPSet<K>
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

impl<K> Semilattice<TPSet<K>> for TPSet<K>
where
    K: Ord + Clone,
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
