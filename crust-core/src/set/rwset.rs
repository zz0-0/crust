use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::CvRDTValidation,
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use uuid::timestamp;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RWSet<K>
where
    K: Eq + Ord + Clone,
{
    elements: BTreeMap<K, (u128, bool)>,
    previous_elements: BTreeMap<K, (u128, bool)>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
    Remove(K, u128),
}

impl<K> RWSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            elements: BTreeMap::new(),
            previous_elements: BTreeMap::new(),
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
        if let Some((existing_ts, _)) = self.elements.get(&value) {
            if timestamp > *existing_ts {
                self.elements.insert(value, (timestamp, true));
            }
        } else {
            self.elements.insert(value, (timestamp, true));
        }
    }

    pub fn remove(&mut self, value: K, timestamp: u128) {
        if let Some((existing_ts, _)) = self.elements.get(&value) {
            if timestamp >= *existing_ts {
                self.elements.insert(value, (timestamp, false));
            }
        } else {
            self.elements.insert(value, (timestamp, false));
        }
    }
}

impl<K> CmRDT for RWSet<K>
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

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> CvRDT for RWSet<K>
where
    K: Eq + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, (other_ts, other_is_add)) in &other.elements {
            match self.elements.get(key) {
                Some((existing_ts, _)) => {
                    if other_ts > existing_ts || (other_ts == existing_ts && !other_is_add) {
                        // Remove wins on equal timestamps
                        self.elements
                            .insert(key.clone(), (*other_ts, *other_is_add));
                    }
                }
                None => {
                    self.elements
                        .insert(key.clone(), (*other_ts, *other_is_add));
                }
            }
        }
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> Delta for RWSet<K>
where
    K: Eq + Ord + Clone,
{
    type De = RWSet<K>;

    fn generate_delta(&self) -> Self::De {
        todo!()
    }
    fn merge_delta(&mut self, delta: &Self::De) {
        todo!()
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> CvRDTValidation<RWSet<K>> for RWSet<K>
where
    K: Eq + Ord + Clone + Debug,
{
    fn cvrdt_associativity(a: RWSet<K>, b: RWSet<K>, c: RWSet<K>) -> bool {
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

    fn cvrdt_commutativity(a: RWSet<K>, b: RWSet<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: RWSet<K>) -> bool {
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
        let mut a = RWSet::<String>::new();
        let mut b = RWSet::<String>::new();
        let mut c = RWSet::<String>::new();

        let timestamp = get_current_timestamp();

        a.insert("a".to_string(), timestamp);
        b.insert("b".to_string(), timestamp + 1);
        c.insert("c".to_string(), timestamp + 2);
        a.remove("a".to_string(), timestamp + 3);

        assert!(RWSet::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(RWSet::<String>::cvrdt_commutativity(a.clone(), b.clone()));
        assert!(RWSet::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {}

    #[test]
    fn test_delta_validation() {}
}
