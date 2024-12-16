use std::{collections::HashMap, hash::Hash};

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

struct GMap<K, V> {
    entries: HashMap<K, V>,
}

pub enum Operation<K, V> {
    Put { key: K, value: V },
    Remove { key: K },
}

impl<K, V> GMap<K, V> {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn value() {}

    fn put() {}

    fn get() {}
}

impl<K, V> CmRDT for GMap<K, V> {
    type Op = Operation<K, V>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Put { key, value } => {}
            Operation::Remove { key } => {}
        }
    }
}

impl<K, V> CvRDT for GMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, value) in other.entries.iter() {
            self.entries
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
    }
}

impl<K, V> Delta for GMap<K, V>
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

impl<K, V> Semilattice<GMap<K, V>> for GMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    type Op = Operation<K, V>;

    fn cmrdt_associative(a: GMap<K, V>, b: GMap<K, V>, c: GMap<K, V>) -> bool
    where
        GMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: GMap<K, V>, b: GMap<K, V>) -> bool
    where
        GMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: GMap<K, V>) -> bool
    where
        GMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: GMap<K, V>, b: GMap<K, V>, c: GMap<K, V>) -> bool
    where
        GMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: GMap<K, V>, b: GMap<K, V>) -> bool
    where
        GMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: GMap<K, V>) -> bool
    where
        GMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: GMap<K, V>, b: GMap<K, V>, c: GMap<K, V>) -> bool
    where
        GMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: GMap<K, V>, b: GMap<K, V>) -> bool
    where
        GMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: GMap<K, V>) -> bool
    where
        GMap<K, V>: Delta,
    {
        todo!()
    }
}
