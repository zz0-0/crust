pub mod counter;
pub mod graph;
pub mod register;
pub mod sequence;
pub mod set;
pub mod tree;

// #[cfg(test)]
// pub mod cvrdt_validation {
//     use crate::crdt_sync_type::CvRDT;

//     pub trait CvRDTValidation<K>
//     where
//         K: CvRDT,
//     {
//         fn cvrdt_associativity(a: K, b: K, c: K) -> bool;
//         fn cvrdt_commutativity(a: K, b: K) -> bool;
//         fn cvrdt_idempotence(a: K) -> bool;
//     }
// }

// #[cfg(test)]
// pub mod cmrdt_validation {
//     use crate::crdt_sync_type::CmRDT;

//     pub trait CmRDTValidation<K>
//     where
//         K: CmRDT,
//     {
//         fn cmrdt_commutativity(a: K, op1: K::Op, op2: K::Op) -> bool;
//         fn cmrdt_idempotence(a: K, op1: K::Op) -> bool;
//         fn cmrdt_sequential_consistency(a: K, ops: Vec<K::Op>) -> bool;
//     }
// }

// #[cfg(test)]
// pub mod delta_validation {
//     use crate::crdt_sync_type::Delta;

//     pub trait DeltaValidation<K>
//     where
//         K: Delta,
//     {
//         fn delta_associativity(a: K, de1: K::De, de2: K::De, de3: K::De) -> bool;
//         fn delta_commutativity(a: K, de1: K::De, de2: K::De) -> bool;
//         fn delta_idempotence(a: K, de1: K::De) -> bool;
//     }
// }
