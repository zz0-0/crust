use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::{
        TextOperation, TextOperationToCmRDT, TextOperationToCvRDT, TextOperationToDelta,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GCounter<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    counter: HashMap<K, u64>,
}

pub enum Operation<K> {
    Increment { key: K },
}

impl<K> GCounter<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    pub fn new() -> Self {
        Self {
            counter: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Self::Op::Increment { key } => {
                let current_count = self.counter.entry(key.clone()).or_insert(0);
                *current_count += 1;
            }
        }
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.counter {
            let current_count = self.counter.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
    }
}

impl<K> Delta for GCounter<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!();
    }

    fn apply_delta(&mut self, delta: &Self) {
        self.merge(delta);
    }
}

impl<K> TextOperationToCmRDT<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize + From<String>,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert { position: _, text } => {
                let key = K::from(text);
                vec![Self::Op::Increment { key }]
            }
            _ => vec![],
        }
    }
}

impl<K> TextOperationToCvRDT<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

// impl<K> Semilattice<GCounter<K>> for GCounter<K>
// where
//     K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
//     Self: CmRDT<Op = Operation<K>>,
// {
//     type Op = Operation<K>;

//     fn cmrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
//     where
//         GCounter<K>: CmRDT,
//     {
//         let mut ab_c = a.clone();
//         let mut bc = b.clone();
//         if let Some((k, v)) = b.counter.iter().next() {
//             ab_c.apply(Self::Op::Increment { key: k.clone() });
//         }
//         if let Some((k, v)) = c.counter.iter().next() {
//             bc.apply(Self::Op::Increment { key: k.clone() });
//         }
//         if let Some((k, v)) = c.counter.iter().next() {
//             ab_c.apply(Self::Op::Increment { key: k.clone() });
//         }
//         let mut a_bc = a.clone();
//         for (k, v) in bc.counter.iter() {
//             a_bc.apply(Self::Op::Increment { key: k.clone() });
//         }
//         ab_c.value() == a_bc.value()
//     }

//     fn cmrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
//     where
//         GCounter<K>: CmRDT,
//     {
//         let mut ab = a.clone();
//         let mut ba = b.clone();
//         if let Some((k, v)) = b.counter.iter().next() {
//             ab.apply(Self::Op::Increment { key: k.clone() });
//         }
//         if let Some((k, v)) = a.counter.iter().next() {
//             ba.apply(Self::Op::Increment { key: k.clone() });
//         }
//         ab.value() == ba.value()
//     }

//     fn cmrdt_idempotent(a: GCounter<K>) -> bool
//     where
//         GCounter<K>: CmRDT,
//     {
//         let mut once = a.clone();
//         let mut twice = a.clone();
//         if let Some((k, v)) = a.counter.iter().next() {
//             once.apply(Operation::Increment { key: k.clone() });
//         }
//         if let Some((k, v)) = a.counter.iter().next() {
//             twice.apply(Operation::Increment { key: k.clone() });
//         }
//         if let Some((k, v)) = a.counter.iter().next() {
//             twice.apply(Operation::Increment { key: k.clone() });
//         }
//         once.value() == twice.value()
//     }

//     fn cvrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
//     where
//         GCounter<K>: CvRDT,
//     {
//         let mut ab_c = a.clone();
//         let mut bc = b.clone();
//         ab_c.merge(&b);
//         bc.merge(&c);
//         ab_c.merge(&c);
//         let mut a_bc = a.clone();
//         a_bc.merge(&bc);
//         ab_c.value() == a_bc.value()
//     }

//     fn cvrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
//     where
//         GCounter<K>: CvRDT,
//     {
//         let mut ab = a.clone();
//         let mut ba = b.clone();
//         ab.merge(&b);
//         ba.merge(&a);
//         ab.value() == ba.value()
//     }

//     fn cvrdt_idempotent(a: GCounter<K>) -> bool
//     where
//         GCounter<K>: CvRDT,
//     {
//         let mut once = a.clone();
//         let mut twice = a.clone();
//         once.merge(&a);
//         twice.merge(&a);
//         twice.merge(&a);
//         once.value() == twice.value()
//     }

//     fn delta_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
//     where
//         GCounter<K>: Delta,
//     {
//         let mut ab_c = a.clone();
//         let mut bc = b.clone();
//         ab_c.apply_delta(&b);
//         bc.apply_delta(&c);
//         ab_c.apply_delta(&c);
//         let mut a_bc = a.clone();
//         a_bc.apply_delta(&bc);
//         ab_c.value() == a_bc.value()
//     }

//     fn delta_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
//     where
//         GCounter<K>: Delta,
//     {
//         let mut ab = a.clone();
//         let mut ba = b.clone();
//         ab.apply_delta(&b);
//         ba.apply_delta(&a);
//         ab.value() == ba.value()
//     }

//     fn delta_idempotent(a: GCounter<K>) -> bool
//     where
//         GCounter<K>: Delta,
//     {
//         let mut once = a.clone();
//         let mut twice = a.clone();
//         once.apply_delta(&a);
//         twice.apply_delta(&a);
//         twice.apply_delta(&a);
//         once.value() == twice.value()
//     }
// }

// #[cfg(test)]
// mod tests {
// use super::*;

// #[test]
// fn test_semilattice() {
// let mut a = GCounter::new();
// let mut b = GCounter::new();
// let mut c = GCounter::new();
// a.increment("r1".to_string());
// b.increment("r2".to_string());
// c.increment("r3".to_string());
// assert!(GCounter::<String>::cmrdt_associative(
//     a.clone(),
//     b.clone(),
//     c.clone()
// ));
// assert!(GCounter::<String>::cmrdt_commutative(a.clone(), b.clone()));
// assert!(GCounter::<String>::cmrdt_idempotent(a.clone()));
// assert!(GCounter::<String>::cvrdt_associative(
//     a.clone(),
//     b.clone(),
//     c.clone()
// ));
// assert!(GCounter::<String>::cvrdt_commutative(a.clone(), b.clone()));
// assert!(GCounter::<String>::cvrdt_idempotent(a.clone()));
// assert!(GCounter::<String>::delta_associative(
//     a.clone(),
//     b.clone(),
//     c.clone()
// ));
// assert!(GCounter::<String>::delta_commutative(a.clone(), b.clone()));
// assert!(GCounter::<String>::delta_idempotent(a.clone()));
//     }
// }
