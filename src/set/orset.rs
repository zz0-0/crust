use std::collections::BTreeSet;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

#[derive(Clone)]
pub struct ORSet<T>
where
    T: Ord,
{
    set: BTreeSet<T>,
}

impl<T> ORSet<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        ORSet {
            set: BTreeSet::new(),
        }
    }
    pub fn insert(&mut self, value: T) {
        self.set.insert(value);
    }

    pub fn remove(&mut self, value: &T) {
        self.set.remove(value);
    }
}

impl<T> CmRDT for ORSet<T>
where
    T: Ord,
{
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for ORSet<T>
where
    T: Ord,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for ORSet<T>
where
    T: Ord,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Semilattice<ORSet<T>> for ORSet<T>
where
    T: Ord + Clone,
{
    fn cmrdt_associative(a: ORSet<T>, b: ORSet<T>, c: ORSet<T>) -> bool
    where
        ORSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: ORSet<T>, b: ORSet<T>) -> bool
    where
        ORSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: ORSet<T>) -> bool
    where
        ORSet<T>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: ORSet<T>, b: ORSet<T>, c: ORSet<T>) -> bool
    where
        ORSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: ORSet<T>, b: ORSet<T>) -> bool
    where
        ORSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: ORSet<T>) -> bool
    where
        ORSet<T>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: ORSet<T>, b: ORSet<T>, c: ORSet<T>) -> bool
    where
        ORSet<T>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: ORSet<T>, b: ORSet<T>) -> bool
    where
        ORSet<T>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: ORSet<T>) -> bool
    where
        ORSet<T>: Delta,
    {
        todo!()
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
