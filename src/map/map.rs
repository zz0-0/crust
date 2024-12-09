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

pub enum Operation<K, V> {
    Insert { key: K, value: V },
    Remove { key: K },
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
    type Op = Operation<K, V>;

    fn apply(&mut self, op: Self::Op) {
        todo!()
    }
}

impl<K, V> CvRDT for Map<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
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

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V> Semilattice<Map<K, V>> for Map<K, V>
where
    K: Eq + Clone + Hash,
    V: Eq + Clone + Hash,
    Self: CmRDT<Op = Operation<K, V>>,
{
    type Op = Operation<K, V>;

    fn cmrdt_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: Map<K, V>, b: Map<K, V>, c: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: Map<K, V>, b: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: Map<K, V>) -> bool
    where
        Map<K, V>: Delta,
    {
        todo!();
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
