use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    get_current_timestamp,
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PNCounter<K>
where
    K: Eq + Hash,
{
    p: HashMap<K, u64>,
    n: HashMap<K, u64>,
    applied_ops: HashSet<u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Increment { key: K, timestamp: u128 },
    Decrement { key: K, timestamp: u128 },
}

impl<K> PNCounter<K>
where
    K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            p: HashMap::new(),
            n: HashMap::new(),
            applied_ops: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<(HashMap<K, u64>, HashMap<K, u64>), serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn value(&self) -> i64 {
        let p: u64 = self.p.values().sum();
        let n: u64 = self.n.values().sum();
        p as i64 - n as i64
    }

    pub fn increment(&mut self, key: K) {
        *self.p.entry(key).or_insert(0) += 1;
    }

    pub fn decrement(&mut self, key: K) {
        *self.n.entry(key).or_insert(0) += 1;
    }
}

impl<K> CmRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Self::Op::Increment { ref key, timestamp } => {
                if !self.applied_ops.contains(&timestamp) {
                    *self.p.entry(key.clone()).or_insert(0) += 1;
                    self.applied_ops.insert(timestamp);
                }
            }
            Self::Op::Decrement { ref key, timestamp } => {
                if !self.applied_ops.contains(&timestamp) {
                    *self.n.entry(key.clone()).or_insert(0) += 1;
                    self.applied_ops.insert(timestamp);
                }
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, value } => {
                vec![Self::Op::Increment {
                    key: value,
                    timestamp: get_current_timestamp(),
                }]
            }
            TextOperation::Delete { position: _, value } => {
                vec![Self::Op::Decrement {
                    key: value,
                    timestamp: get_current_timestamp(),
                }]
            }
        }
    }
}

impl<K> CvRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.p {
            let current_p = self.p.entry(k.clone()).or_insert(0);
            *current_p = (*current_p).max(*v);
        }
        for (k, v) in &other.n {
            let current_n = self.n.entry(k.clone()).or_insert(0);
            *current_n = (*current_n).max(*v);
        }
        self.applied_ops.extend(other.applied_ops.iter().copied());
    }
}

impl<K> Delta for PNCounter<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = (HashMap<K, u64>, HashMap<K, u64>);

    fn generate_delta(&self, since: &Self) -> Self::De {
        let mut p_delta = HashMap::new();
        let mut n_delta = HashMap::new();

        for (k, v) in &self.p {
            let since_v = since.p.get(k).unwrap_or(&0);
            if v > since_v {
                p_delta.insert(k.clone(), *v);
            }
        }

        for (k, v) in &self.n {
            let since_v = since.n.get(k).unwrap_or(&0);
            if v > since_v {
                n_delta.insert(k.clone(), *v);
            }
        }

        (p_delta, n_delta)
    }

    fn merge_delta(&mut self, delta: Self::De) {
        for (k, v) in delta.0 {
            let current_p = self.p.entry(k).or_insert(0);
            *current_p = (*current_p).max(v);
        }
        for (k, v) in delta.1 {
            let current_n = self.n.entry(k).or_insert(0);
            *current_n = (*current_n).max(v);
        }
    }
}

