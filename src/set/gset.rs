use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::BTreeSet;

#[derive(Clone, PartialEq)]
pub struct GSet<K: Ord + Clone> {
    set: BTreeSet<K>,
}

pub enum Operation<K> {
    Add(K),
}

impl<K: Ord + Clone> GSet<K> {
    pub fn new() -> Self {
        GSet {
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: K) {
        self.set.insert(value);
    }

    pub fn contains(&self, value: &K) -> bool {
        self.set.contains(value)
    }

    pub fn read(&self) -> BTreeSet<K> {
        self.set.clone()
    }
}

impl<K: Ord + Clone> CmRDT for GSet<K> {
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        todo!();
    }
}

impl<K: Ord + Clone> CvRDT for GSet<K> {
    fn merge(&mut self, other: &Self) {
        self.set.extend(other.set.iter().cloned());
    }
}

impl<K: Ord + Clone> Delta for GSet<K> {
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

impl<K> Semilattice<GSet<K>> for GSet<K>
where
    K: Ord + Clone,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: GSet<K>, b: GSet<K>, c: GSet<K>) -> bool
    where
        GSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: GSet<K>, b: GSet<K>) -> bool
    where
        GSet<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: GSet<K>) -> bool
    where
        GSet<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: GSet<K>, b: GSet<K>, c: GSet<K>) -> bool
    where
        GSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: GSet<K>, b: GSet<K>) -> bool
    where
        GSet<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: GSet<K>) -> bool
    where
        GSet<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: GSet<K>, b: GSet<K>, c: GSet<K>) -> bool
    where
        GSet<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: GSet<K>, b: GSet<K>) -> bool
    where
        GSet<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: GSet<K>) -> bool
    where
        GSet<K>: Delta,
    {
        todo!();
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
