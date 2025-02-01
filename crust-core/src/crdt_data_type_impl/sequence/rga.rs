// use crate::{
//     crdt_type::{CmRDT, CvRDT, Delta},
//     crdt_validation::CvRDTValidation,
//     text_operation::TextOperation,
// };
// use serde::{Deserialize, Serialize};
// use std::fmt::Debug;
// use std::hash::Hash;

// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
// pub struct RGA<K>
// where
//     K: Eq + Hash,
// {
//     elements: Vec<(K, usize)>,
// }

// pub enum Operation<K> {
//     Insert { index: usize, element: K },
//     Delete { index: usize },
// }

// impl<K> RGA<K>
// where
//     K: Eq + Hash + Serialize + for<'a> Deserialize<'a>,
// {
//     pub fn new() -> Self {
//         Self {
//             elements: Vec::new(),
//         }
//     }

//     pub fn to_string(&self) -> Result<String, serde_json::Error> {
//         serde_json::to_string(&self)
//     }

//     pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
//         serde_json::from_str(&str)
//     }

//     pub fn insert() {}

//     pub fn delete() {}
// }

// impl<K> CmRDT for RGA<K>
// where
//     K: Eq + Hash,
// {
//     type Op = Operation<K>;
//     type Value = K;

//     fn apply(&mut self, op: Self::Op) {
//         match op {
//             Operation::Insert { index, element } => {}
//             Operation::Delete { index } => {}
//         }
//     }

//     fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
//         todo!()
//     }
// }

// impl<K> CvRDT for RGA<K>
// where
//     K: Eq + Hash + Clone + PartialOrd,
// {
//     fn merge(&mut self, other: &Self) {
//         let mut merged = Vec::new();
//         let mut i = 0;
//         let mut j = 0;

//         while i < self.elements.len() && j < other.elements.len() {
//             let (self_id, _) = self.elements[i].clone();
//             let (other_id, _) = other.elements[j].clone();

//             if self_id < other_id {
//                 merged.push(self.elements[i].clone());
//                 i += 1;
//             } else if self_id > other_id {
//                 merged.push(other.elements[j].clone());
//                 j += 1;
//             } else {
//                 // Same id; elements should be the same if correctly implemented
//                 merged.push(self.elements[i].clone());
//                 i += 1;
//                 j += 1;
//             }
//         }

//         // Append remaining elements
//         merged.extend_from_slice(&self.elements[i..]);
//         merged.extend_from_slice(&other.elements[j..]);

//         self.elements = merged;
//     }
// }

// impl<K> Delta for RGA<K>
// where
//     K: Eq + Hash + Clone + PartialOrd,
// {
//     type Value = K;

//     fn generate_delta(&self) -> Self::Delta {
//         // let delta = RGA {
//         //     elements: self
//         //         .elements
//         //         .iter()
//         //         .filter(|&&(id, _)| !since.elements.contains(&(id, _)))
//         //         .cloned()
//         //         .collect(),
//         // };
//         // delta
//         todo!()
//     }

//     fn apply_delta(&mut self, delta: &Self::Delta) {
//         self.merge(&delta);
//     }

//     fn convert_delta(&self, op: TextOperation<K>) {
//         todo!()
//     }
// }
