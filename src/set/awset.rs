use std::collections::BTreeSet;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

#[derive(Clone)]
pub struct AWSet<T>
where
    T: Ord + Clone,
{
    set: BTreeSet<T>,
}

impl<T> AWSet<T>
where
    T: Ord + Clone,
{
    pub fn new() -> Self {
        AWSet {
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.set.insert(value);
    }

    pub fn remove(&mut self, value: &T) {
        self.set.remove(value);
    }

    pub fn read(&self) -> BTreeSet<T> {
        self.set.clone()
    }
}

impl<T> CmRDT for AWSet<T>
where
    T: Ord + Clone,
{
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for AWSet<T>
where
    T: Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for AWSet<T>
where
    T: Ord + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Semilattice<AWSet<T>> for AWSet<T>
where
    T: Ord + Clone,
{
    fn cmrdt_associative(a: AWSet<T>, b: AWSet<T>, c: AWSet<T>) -> bool
    where
        AWSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: AWSet<T>, b: AWSet<T>) -> bool
    where
        AWSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: AWSet<T>) -> bool
    where
        AWSet<T>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: AWSet<T>, b: AWSet<T>, c: AWSet<T>) -> bool
    where
        AWSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: AWSet<T>, b: AWSet<T>) -> bool
    where
        AWSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: AWSet<T>) -> bool
    where
        AWSet<T>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: AWSet<T>, b: AWSet<T>, c: AWSet<T>) -> bool
    where
        AWSet<T>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: AWSet<T>, b: AWSet<T>) -> bool
    where
        AWSet<T>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: AWSet<T>) -> bool
    where
        AWSet<T>: Delta,
    {
        todo!()
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
