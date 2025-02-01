use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    get_current_timestamp,
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use std::{collections::HashSet, hash::Hash};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MVRegister<K>
where
    K: Eq + Hash,
{
    pub values: HashMap<Uuid, (K, u128)>,
    pub tombstones: HashSet<(Uuid, u128)>,
    pub previous_values: HashMap<Uuid, (K, u128)>,
    pub previous_tombstones: HashSet<(Uuid, u128)>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Write { value: K, replica_id: Uuid },
}

impl<K> MVRegister<K>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            tombstones: HashSet::new(),
            previous_values: HashMap::new(),
            previous_tombstones: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn write(&mut self, replica_id: Uuid, value: K) {
        let timestamp = self.values.get(&replica_id).map(|(_, ts)| *ts);
        if let Some(ts) = timestamp {
            self.values.remove(&replica_id);
            self.tombstones.insert((replica_id.clone(), ts));
        }
        self.values
            .insert(replica_id, (value, get_current_timestamp()));
    }

    pub fn name(&self) -> String {
        "mvregister".to_string()
    }
}

impl<K> CmRDT for MVRegister<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Write {
                ref value,
                replica_id,
            } => {
                self.write(replica_id, value.clone());
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => {
                vec![Operation::Write {
                    value,
                    replica_id: Uuid::new_v4(),
                }]
            }
            TextOperation::Delete {
                position: _,
                value: _,
            } => todo!(),
        }
    }
}

impl<K> CvRDT for MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.tombstones.extend(other.tombstones.iter());
        for (replica_id, (value, timestamp)) in &other.values {
            if !self.tombstones.contains(&(*replica_id, *timestamp)) {
                match self.values.get(replica_id) {
                    Some((_, current_timestamp)) if timestamp > current_timestamp => {
                        self.values.insert(*replica_id, (value.clone(), *timestamp));
                    }
                    None => {
                        self.values.insert(*replica_id, (value.clone(), *timestamp));
                    }
                    _ => {}
                }
            }
        }

        self.values.retain(|replica_id, (_, timestamp)| {
            !self.tombstones.contains(&(*replica_id, *timestamp))
        });
    }
}

impl<K> Delta for MVRegister<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = MVRegister<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = MVRegister::new();
        for (replica_id, (value, timestamp)) in &self.values {
            match self.previous_values.get(replica_id) {
                Some((_, since_timestamp)) if timestamp > since_timestamp => {
                    delta
                        .values
                        .insert(*replica_id, (value.clone(), *timestamp));
                }
                None => {
                    delta
                        .values
                        .insert(*replica_id, (value.clone(), *timestamp));
                }
                _ => {}
            }
        }
        for tombstone in &self.tombstones {
            if !self.previous_tombstones.contains(tombstone) {
                delta.tombstones.insert(*tombstone);
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        self.merge(&delta);
    }
}
