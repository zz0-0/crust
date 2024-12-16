use std::{collections::HashMap, hash::Hash};

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

struct LWWMap<K, V> {
    entries: HashMap<K, (V, u128)>,
}

pub enum Operation<K, V> {
    Put { key: K, value: (V, u128) },
    Remove { key: K },
}

impl<K, V> LWWMap<K, V> {
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

impl<K, V> CmRDT for LWWMap<K, V> {
    type Op = Operation<K, V>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Put { key, value } => {}
            Operation::Remove { key } => {}
        }
    }
}

impl<K, V> CvRDT for LWWMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, (value, timestamp)) in other.entries.iter() {
            match self.entries.get(key) {
                Some((_, current_timestamp)) => {
                    if timestamp > current_timestamp {
                        self.entries
                            .insert(key.clone(), (value.clone(), *timestamp));
                    }
                }
                None => {
                    self.entries
                        .insert(key.clone(), (value.clone(), *timestamp));
                }
            }
        }
    }
}

impl<K, V> Delta for LWWMap<K, V>
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

impl<K, V> Semilattice<LWWMap<K, V>> for LWWMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    type Op = Operation<K, V>;

    fn cmrdt_associative(a: LWWMap<K, V>, b: LWWMap<K, V>, c: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: LWWMap<K, V>, b: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: LWWMap<K, V>, b: LWWMap<K, V>, c: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: LWWMap<K, V>, b: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: LWWMap<K, V>, b: LWWMap<K, V>, c: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: LWWMap<K, V>, b: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: LWWMap<K, V>) -> bool
    where
        LWWMap<K, V>: Delta,
    {
        todo!()
    }
}
