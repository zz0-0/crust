use std::collections::HashMap;
use std::hash::Hash;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

struct RMap<K, V> {
    entries: HashMap<K, (V, u128)>,
}

pub enum Operation<K, V> {
    Put { key: K, value: (V, u128) },
    Remove { key: K },
}

impl<K, V> RMap<K, V> {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn value() {}

    fn put() {}

    fn remove() {}

    fn get() {}
}

impl<K, V> CmRDT for RMap<K, V> {
    type Op = Operation<K, V>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Put { key, value } => {}
            Operation::Remove { key } => {}
        }
    }
}

impl<K, V> CvRDT for RMap<K, V> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V> Delta for RMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K, V> Semilattice<RMap<K, V>> for RMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    type Op = Operation<K, V>;

    fn cmrdt_associative(a: RMap<K, V>, b: RMap<K, V>, c: RMap<K, V>) -> bool
    where
        RMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: RMap<K, V>, b: RMap<K, V>) -> bool
    where
        RMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: RMap<K, V>) -> bool
    where
        RMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: RMap<K, V>, b: RMap<K, V>, c: RMap<K, V>) -> bool
    where
        RMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: RMap<K, V>, b: RMap<K, V>) -> bool
    where
        RMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: RMap<K, V>) -> bool
    where
        RMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: RMap<K, V>, b: RMap<K, V>, c: RMap<K, V>) -> bool
    where
        RMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: RMap<K, V>, b: RMap<K, V>) -> bool
    where
        RMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: RMap<K, V>) -> bool
    where
        RMap<K, V>: Delta,
    {
        todo!()
    }
}
