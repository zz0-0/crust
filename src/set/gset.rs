use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Clone, PartialEq)]
pub struct GSet<T: Ord + Clone> {
    set: BTreeSet<T>,
}

impl<T: Ord + Clone> GSet<T> {
    pub fn new() -> Self {
        GSet {
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.set.insert(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    pub fn read(&self) -> BTreeSet<T> {
        self.set.clone()
    }
}

impl<T: Ord + Clone> CmRDT for GSet<T> {
    fn apply(&mut self, other: &Self) -> Self {
        for element in other.set.iter() {
            if !self.set.contains(element) {
                self.set.insert(element.clone());
            }
        }
        self.clone()
    }
}

impl<T: Ord + Clone> CvRDT for GSet<T> {
    fn merge(&mut self, other: &Self) -> Self {
        self.set.extend(other.set.iter().cloned());
        self.clone()
    }
}

impl<T: Ord + Clone> Delta for GSet<T> {
    fn generate_delta(&self, since: &Self) -> Self {
        let mut delta = GSet::new();
        for element in self.set.iter() {
            if !since.set.contains(element) {
                delta.insert(element.clone());
            }
        }
        delta
    }

    fn apply_delta(&mut self, other: &Self) -> Self {
        self.set.extend(other.set.iter().cloned());
        self.clone()
    }
}

impl<T> Semilattice<GSet<T>> for GSet<T>
where
    T: Ord + Clone,
{
    fn cmrdt_associative(a: GSet<T>, b: GSet<T>, c: GSet<T>) -> bool
    where
        GSet<T>: CmRDT,
    {
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: GSet<T>, b: GSet<T>) -> bool
    where
        GSet<T>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: GSet<T>) -> bool
    where
        GSet<T>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: GSet<T>, b: GSet<T>, c: GSet<T>) -> bool
    where
        GSet<T>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: GSet<T>, b: GSet<T>) -> bool
    where
        GSet<T>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: GSet<T>) -> bool
    where
        GSet<T>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: GSet<T>, b: GSet<T>, c: GSet<T>) -> bool
    where
        GSet<T>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: GSet<T>, b: GSet<T>) -> bool
    where
        GSet<T>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: GSet<T>) -> bool
    where
        GSet<T>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = GSet::new();
        // let mut b = GSet::new();
        // let mut c = GSet::new();
        // assert!(GSet::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GSet::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(GSet::cmrdt_idempotent(a.clone()));
        // assert!(GSet::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GSet::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(GSet::cvrdt_idempotent(a.clone()));
        // assert!(GSet::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GSet::delta_commutative(a.clone(), b.clone()));
        // assert!(GSet::delta_idempotent(a.clone()));
    }
}
