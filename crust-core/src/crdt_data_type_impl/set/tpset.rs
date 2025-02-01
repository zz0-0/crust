use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TPSet<K>
where
    K: Eq + Ord + Clone,
{
    pub node_id: Uuid,
    pub elements: BTreeMap<K, (u128, ElementState)>,
    pub tombstones: BTreeMap<K, HashSet<u128>>,
    pub removal_candidates: BTreeMap<K, (u128, HashSet<Uuid>)>,
    pub previous_elements: BTreeMap<K, (u128, ElementState)>,
    pub previous_tombstones: BTreeMap<K, HashSet<u128>>,
    pub previous_removal_candidates: BTreeMap<K, (u128, HashSet<Uuid>)>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ElementState {
    Active,
    MarkedForRemoval,
    Removed,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
    Remove(K, u128),
}

impl<K> TPSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            node_id: Uuid::new_v4(),
            elements: BTreeMap::new(),
            tombstones: BTreeMap::new(),
            removal_candidates: BTreeMap::new(),
            previous_elements: BTreeMap::new(),
            previous_tombstones: BTreeMap::new(),
            previous_removal_candidates: BTreeMap::new(),
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

    pub fn insert(&mut self, value: K, timestamp: u128) {
        match self.elements.get(&value) {
            Some((ts, state)) => {
                if timestamp > *ts && *state != ElementState::Removed {
                    self.elements
                        .insert(value, (timestamp, ElementState::Active));
                }
            }
            None => {
                self.elements
                    .insert(value, (timestamp, ElementState::Active));
            }
        }
    }

    pub fn prepare_remove(&mut self, value: &K, timestamp: u128) {
        if let Some((ts, state)) = self.elements.get(&value) {
            if timestamp > *ts && *state == ElementState::Active {
                self.elements
                    .insert(value.clone(), (*ts, ElementState::MarkedForRemoval));
                self.removal_candidates
                    .insert(value.clone(), (timestamp, HashSet::new()));
            }
        }
    }

    pub fn commit_remove(&mut self, value: &K, from_node: Uuid, timestamp: u128) {
        if let Some((ts, acks)) = self.removal_candidates.get_mut(&value) {
            if timestamp > *ts {
                acks.insert(from_node);
                if self.has_majority_acks(&value) {
                    self.complete_removal(value, timestamp);
                }
            }
        }
    }

    pub fn has_majority_acks(&self, value: &K) -> bool {
        if let Some((_, acks)) = self.removal_candidates.get(&value) {
            acks.len() > 3
        } else {
            false
        }
    }

    pub fn complete_removal(&mut self, value: &K, timestamp: u128) {
        if let Some((ts, _)) = self.removal_candidates.get(&value) {
            if timestamp > *ts {
                self.elements
                    .insert(value.clone(), (timestamp, ElementState::Removed));
                self.removal_candidates.remove(&value);
                self.tombstones.insert(value.clone(), HashSet::new());
            }
        }
    }

    pub fn name(&self) -> String {
        "tpset".to_string()
    }
}

impl<K> CmRDT for TPSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Add(ref value, timestamp) => {
                self.insert(value.clone(), timestamp);
            }
            Operation::Remove(ref value, timestamp) => {
                self.prepare_remove(value, timestamp);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert {
                position: _,
                value: _,
            } => vec![],
            TextOperation::Delete {
                position: _,
                value: _,
            } => vec![],
        }
    }
}

impl<K> CvRDT for TPSet<K>
where
    K: Eq + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (element, (ts, state)) in &other.elements {
            match self.elements.get(&element) {
                Some((existing_ts, existing_state)) => {
                    if ts > existing_ts {
                        self.elements
                            .insert(element.clone(), (ts.clone(), state.clone()));
                    }
                }
                None => {
                    self.elements
                        .insert(element.clone(), (ts.clone(), state.clone()));
                }
            }
        }

        for (element, acks) in &other.removal_candidates {
            match self.removal_candidates.get(element) {
                Some((ts, existing_acks)) => {
                    if acks.1.len() > existing_acks.len() {
                        self.removal_candidates
                            .insert(element.clone(), (ts.clone(), acks.1.clone()));
                    }
                }
                None => {
                    self.removal_candidates
                        .insert(element.clone(), (acks.0.clone(), acks.1.clone()));
                }
            }
        }

        for (element, tombstone) in &other.tombstones {
            match self.tombstones.get(element) {
                Some(existing_tombstone) => {
                    if tombstone.len() > existing_tombstone.len() {
                        self.tombstones.insert(element.clone(), tombstone.clone());
                    }
                }
                None => {
                    self.tombstones.insert(element.clone(), tombstone.clone());
                }
            }
        }
    }
}

impl<K> Delta for TPSet<K>
where
    K: Eq + Ord + Clone,
{
    type De = TPSet<K>;

    fn generate_delta(&self) -> Self::De {
        TPSet {
            node_id: self.node_id,
            elements: self
                .elements
                .iter()
                .filter(|(element, (ts, _))| {
                    if let Some((since_ts, _)) = self.previous_elements.get(element) {
                        ts > since_ts
                    } else {
                        true
                    }
                })
                .map(|(element, (ts, state))| (element.clone(), (ts.clone(), state.clone())))
                .collect(),
            tombstones: self
                .tombstones
                .iter()
                .filter(|(element, tombstone)| {
                    if let Some(since_tombstone) = self.previous_tombstones.get(element) {
                        tombstone.len() > since_tombstone.len()
                    } else {
                        true
                    }
                })
                .map(|(element, tombstone)| (element.clone(), tombstone.clone()))
                .collect(),
            removal_candidates: self
                .removal_candidates
                .iter()
                .filter(|(element, (ts, _))| {
                    if let Some((since_ts, _)) = self.previous_removal_candidates.get(element) {
                        ts > since_ts
                    } else {
                        true
                    }
                })
                .map(|(element, (ts, acks))| (element.clone(), (ts.clone(), acks.clone())))
                .collect(),
            previous_elements: self.elements.clone(),
            previous_tombstones: self.tombstones.clone(),
            previous_removal_candidates: self.removal_candidates.clone(),
        }
    }
    fn apply_delta(&mut self, delta: &Self::De) {
        self.merge(&delta);
    }
}
