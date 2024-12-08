use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq)]
pub struct Map<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    entries: HashMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        todo!()
    }
}

impl<K, V> CmRDT for Map<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn apply(&mut self, other: &Self) -> Self {
        todo!()
    }
}

impl<K, V> CvRDT for Map<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) -> Self {
        todo!()
    }
}

impl<K, V> Delta for Map<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) -> Self {
        todo!()
    }
}

impl<K, V> Semilattice<Map<K, V>> for Map<K, V>
where
    K: Eq + Clone + Hash,
    V: Eq + Clone + Hash,
{
    fn cmrdt_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
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
