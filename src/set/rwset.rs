use std::collections::BTreeSet;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

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

#[cfg(test)]
mod tests {
    use crate::crdt_prop::Semilattice;

    use super::*;

    impl Semilattice for RWSet<String> {
        fn associative() {}
        fn commutative() {}
        fn idempotent() {}
    }

    #[test]
    fn test_semilattice_properties() {
        RWSet::<String>::associative();
        RWSet::<String>::commutative();
        RWSet::<String>::idempotent();
    }
}
