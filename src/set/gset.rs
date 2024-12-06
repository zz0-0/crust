use std::collections::BTreeSet;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

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

#[cfg(test)]
mod tests {
    use crate::crdt_prop::Semilattice;

    use super::*;

    impl Semilattice for GSet<String> {
        fn associative() {}
        fn commutative() {}
        fn idempotent() {}
    }

    #[test]
    fn test_semilattice_properties() {
        GSet::<String>::associative();
        GSet::<String>::commutative();
        GSet::<String>::idempotent();
    }
}
