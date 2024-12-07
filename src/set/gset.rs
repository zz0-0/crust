use std::collections::BTreeSet;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

#[derive(Clone)]
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
    fn apply(&mut self, other: &Self) {
        for element in other.set.iter() {
            if !self.set.contains(element) {
                self.set.insert(element.clone());
            }
        }
    }
}

impl<T: Ord + Clone> CvRDT for GSet<T> {
    fn merge(&mut self, other: &Self) {
        self.set.extend(other.set.iter().cloned());
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

    fn apply_delta(&mut self, other: &Self) {
        self.set.extend(other.set.iter().cloned());
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
        todo!()
    }

    fn cmrdt_commutative(a: GSet<T>, b: GSet<T>) -> bool
    where
        GSet<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: GSet<T>) -> bool
    where
        GSet<T>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: GSet<T>, b: GSet<T>, c: GSet<T>) -> bool
    where
        GSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: GSet<T>, b: GSet<T>) -> bool
    where
        GSet<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: GSet<T>) -> bool
    where
        GSet<T>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: GSet<T>, b: GSet<T>, c: GSet<T>) -> bool
    where
        GSet<T>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: GSet<T>, b: GSet<T>) -> bool
    where
        GSet<T>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: GSet<T>) -> bool
    where
        GSet<T>: Delta,
    {
        todo!()
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