impl<K> CvRDTValidation<PNCounter<K>> for PNCounter<K>
where
    K: Eq + Hash + Clone + Debug,
{
    fn cvrdt_associativity(a: PNCounter<K>, b: PNCounter<K>, c: PNCounter<K>) -> bool {
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

    fn cvrdt_commutativity(a: PNCounter<K>, b: PNCounter<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: PNCounter<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<PNCounter<K>> for PNCounter<K>
where
    K: Eq + Hash + Clone,
    PNCounter<K>: CmRDT + Debug,
{
    fn cmrdt_commutativity(
        a: PNCounter<K>,
        op1: <PNCounter<K> as CmRDT>::Op,
        op2: <PNCounter<K> as CmRDT>::Op,
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

    fn cmrdt_idempotence(a: PNCounter<K>, op1: <PNCounter<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(
        a: PNCounter<K>,
        ops: Vec<<PNCounter<K> as CmRDT>::Op>,
    ) -> bool {
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

impl<K> DeltaValidation<PNCounter<K>> for PNCounter<K>
where
    K: Eq + Hash + Clone,
    PNCounter<K>: Delta<De = (HashMap<K, u64>, HashMap<K, u64>)> + Debug,
{
    fn delta_associativity(
        a: PNCounter<K>,
        de1: <PNCounter<K> as Delta>::De,
        de2: <PNCounter<K> as Delta>::De,
        de3: <PNCounter<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(de1.clone());
        a1.merge_delta(de2.clone());
        a1.merge_delta(de3.clone());

        let mut a2 = a.clone();
        let mut combined_delta = (HashMap::new(), HashMap::new());
        for (k, v) in de2.0.into_iter() {
            *combined_delta.0.entry(k).or_insert(0) += v;
        }
        for (k, v) in de2.1.into_iter() {
            *combined_delta.1.entry(k).or_insert(0) += v;
        }
        for (k, v) in de3.0.into_iter() {
            *combined_delta.0.entry(k).or_insert(0) += v;
        }
        for (k, v) in de3.1.into_iter() {
            *combined_delta.1.entry(k).or_insert(0) += v;
        }
        a2.merge_delta(de1);
        a2.merge_delta(combined_delta);

        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_commutativity(
        a: PNCounter<K>,
        de1: <PNCounter<K> as Delta>::De,
        de2: <PNCounter<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(de1.clone());
        a1.merge_delta(de2.clone());
        let mut a2 = a.clone();
        a2.merge_delta(de2);
        a2.merge_delta(de1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_idempotence(a: PNCounter<K>, de1: <PNCounter<K> as Delta>::De) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(de1.clone());
        a1.merge_delta(de1.clone());
        let mut a2 = a.clone();
        a2.merge_delta(de1.clone());
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = PNCounter::<String>::new();
        let mut b = PNCounter::<String>::new();
        let mut c = PNCounter::<String>::new();

        a.increment("a".to_string());
        b.increment("b".to_string());
        c.increment("c".to_string());

        assert!(PNCounter::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(PNCounter::<String>::cvrdt_commutativity(
            a.clone(),
            b.clone()
        ));
        assert!(PNCounter::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let a = PNCounter::<String>::new();
        let op1 = Operation::Increment {
            key: "a".to_string(),
            timestamp: get_current_timestamp(),
        };
        let op2 = Operation::Increment {
            key: "b".to_string(),
            timestamp: get_current_timestamp() + 1,
        };
        assert!(PNCounter::<String>::cmrdt_commutativity(
            a.clone(),
            op1.clone(),
            op2.clone()
        ));
        assert!(PNCounter::<String>::cmrdt_idempotence(
            a.clone(),
            op1.clone()
        ));
        assert!(PNCounter::<String>::cmrdt_sequential_consistency(
            a.clone(),
            vec![op1.clone(), op2.clone()]
        ));
    }

    #[test]
    fn test_delta_validation() {
        let mut a = PNCounter::<String>::new();
        let mut b = PNCounter::<String>::new();
        let mut c = PNCounter::<String>::new();

        a.increment("x".to_string());
        a.increment("x".to_string());
        b.increment("x".to_string());
        b.increment("y".to_string());
        c.increment("z".to_string());

        let d1 = a.generate_delta(&b);
        let d2 = b.generate_delta(&c);
        let d3 = c.generate_delta(&a);

        assert!(PNCounter::<String>::delta_associativity(
            a.clone(),
            d1.clone(),
            d2.clone(),
            d3.clone()
        ));
        assert!(PNCounter::<String>::delta_commutativity(
            a.clone(),
            d1.clone(),
            d2.clone()
        ));
        assert!(PNCounter::<String>::delta_idempotence(
            a.clone(),
            d1.clone()
        ));
    }
}
