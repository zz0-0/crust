use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::CvRDTValidation,
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct AWSet<K>
where
    K: Eq + Ord,
{
    elements: BTreeMap<K, u128>,
    removed_elements: BTreeMap<K, u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
    Remove(K, u128),
}

impl<K> AWSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            elements: BTreeMap::new(),
            removed_elements: BTreeMap::new(),
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
        if let Some(&remove_ts) = self.removed_elements.get(&value) {
            if timestamp > remove_ts {
                self.removed_elements.remove(&value);
                self.elements.insert(value, timestamp);
            }
        } else {
            self.elements.insert(value, timestamp);
        }
    }

    pub fn remove(&mut self, value: K, timestamp: u128) {
        if let Some(current_timestamp) = self.elements.get(&value) {
            if timestamp > *current_timestamp {
                self.elements.remove(&value);
                self.removed_elements.insert(value, timestamp);
            }
        }
    }
}

impl<K> CmRDT for AWSet<K>
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
                self.remove(value.clone(), timestamp);
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

impl<K> CvRDT for AWSet<K>
where
    K: Eq + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, timestamp) in &other.elements {
            if let Some(ref remove_ts) = self.removed_elements.get(key) {
                if timestamp > remove_ts {
                    self.removed_elements.remove(key);
                    self.elements.insert(key.clone(), *timestamp);
                }
            } else {
                if let Some(current_timestamp) = self.elements.get(key) {
                    if timestamp > current_timestamp {
                        self.elements.insert(key.clone(), *timestamp);
                    }
                } else {
                    self.elements.insert(key.clone(), *timestamp);
                }
            }
        }

        for (key, timestamp) in &other.removed_elements {
            if let Some(current_timestamp) = self.elements.get(key) {
                if timestamp > current_timestamp {
                    self.elements.remove(key);
                    self.removed_elements.insert(key.clone(), *timestamp);
                }
            }
        }
    }
}

impl<K> Delta for AWSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = AWSet<K>;

    fn generate_delta(&self, since: &Self) -> Self::De {
        let mut delta = AWSet::new();
        for (key, timestamp) in &self.elements {
            match since.elements.get(key) {
                Some(since_timestamp) if timestamp > since_timestamp => {
                    delta.elements.insert(key.clone(), *timestamp);
                }
                None => {
                    delta.elements.insert(key.clone(), *timestamp);
                }
                _ => {}
            }
        }
        for (key, timestamp) in &self.removed_elements {
            match since.removed_elements.get(key) {
                Some(since_timestamp) if timestamp > since_timestamp => {
                    delta.removed_elements.insert(key.clone(), *timestamp);
                }
                None => {
                    delta.removed_elements.insert(key.clone(), *timestamp);
                }
                _ => {}
            }
        }
        delta
    }

    fn merge_delta(&mut self, delta: Self::De) {
        for (key, timpstamp) in &delta.elements {
            if let Some(ref remove_ts) = self.removed_elements.get(key) {
                if timpstamp > remove_ts {
                    self.removed_elements.remove(key);
                    self.elements.insert(key.clone(), *timpstamp);
                }
            } else {
                if let Some(current_timestamp) = self.elements.get(key) {
                    if timpstamp > current_timestamp {
                        self.elements.insert(key.clone(), *timpstamp);
                    }
                } else {
                    self.elements.insert(key.clone(), *timpstamp);
                }
            }
        }

        self.removed_elements.extend(delta.removed_elements);
    }
}

impl<K> CvRDTValidation<AWSet<K>> for AWSet<K>
where
    K: Eq + Ord + Clone + Debug,
{
    fn cvrdt_associativity(a: AWSet<K>, b: AWSet<K>, c: AWSet<K>) -> bool {
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

    fn cvrdt_commutativity(a: AWSet<K>, b: AWSet<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: AWSet<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

#[cfg(test)]
mod tests {
    use crate::get_current_timestamp;

    use super::*;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = AWSet::<String>::new();
        let mut b = AWSet::<String>::new();
        let mut c = AWSet::<String>::new();

        let timestamp = get_current_timestamp();

        a.insert("a".to_string(), timestamp);
        b.insert("b".to_string(), timestamp + 1);
        c.insert("c".to_string(), timestamp + 2);
        a.remove("a".to_string(), timestamp + 3);

        assert!(AWSet::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(AWSet::<String>::cvrdt_commutativity(a.clone(), b.clone()));
        assert!(AWSet::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let mut a = AWSet::<String>::new();
        let mut b = AWSet::<String>::new();
        let mut c = AWSet::<String>::new();

        let timestamp = get_current_timestamp();

        a.insert("a".to_string(), timestamp);
        b.insert("b".to_string(), timestamp + 1);
        c.insert("c".to_string(), timestamp + 2);
        a.remove("a".to_string(), timestamp + 3);
    }

    #[test]
    fn test_delta_validation() {
        let mut a = AWSet::<String>::new();
        let mut b = AWSet::<String>::new();
        let mut c = AWSet::<String>::new();

        let timestamp = get_current_timestamp();

        a.insert("a".to_string(), timestamp);
        b.insert("b".to_string(), timestamp + 1);
        c.insert("c".to_string(), timestamp + 2);
        a.remove("a".to_string(), timestamp + 3);
    }
}
