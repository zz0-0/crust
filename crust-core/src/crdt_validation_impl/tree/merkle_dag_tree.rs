#[cfg(test)]
mod tests {

    use crate::crdt_sync_type::{CmRDT, Delta};
use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};

    use std::fmt::Debug;
    

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
}

#[cfg(test)]
mod correctness_tests {

    use crate::get_current_timestamp;

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
}
