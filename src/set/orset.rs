use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Clone, PartialEq)]
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
    fn apply(&mut self, other: &Self) -> Self {
        todo!()
    }
}

impl<T> CvRDT for ORSet<T>
where
    T: Ord,
{
    fn merge(&mut self, other: &Self) -> Self {
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

    fn apply_delta(&mut self, other: &Self) -> Self {
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
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: ORSet<T>, b: ORSet<T>) -> bool
    where
        ORSet<T>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: ORSet<T>) -> bool
    where
        ORSet<T>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: ORSet<T>, b: ORSet<T>, c: ORSet<T>) -> bool
    where
        ORSet<T>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: ORSet<T>, b: ORSet<T>) -> bool
    where
        ORSet<T>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: ORSet<T>) -> bool
    where
        ORSet<T>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: ORSet<T>, b: ORSet<T>, c: ORSet<T>) -> bool
    where
        ORSet<T>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: ORSet<T>, b: ORSet<T>) -> bool
    where
        ORSet<T>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: ORSet<T>) -> bool
    where
        ORSet<T>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
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
