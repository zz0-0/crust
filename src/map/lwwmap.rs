use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::{
        TextOperation, TextOperationToCmRDT, TextOperationToCvRDT, TextOperationToDelta,
    },
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LWWMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    entries: HashMap<K, (V, u128)>,
}

pub enum Operation<K, V> {
    Put { key: K, value: (V, u128) },
    Remove { key: K },
}

impl<K, V> LWWMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn value() {}

    pub fn put() {}

    pub fn remove() {}

    pub fn get() {}
}

impl<K, V> CmRDT for LWWMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K, V> TextOperationToCmRDT for LWWMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K, V>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K, V> TextOperationToCvRDT for LWWMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K, V> TextOperationToDelta for LWWMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K, V> Semilattice<LWWMap<K, V>> for LWWMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
