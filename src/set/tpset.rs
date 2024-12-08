use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Clone, PartialEq)]
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
    fn apply(&mut self, other: &Self) -> Self {
        todo!()
    }
}

impl<T> CvRDT for TPSet<T>
where
    T: Ord,
{
    fn merge(&mut self, other: &Self) -> Self {
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
    fn apply_delta(&mut self, other: &Self) -> Self {
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
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: TPSet<T>, b: TPSet<T>) -> bool
    where
        TPSet<T>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: TPSet<T>) -> bool
    where
        TPSet<T>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: TPSet<T>, b: TPSet<T>, c: TPSet<T>) -> bool
    where
        TPSet<T>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: TPSet<T>, b: TPSet<T>) -> bool
    where
        TPSet<T>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: TPSet<T>) -> bool
    where
        TPSet<T>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: TPSet<T>, b: TPSet<T>, c: TPSet<T>) -> bool
    where
        TPSet<T>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: TPSet<T>, b: TPSet<T>) -> bool
    where
        TPSet<T>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: TPSet<T>) -> bool
    where
        TPSet<T>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
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
