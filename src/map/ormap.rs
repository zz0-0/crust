use serde::{Deserialize, Serialize};

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::{
        TextOperation, TextOperationToCmRDT, TextOperationToCvRDT, TextOperationToDelta,
    },
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ORMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    entries: HashMap<K, (V, u128)>,
    tombstone: HashSet<(K, u128)>,
}

pub enum Operation<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    Put { key: K, value: (V, u128) },
    Remove { key: K },
}

impl<K, V> ORMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            tombstone: HashSet::new(),
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

impl<K, V> CmRDT for ORMap<K, V>
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

impl<K, V> CvRDT for ORMap<K, V>
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

        for tombstone in other.tombstone.iter() {
            if !self.tombstone.contains(tombstone) {
                self.tombstone.insert(tombstone.clone());
            }
        }
    }
}

impl<K, V> Delta for ORMap<K, V>
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

impl<K, V> TextOperationToCmRDT for ORMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K, V>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K, V> TextOperationToCvRDT for ORMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K, V> TextOperationToDelta for ORMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K, V> Semilattice<ORMap<K, V>> for ORMap<K, V>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    V: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K, V>;

    fn cmrdt_associative(a: ORMap<K, V>, b: ORMap<K, V>, c: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: ORMap<K, V>, b: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: ORMap<K, V>, b: ORMap<K, V>, c: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: ORMap<K, V>, b: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: ORMap<K, V>, b: ORMap<K, V>, c: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: ORMap<K, V>, b: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: ORMap<K, V>) -> bool
    where
        ORMap<K, V>: Delta,
    {
        todo!()
    }
}
