use std::collections::HashMap;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

#[derive(Clone)]
pub struct Map<K, V> {
    entries: HashMap<K, V>,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<K, V> CmRDT for Map<K, V> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V> CvRDT for Map<K, V> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V> Delta for Map<K, V> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V> Semilattice<Map<K, V>> for Map<K, V>
where
    K: Eq + Clone,
    V: Eq + Clone,
{
    fn cmrdt_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = Map::new();
        // let mut b = Map::new();
        // let mut c = Map::new();

        // assert!(Map::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(Map::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(Map::cmrdt_idempotent(a.clone()));
        // assert!(Map::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(Map::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(Map::cvrdt_idempotent(a.clone()));
        // assert!(Map::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(Map::delta_commutative(a.clone(), b.clone()));
        // assert!(Map::delta_idempotent(a.clone()));
    }
}
