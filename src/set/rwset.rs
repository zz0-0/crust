use std::collections::BTreeSet;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

#[derive(Clone)]
pub struct RWSet<T>
where
    T: Ord,
{
    set: BTreeSet<T>,
}

impl<T> RWSet<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        RWSet {
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

impl<T> CmRDT for RWSet<T>
where
    T: Ord,
{
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for RWSet<T>
where
    T: Ord,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for RWSet<T>
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

impl<T> Semilattice<RWSet<T>> for RWSet<T>
where
    T: Ord + Clone,
{
    fn cmrdt_associative(a: RWSet<T>, b: RWSet<T>, c: RWSet<T>) -> bool
    where
        RWSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: RWSet<T>, b: RWSet<T>) -> bool
    where
        RWSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: RWSet<T>) -> bool
    where
        RWSet<T>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: RWSet<T>, b: RWSet<T>, c: RWSet<T>) -> bool
    where
        RWSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: RWSet<T>, b: RWSet<T>) -> bool
    where
        RWSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: RWSet<T>) -> bool
    where
        RWSet<T>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: RWSet<T>, b: RWSet<T>, c: RWSet<T>) -> bool
    where
        RWSet<T>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: RWSet<T>, b: RWSet<T>) -> bool
    where
        RWSet<T>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: RWSet<T>) -> bool
    where
        RWSet<T>: Delta,
    {
        todo!()
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
