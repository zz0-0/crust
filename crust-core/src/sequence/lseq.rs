// use crate::{
//     crdt_type::{CmRDT, CvRDT, Delta},
//     crdt_validation::CvRDTValidation,
//     text_operation::TextOperation,
// };
// use serde::{Deserialize, Serialize};
// use std::fmt::Debug;
// use std::hash::Hash;

// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
// pub struct LSeq<K>
// where
//     K: Eq + Hash,
// {
//     elements: Vec<(K, u64)>,
// }

// pub enum Operation<K> {
//     Insert { index: usize, element: K },
//     Delete { index: usize },
// }

// impl<K> LSeq<K>
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

//     pub fn insert(&mut self, id: K, element: u64) {
//         // let pos = self
//         //     .elements
//         //     .binary_search_by_key(&id, |&(elem_id, _)| elem_id)
//         //     .unwrap_or_else(|x| x);
//         // self.elements.insert(pos, (id, element));
//     }

//     pub fn delete() {}
// }

// impl<K> CmRDT for LSeq<K>
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

// impl<K> CvRDT for LSeq<K>
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
//                 // Same id; choose one, perhaps based on some criteria like timestamp
//                 // For simplicity, choose one arbitrarily
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

// impl<K> Delta for LSeq<K>
// where
//     K: Eq + Hash + Clone + PartialOrd + Serialize + for<'a> Deserialize<'a>,
// {
//     type Value = K;

//     fn generate_delta(&self) -> Self::Delta {
//         let mut delta = LSeq::new();
//         let mut since_iter = since.elements.iter();
//         let mut since_ptr = since_iter.next();

//         // for &(self_id, ref self_elem) in &self.elements.clone() {
//         //     loop {
//         //         match since_ptr {
//         //             Some(&(since_id, _)) => {
//         //                 if since_id < self_id {
//         //                     since_ptr = since_iter.next();
//         //                 } else if since_id == self_id {
//         //                     since_ptr = since_iter.next();
//         //                     break;
//         //                 } else {
//         //                     delta.insert(self_id, self_elem.clone());
//         //                     break;
//         //                 }
//         //             }
//         //             None => {
//         //                 delta.insert(self_id, self_elem.clone());
//         //                 break;
//         //             }
//         //         }
//         //     }
//         // }
//         delta
//     }

//     fn apply_delta(&mut self, delta: &Self::Delta) {
//         self.merge(&delta);
//     }

//     fn convert_delta(&self, op: TextOperation<K>) {
//         todo!()
//     }
// }

// impl<K> CvRDTValidation<LSeq<K>> for LSeq<K>
// where
//     K: Eq + Hash + Clone + PartialOrd + Debug,
// {
//     fn associativity(a: LSeq<K>, b: LSeq<K>, c: LSeq<K>) -> bool {
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

//     fn commutativity(a: LSeq<K>, b: LSeq<K>) -> bool {
//         let mut a_b = a.clone();
//         a_b.merge(&b);
//         let mut b_a = b.clone();
//         b_a.merge(&a);
//         println!("{:?} {:?}", a_b, b_a);
//         a_b == b_a
//     }

//     fn idempotence(a: LSeq<K>) -> bool {
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
//         let mut a = LSeq::<String>::new();
//         let mut b = LSeq::<String>::new();
//         let mut c = LSeq::<String>::new();

//         // a.increment("a".to_string());
//         // b.increment("b".to_string());
//         // c.increment("c".to_string());

//         assert!(LSeq::<String>::associativity(
//             a.clone(),
//             b.clone(),
//             c.clone()
//         ));
//         assert!(LSeq::<String>::commutativity(a.clone(), b.clone()));
//         assert!(LSeq::<String>::idempotence(a.clone()));
//     }
// }
