#[cfg(test)]
mod tests {
    use crate::crdt_sync_type::{CmRDT, Delta};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use std::fmt::Debug;
    

    // impl<K> CvRDTValidation<RGA<K>> for RGA<K>
    // where
    //     K: Eq + Hash + Clone + PartialOrd + Debug,
    // {
    //     fn associativity(a: RGA<K>, b: RGA<K>, c: RGA<K>) -> bool {
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

    //     fn commutativity(a: RGA<K>, b: RGA<K>) -> bool {
    //         let mut a_b = a.clone();
    //         a_b.merge(&b);
    //         let mut b_a = b.clone();
    //         b_a.merge(&a);
    //         println!("{:?} {:?}", a_b, b_a);
    //         a_b == b_a
    //     }

    //     fn idempotence(a: RGA<K>) -> bool {
    //         let mut a_a = a.clone();
    //         a_a.merge(&a);
    //         println!("{:?} {:?}", a_a, a);
    //         a_a == a
    //     }
    // }
}

#[cfg(test)]
mod correctness_tests {
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use crate::get_current_timestamp;

    //     #[test]
    //     fn test_cvrdt_validation() {
    //         let mut a = RGA::<String>::new();
    //         let mut b = RGA::<String>::new();
    //         let mut c = RGA::<String>::new();

    //         // a.increment("a".to_string());
    //         // b.increment("b".to_string());
    //         // c.increment("c".to_string());

    //         assert!(RGA::<String>::associativity(
    //             a.clone(),
    //             b.clone(),
    //             c.clone()
    //         ));
    //         assert!(RGA::<String>::commutativity(a.clone(), b.clone()));
    //         assert!(RGA::<String>::idempotence(a.clone()));
    //     }
}
