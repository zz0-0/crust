// 

// use crate::{
//     crdt_type::{CmRDT, CvRDT, Delta},
//     crdt_validation::CvRDTValidation,
//     text_operation::TextOperation,
// };
// use serde::{Deserialize, Serialize};
// use std::fmt::Debug;
// use std::hash::Hash;

// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
// pub struct MerkleDAGTree<K>
// where
//     K: Eq + Hash,
// {
//     vertices: HashMap<K, Vec<u8>>,
//     edges: HashMap<K, Vec<K>>,
// }

// pub enum Operation<K> {
//     AddVertex { hash: K, data: Vec<u8> },
//     AddEdge { from: K, to: K },
// }

// impl<K> MerkleDAGTree<K>
// where
//     K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
// {
//     pub fn new() -> Self {
//         Self {
//             vertices: HashMap::new(),
//             edges: HashMap::new(),
//         }
//     }

//     pub fn to_string(&self) -> Result<String, serde_json::Error> {
//         serde_json::to_string(&self)
//     }

//     pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
//         serde_json::from_str(&str)
//     }

//     pub fn add_edge() {}

//     pub fn add_vertex() {}

//     pub fn verify() {}
// }

// impl<K> CmRDT for MerkleDAGTree<K>
// where
//     K: Eq + Hash,
// {
//     type Op = Operation<K>;
//     type Value = K;

//     fn apply(&mut self, op: Self::Op) {
//         match op {
//             Operation::AddVertex { hash, data } => {}
//             Operation::AddEdge { from, to } => {}
//         }
//     }

//     fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
//         todo!()
//     }
// }

// impl<K> CvRDT for MerkleDAGTree<K>
// where
//     K: Eq + Hash,
// {
//     fn merge(&mut self, other: &Self) {
//         todo!()
//     }
// }

// impl<K> Delta for MerkleDAGTree<K>
// where
//     K: Eq + Hash,
// {
//     type Value = K;

//     fn generate_delta(&self) -> Self::Delta {
//         todo!()
//     }

//     fn apply_delta(&mut self, delta: &Self::Delta) {
//         todo!()
//     }

//     fn convert_delta(&self, op: TextOperation<K>) {
//         todo!()
//     }
// }

// impl<K> CvRDTValidation<MerkleDAGTree<K>> for MerkleDAGTree<K>
// where
//     K: Eq + Hash + Clone + Debug,
// {
//     fn associativity(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>, c: MerkleDAGTree<K>) -> bool {
//         let mut ab_c = a.clone();
//         ab_c.merge(&b);
//         let mut bc = b.clone();
//         bc.merge(&c);
//         ab_c.merge(&c);
//         let mut a_bc = a.clone();
//         a_bc.merge(&bc);
//         println!("{:?} {:?}", ab_c, a_bc);
//         ab_c == a_bc
//     }

//     fn commutativity(a: MerkleDAGTree<K>, b: MerkleDAGTree<K>) -> bool {
//         let mut a_b = a.clone();
//         a_b.merge(&b);
//         let mut b_a = b.clone();
//         b_a.merge(&a);
//         println!("{:?} {:?}", a_b, b_a);
//         a_b == b_a
//     }

//     fn idempotence(a: MerkleDAGTree<K>) -> bool {
//         let mut a_a = a.clone();
//         a_a.merge(&a);
//         println!("{:?} {:?}", a_a, a);
//         a_a == a
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_cvrdt_validation() {
//         let mut a = MerkleDAGTree::<String>::new();
//         let mut b = MerkleDAGTree::<String>::new();
//         let mut c = MerkleDAGTree::<String>::new();

//         // a.increment("a".to_string());
//         // b.increment("b".to_string());
//         // c.increment("c".to_string());

//         assert!(MerkleDAGTree::<String>::associativity(
//             a.clone(),
//             b.clone(),
//             c.clone()
//         ));
//         assert!(MerkleDAGTree::<String>::commutativity(a.clone(), b.clone()));
//         assert!(MerkleDAGTree::<String>::idempotence(a.clone()));
//     }
// }
