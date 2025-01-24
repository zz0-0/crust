use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{de::value, Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Debug;
use uuid::{timestamp, Uuid};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TPSet<K>
where
    K: Eq + Ord + Clone,
{
    node_id: Uuid,
    elements: BTreeMap<K, (u128, ElementState)>,
    tombstones: BTreeMap<K, HashSet<u128>>,
    removal_candidates: BTreeMap<K, (u128, HashSet<Uuid>)>,
    previous_elements: BTreeMap<K, (u128, ElementState)>,
    previous_tombstones: BTreeMap<K, HashSet<u128>>,
    previous_removal_candidates: BTreeMap<K, (u128, HashSet<Uuid>)>,
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

    fn name(&self) -> String {
        "PNCounter".to_string()
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

    fn name(&self) -> String {
        "PNCounter".to_string()
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
    fn merge_delta(&mut self, delta: &Self::De) {
        self.merge(&delta);
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> CvRDTValidation<TPSet<K>> for TPSet<K>
where
    K: Eq + Ord + Clone + Debug,
{
    fn cvrdt_associativity(a: TPSet<K>, b: TPSet<K>, c: TPSet<K>) -> bool {
        let mut ab_c = a.clone();
        ab_c.merge(&b);
        let mut bc = b.clone();
        bc.merge(&c);
        ab_c.merge(&c);
        let mut a_bc = a.clone();
        a_bc.merge(&bc);
        println!("{:?} {:?}", ab_c, a_bc);
        ab_c == a_bc
    }

    fn cvrdt_commutativity(a: TPSet<K>, b: TPSet<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: TPSet<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<TPSet<K>> for TPSet<K>
where
    K: Eq + Ord + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
{
    fn cmrdt_commutativity(
        a: TPSet<K>,
        op1: <TPSet<K> as CmRDT>::Op,
        op2: <TPSet<K> as CmRDT>::Op,
    ) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op2);
        let mut a2 = a.clone();
        a2.apply(&op2);
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_idempotence(a: TPSet<K>, op1: <TPSet<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(a: TPSet<K>, ops: Vec<<TPSet<K> as CmRDT>::Op>) -> bool {
        let mut a1 = a.clone();
        for op in &ops {
            a1.apply(op);
        }

        let mut rng = rand::thread_rng();
        let mut ops_permuted = ops.clone();
        for _ in 0..5 {
            ops_permuted.shuffle(&mut rng);
            let mut a2 = a.clone();
            for op in &ops_permuted {
                a2.apply(op);
            }
            if a1 != a2 {
                return false;
            }
        }
        true
    }
}

impl<K> DeltaValidation<TPSet<K>> for TPSet<K>
where
    K: Eq + Ord + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
{
    fn delta_associativity(
        a: TPSet<K>,
        de1: <TPSet<K> as Delta>::De,
        de2: <TPSet<K> as Delta>::De,
        de3: <TPSet<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de2.clone());
        a1.merge_delta(&de3.clone());

        let mut a2 = a.clone();
        let mut combined_delta = TPSet {
            node_id: de2.node_id,
            elements: de2.elements.clone(),
            tombstones: de2.tombstones.clone(),
            removal_candidates: de2.removal_candidates.clone(),
            previous_elements: de2.previous_elements.clone(),
            previous_tombstones: de2.previous_tombstones.clone(),
            previous_removal_candidates: de2.previous_removal_candidates.clone(),
        };

        todo!();

        a2.merge_delta(&de1);
        a2.merge_delta(&combined_delta);

        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_commutativity(
        a: TPSet<K>,
        de1: <TPSet<K> as Delta>::De,
        de2: <TPSet<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de2.clone());
        let mut a2 = a.clone();
        a2.merge_delta(&de2);
        a2.merge_delta(&de1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_idempotence(a: TPSet<K>, de1: <TPSet<K> as Delta>::De) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de1.clone());
        let mut a2 = a.clone();
        a2.merge_delta(&de1.clone());
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = TPSet::<String>::new();
        let mut b = TPSet::<String>::new();
        let mut c = TPSet::<String>::new();

        todo!();

        assert!(TPSet::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(TPSet::<String>::cvrdt_commutativity(a.clone(), b.clone()));
        assert!(TPSet::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {}

    #[test]
    fn test_delta_validation() {}
}
