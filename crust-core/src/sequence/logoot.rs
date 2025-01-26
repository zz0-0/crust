// use crate::{
//     crdt_type::{CmRDT, CvRDT, Delta},
//     crdt_validation::CvRDTValidation,
//     text_operation::TextOperation,
// };
// use serde::{Deserialize, Serialize};
// use std::fmt::Debug;
// use std::hash::Hash;

// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
// pub struct Logoot<K>
// where
//     K: Eq + Hash,
// {
//     elements: Vec<(K, usize)>,
// }

// pub enum Operation<K> {
//     Insert { index: usize, element: K },
//     Delete { index: usize },
// }

// impl<K> Logoot<K>
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

//     pub fn generate_position(prev: usize, next: usize) -> usize {
//         prev + (next - prev) / 2
//     }

//     pub fn insert(&mut self, element: K, index: usize) {
//         let position = if self.elements.is_empty() {
//             1000
//         } else if index == 0 {
//             self.elements[0].1 / 2
//         } else if index >= self.elements.len() {
//             self.elements.last().unwrap().1 + 1000
//         } else {
//             Self::generate_position(self.elements[index - 1].1, self.elements[index].1)
//         };
//         self.elements.insert(index, (element, position));
//     }

//     pub fn delete(&mut self, index: usize) -> Option<K> {
//         if index < self.elements.len() {
//             Some(self.elements.remove(index).0)
//         } else {
//             None
//         }
//     }

//     // pub fn position() -> usize {}
// }

// impl<K> CmRDT for Logoot<K>
// where
//     K: Eq + Hash + Clone,
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

// impl<K> CvRDT for Logoot<K>
// where
//     K: Eq + Hash + Clone + PartialOrd,
// {
//     fn merge(&mut self, other: &Self) {
//         let mut merged = self.elements.clone();

//         for (element, position) in &other.elements {
//             if !self.elements.iter().any(|(e, _)| e == element) {
//                 merged.push((element.clone(), *position));
//             }
//         }

//         merged.sort_by_key(|(_, position)| *position);
//         self.elements = merged;
//     }
// }

// impl<K> Delta for Logoot<K>
// where
//     K: Eq + Hash + Clone + PartialOrd,
// {
//     type Value = K;

//     fn generate_delta(&self) -> Self::Delta {
//         let delta_elements: Vec<(K, usize)> = self
//             .elements
//             .iter()
//             .filter(|elem| !since.elements.contains(elem))
//             .cloned()
//             .collect();
//         Logoot {
//             elements: delta_elements,
//         }
//     }

//     fn apply_delta(&mut self, delta: &Self::Delta) {
//         self.merge(&delta);
//     }

//     fn convert_delta(&self, op: TextOperation<K>) {
//         todo!()
//     }
// }

// impl<K> CvRDTValidation<Logoot<K>> for Logoot<K>
// where
//     K: Eq + Hash + Clone + PartialOrd + Debug,
// {
//     fn associativity(a: Logoot<K>, b: Logoot<K>, c: Logoot<K>) -> bool {
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

//     fn commutativity(a: Logoot<K>, b: Logoot<K>) -> bool {
//         let mut a_b = a.clone();
//         a_b.merge(&b);
//         let mut b_a = b.clone();
//         b_a.merge(&a);
//         println!("{:?} {:?}", a_b, b_a);
//         a_b == b_a
//     }

//     fn idempotence(a: Logoot<K>) -> bool {
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
//         let mut a = Logoot::<String>::new();
//         let mut b = Logoot::<String>::new();
//         let mut c = Logoot::<String>::new();

//         a.insert("a".to_string(), 0);
//         b.insert("b".to_string(), 0);
//         c.insert("c".to_string(), 0);

//         assert!(Logoot::<String>::associativity(
//             a.clone(),
//             b.clone(),
//             c.clone()
//         ));
//         assert!(Logoot::<String>::commutativity(a.clone(), b.clone()));
//         assert!(Logoot::<String>::idempotence(a.clone()));
//     }
// }
