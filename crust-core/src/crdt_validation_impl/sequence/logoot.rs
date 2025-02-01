#[cfg(test)]
mod tests {
    use crate::crdt_sync_type::{CmRDT, Delta};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use std::fmt::Debug;
    

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
}

#[cfg(test)]
mod correctness_tests {
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use crate::get_current_timestamp;

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
}
