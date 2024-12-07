use std::collections::BTreeSet;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

#[derive(Clone)]
pub struct TPSet<T>
where
    T: Ord,
{
    set: BTreeSet<T>,
}

impl<T> TPSet<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        TPSet {
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

impl<T> CmRDT for TPSet<T>
where
    T: Ord,
{
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for TPSet<T>
where
    T: Ord,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for TPSet<T>
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

impl<T> Semilattice<TPSet<T>> for TPSet<T>
where
    T: Ord + Clone,
{
    fn cmrdt_associative(a: TPSet<T>, b: TPSet<T>, c: TPSet<T>) -> bool
    where
        TPSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: TPSet<T>, b: TPSet<T>) -> bool
    where
        TPSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: TPSet<T>) -> bool
    where
        TPSet<T>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: TPSet<T>, b: TPSet<T>, c: TPSet<T>) -> bool
    where
        TPSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: TPSet<T>, b: TPSet<T>) -> bool
    where
        TPSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: TPSet<T>) -> bool
    where
        TPSet<T>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: TPSet<T>, b: TPSet<T>, c: TPSet<T>) -> bool
    where
        TPSet<T>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: TPSet<T>, b: TPSet<T>) -> bool
    where
        TPSet<T>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: TPSet<T>) -> bool
    where
        TPSet<T>: Delta,
    {
        todo!()
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
