use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Clone, PartialEq)]
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
    fn apply(&mut self, other: &Self) -> Self {
        todo!()
    }
}

impl<T> CvRDT for AWSet<T>
where
    T: Ord + Clone,
{
    fn merge(&mut self, other: &Self) -> Self {
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

    fn apply_delta(&mut self, other: &Self) -> Self {
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
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: AWSet<T>, b: AWSet<T>) -> bool
    where
        AWSet<T>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: AWSet<T>) -> bool
    where
        AWSet<T>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: AWSet<T>, b: AWSet<T>, c: AWSet<T>) -> bool
    where
        AWSet<T>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: AWSet<T>, b: AWSet<T>) -> bool
    where
        AWSet<T>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: AWSet<T>) -> bool
    where
        AWSet<T>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: AWSet<T>, b: AWSet<T>, c: AWSet<T>) -> bool
    where
        AWSet<T>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: AWSet<T>, b: AWSet<T>) -> bool
    where
        AWSet<T>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: AWSet<T>) -> bool
    where
        AWSet<T>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
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
