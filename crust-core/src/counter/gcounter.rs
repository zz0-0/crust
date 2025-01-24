// Thread Safety and Data Corruption
// The implementation lacks inherent thread safety.
// To address this, consider using:
// Arc<Mutex<GCounter<K>>> for safe concurrent access.
// dashmap for concurrent HashMap operations.

use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    get_current_timestamp,
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GCounter<K>
where
    K: Eq + Hash,
{
    counter: HashMap<K, u64>,
    previous: HashMap<K, u64>,
    applied_ops: HashSet<u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Increment { key: K, timestamp: u128 },
}

impl<K> GCounter<K>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            counter: HashMap::new(),
            previous: HashMap::new(),
            applied_ops: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<HashMap<K, u64>, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn value(&self) -> u64 {
        self.counter.values().sum()
    }

    pub fn increment(&mut self, key: K) {
        *self.counter.entry(key).or_insert(0) += 1;
    }
}

impl<K> CmRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::Increment { ref key, timestamp } => {
                if !self.applied_ops.contains(&timestamp) {
                    *self.counter.entry(key.clone()).or_insert(0) += 1;
                    self.applied_ops.insert(timestamp);
                }
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => {
                vec![Operation::Increment {
                    key: value,
                    timestamp: get_current_timestamp(),
                }]
            }
            TextOperation::Delete { .. } => vec![],
        }
    }

    fn name(&self) -> String {
        "GCounter".to_string()
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.counter {
            let current_count = self.counter.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
    }

    fn name(&self) -> String {
        "GCounter".to_string()
    }
}

impl<K> Delta for GCounter<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = HashMap<K, u64>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = HashMap::new();
        for (k, v) in &self.counter {
            if let Some(&since_v) = self.previous.get(k) {
                if *v > since_v {
                    delta.insert(k.clone(), *v);
                }
            } else {
                delta.insert(k.clone(), *v);
            }
        }
        delta
    }

    fn merge_delta(&mut self, delta: &Self::De) {
        for (k, v) in delta {
            let current = self.counter.entry(k.clone()).or_insert(0);
            *current = (*current).max(v.clone());
        }
    }

    fn name(&self) -> String {
        "GCounter".to_string()
    }
}

impl<K> CvRDTValidation<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone,
    GCounter<K>: Debug,
{
    fn cvrdt_associativity(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool {
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

    fn cvrdt_commutativity(a: GCounter<K>, b: GCounter<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: GCounter<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone,
    GCounter<K>: CmRDT + Debug,
{
    fn cmrdt_commutativity(
        a: GCounter<K>,
        op1: <GCounter<K> as CmRDT>::Op,
        op2: <GCounter<K> as CmRDT>::Op,
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

    fn cmrdt_idempotence(a: GCounter<K>, op1: <GCounter<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(a: GCounter<K>, ops: Vec<<GCounter<K> as CmRDT>::Op>) -> bool {
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

impl<K> DeltaValidation<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone,
    GCounter<K>: Delta<De = HashMap<K, u64>> + Debug,
{
    fn delta_associativity(
        a: GCounter<K>,
        de1: <GCounter<K> as Delta>::De,
        de2: <GCounter<K> as Delta>::De,
        de3: <GCounter<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de2.clone());
        a1.merge_delta(&de3.clone());

        let mut a2 = a.clone();
        let mut combined_delta = HashMap::new();
        for (k, v) in de2.into_iter() {
            *combined_delta.entry(k).or_insert(0) += v;
        }
        for (k, v) in de3.into_iter() {
            *combined_delta.entry(k).or_insert(0) += v;
        }
        a2.merge_delta(&de1);
        a2.merge_delta(&combined_delta);

        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_commutativity(
        a: GCounter<K>,
        de1: <GCounter<K> as Delta>::De,
        de2: <GCounter<K> as Delta>::De,
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

    fn delta_idempotence(a: GCounter<K>, de1: <GCounter<K> as Delta>::De) -> bool {
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
    use crate::get_current_timestamp;

    use super::*;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = GCounter::<String>::new();
        let mut b = GCounter::<String>::new();
        let mut c = GCounter::<String>::new();

        a.increment("a".to_string());
        b.increment("b".to_string());
        c.increment("c".to_string());

        assert!(GCounter::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(GCounter::<String>::cvrdt_commutativity(
            a.clone(),
            b.clone()
        ));
        assert!(GCounter::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let a = GCounter::<String>::new();
        let op1 = Operation::Increment {
            key: "a".to_string(),
            timestamp: get_current_timestamp(),
        };
        let op2 = Operation::Increment {
            key: "b".to_string(),
            timestamp: get_current_timestamp() + 1,
        };
        assert!(GCounter::<String>::cmrdt_commutativity(
            a.clone(),
            op1.clone(),
            op2.clone()
        ));
        assert!(GCounter::<String>::cmrdt_idempotence(
            a.clone(),
            op1.clone()
        ));
        assert!(GCounter::<String>::cmrdt_sequential_consistency(
            a.clone(),
            vec![op1.clone(), op2.clone()]
        ));
    }

    #[test]
    fn test_delta_validation() {
        let mut a = GCounter::<String>::new();
        let mut b = GCounter::<String>::new();
        let mut c = GCounter::<String>::new();

        a.increment("x".to_string());
        a.increment("x".to_string());
        b.increment("x".to_string());
        b.increment("y".to_string());
        c.increment("z".to_string());

        let d1 = a.generate_delta();
        let d2 = b.generate_delta();
        let d3 = c.generate_delta();

        assert!(GCounter::<String>::delta_associativity(
            a.clone(),
            d1.clone(),
            d2.clone(),
            d3.clone()
        ));
        assert!(GCounter::<String>::delta_commutativity(
            a.clone(),
            d1.clone(),
            d2.clone()
        ));
        assert!(GCounter::<String>::delta_idempotence(a.clone(), d1.clone()));
    }
}
