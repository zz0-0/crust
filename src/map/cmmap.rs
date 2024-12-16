use std::{collections::HashMap, hash::Hash};

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

struct CMMap<K, V> {
    entries: HashMap<K, Vec<(V, u128)>>,
}

pub enum Operation<K, V> {
    Put { key: K, value: (V, u128) },
    Remove { key: K },
}

impl<K, V> CMMap<K, V> {
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

impl<K, V> CmRDT for CMMap<K, V> {
    type Op = Operation<K, V>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::Put { key, value } => {}
            Operation::Remove { key } => {}
        }
    }
}

impl<K, V> CvRDT for CMMap<K, V> {
    fn merge(&mut self, other: &Self) {
        for (key, value) in other.entries.iter() {
            // if let Some(current_entry) = self.entries.
        }
    }
}

impl<K, V> Delta for CMMap<K, V> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K, V> Semilattice<CMMap<K, V>> for CMMap<K, V> {
    type Op = Operation<K, V>;

    fn cmrdt_associative(a: CMMap<K, V>, b: CMMap<K, V>, c: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: CMMap<K, V>, b: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: CMMap<K, V>, b: CMMap<K, V>, c: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: CMMap<K, V>, b: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: CMMap<K, V>, b: CMMap<K, V>, c: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: CMMap<K, V>, b: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: CMMap<K, V>) -> bool
    where
        CMMap<K, V>: Delta,
    {
        todo!()
    }
}
