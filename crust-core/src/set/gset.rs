use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GSet<K>
where
    K: Eq + Ord + Clone,
{
    elements: BTreeMap<K, u128>,
    previous_elements: BTreeMap<K, u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    Add(K, u128),
}

impl<K> GSet<K>
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
        match self.elements.get(&value) {
            Some(&ts) if ts >= timestamp => return,
            _ => {
                self.elements.insert(value, timestamp);
            }
        }
    }

    pub fn name(&self) -> String {
        "GSet".to_string()
    }
}

impl<K> CmRDT for GSet<K>
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

impl<K> CvRDT for GSet<K>
where
    K: Eq + Ord + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (key, timestamp) in other.elements.iter() {
            match self.elements.get(key) {
                Some(&ts) if ts >= *timestamp => continue,
                _ => {
                    self.elements.insert(key.clone(), *timestamp);
                }
            }
        }
    }
}

impl<K> Delta for GSet<K>
where
    K: Eq + Ord + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = GSet<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = GSet::new();
        for (key, timestamp) in self.elements.iter() {
            match self.previous_elements.get(key) {
                Some(&ts) if ts >= *timestamp => continue,
                _ => {
                    delta.elements.insert(key.clone(), *timestamp);
                }
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        for (key, timestamp) in delta.elements.iter() {
            match self.elements.get(key) {
                Some(&ts) if ts >= *timestamp => continue,
                _ => {
                    self.elements.insert(key.clone(), *timestamp);
                }
            }
        }
    }
}

impl<K> CvRDTValidation<GSet<K>> for GSet<K>
where
    K: Eq + Ord + Clone + Debug,
{
    fn cvrdt_associativity(a: GSet<K>, b: GSet<K>, c: GSet<K>) -> bool {
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

    fn cvrdt_commutativity(a: GSet<K>, b: GSet<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: GSet<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<GSet<K>> for GSet<K>
where
    K: Eq + Ord + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
{
    fn cmrdt_commutativity(
        a: GSet<K>,
        op1: <GSet<K> as CmRDT>::Op,
        op2: <GSet<K> as CmRDT>::Op,
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

    fn cmrdt_idempotence(a: GSet<K>, op1: <GSet<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(a: GSet<K>, ops: Vec<<GSet<K> as CmRDT>::Op>) -> bool {
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

impl<K> DeltaValidation<GSet<K>> for GSet<K>
where
    K: Eq + Ord + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
{
    fn delta_associativity(
        a: GSet<K>,
        de1: <GSet<K> as Delta>::De,
        de2: <GSet<K> as Delta>::De,
        de3: <GSet<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.apply_delta(&de1.clone());
        a1.apply_delta(&de2.clone());
        a1.apply_delta(&de3.clone());

        let mut a2 = a.clone();
        let mut combined_delta = GSet {
            elements: de2.elements.clone(),
            previous_elements: de2.previous_elements.clone(),
        };

        for (k, v) in de3.elements {
            match combined_delta.elements.get(&k) {
                Some(&existing_ts) if existing_ts >= v => continue,
                _ => {
                    combined_delta.elements.insert(k, v);
                }
            }
        }

        a2.apply_delta(&de1);
        a2.apply_delta(&combined_delta);

        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_commutativity(
        a: GSet<K>,
        de1: <GSet<K> as Delta>::De,
        de2: <GSet<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.apply_delta(&de1.clone());
        a1.apply_delta(&de2.clone());
        let mut a2 = a.clone();
        a2.apply_delta(&de2);
        a2.apply_delta(&de1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_idempotence(a: GSet<K>, de1: <GSet<K> as Delta>::De) -> bool {
        let mut a1 = a.clone();
        a1.apply_delta(&de1.clone());
        a1.apply_delta(&de1.clone());
        let mut a2 = a.clone();
        a2.apply_delta(&de1.clone());
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
        let mut a = GSet::<String>::new();
        let mut b = GSet::<String>::new();
        let mut c = GSet::<String>::new();

        let timpstamp = get_current_timestamp();

        a.insert("a".to_string(), timpstamp);
        b.insert("b".to_string(), timpstamp + 1);
        c.insert("c".to_string(), timpstamp + 2);

        assert!(GSet::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(GSet::<String>::cvrdt_commutativity(a.clone(), b.clone()));
        assert!(GSet::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = GSet::<String>::new();
        let mut g2 = GSet::<String>::new();
        let mut g3 = GSet::<String>::new();

        g1.insert("a".to_string(), timpstamp);
        g1.insert("b".to_string(), timpstamp + 1);
        g2.insert("b".to_string(), timpstamp + 2);
        g2.insert("c".to_string(), timpstamp + 3);
        g3.insert("c".to_string(), timpstamp + 4);
        g3.insert("d".to_string(), timpstamp + 5);

        // let op1 = Operation::Add {
        //     vertex: "x".to_string(),
        //     timestamp: timpstamp + 6,
        // };
        // let op2 = Operation::Add {
        //     vertex: "y".to_string(),
        //     timestamp: timpstamp + 6,
        // };
        // let op3 = Operation::Add {
        //     vertex: "s".to_string(),
        //     timestamp: timpstamp + 8,
        // };

        let op1 = Operation::Add("x".to_string(), timpstamp + 6);
        let op2 = Operation::Add("y".to_string(), timpstamp + 8);
        let op3 = Operation::Add("s".to_string(), timpstamp + 10);

        assert!(GSet::<String>::cmrdt_commutativity(
            g1.clone(),
            op1.clone(),
            op2.clone()
        ));

        // Test idempotence of operations
        assert!(GSet::<String>::cmrdt_idempotence(g1.clone(), op1.clone()));

        // Test sequential consistency
        let ops = vec![op1, op2, op3];
        assert!(GSet::<String>::cmrdt_sequential_consistency(
            g1.clone(),
            ops
        ));
    }

    #[test]
    fn test_delta_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = GSet::<String>::new();
        let mut g2 = GSet::<String>::new();
        let mut g3 = GSet::<String>::new();

        g1.insert("a".to_string(), timpstamp);
        g1.insert("b".to_string(), timpstamp + 1);
        g2.insert("b".to_string(), timpstamp + 2);
        g2.insert("c".to_string(), timpstamp + 3);
        g3.insert("c".to_string(), timpstamp + 4);
        g3.insert("d".to_string(), timpstamp + 5);

        let mut delta_graph = g1.clone();
        delta_graph.insert("x".to_string(), timpstamp + 6);

        let delta1 = delta_graph.generate_delta();
        delta_graph.insert("y".to_string(), timpstamp + 8);
        let delta2 = delta_graph.generate_delta();

        let delta3 = delta_graph.generate_delta();

        assert!(GSet::<String>::delta_associativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone(),
            delta3.clone()
        ));

        assert!(GSet::<String>::delta_commutativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone()
        ));

        assert!(GSet::<String>::delta_idempotence(
            g1.clone(),
            delta1.clone()
        ));
    }
}
