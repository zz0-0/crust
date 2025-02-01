#[cfg(test)]
mod tests {
    use crate::crdt_data_type_impl::counter::pncounter::PNCounter;
    use crate::crdt_sync_type::{CmRDT, CvRDT, Delta};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use rand::seq::SliceRandom;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    impl<K> CvRDTValidation<PNCounter<K>> for PNCounter<K>
    where
        K: Eq + Hash + Clone + Debug,
    {
        fn cvrdt_associativity(a: PNCounter<K>, b: PNCounter<K>, c: PNCounter<K>) -> bool {
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

        fn cvrdt_commutativity(a: PNCounter<K>, b: PNCounter<K>) -> bool {
            let mut a_b = a.clone();
            a_b.merge(&b);
            let mut b_a = b.clone();
            b_a.merge(&a);
            println!("{:?} {:?}", a_b, b_a);
            a_b == b_a
        }

        fn cvrdt_idempotence(a: PNCounter<K>) -> bool {
            let mut a_a = a.clone();
            a_a.merge(&a);
            println!("{:?} {:?}", a_a, a);
            a_a == a
        }
    }

    impl<K> CmRDTValidation<PNCounter<K>> for PNCounter<K>
    where
        K: Eq + Hash + Clone,
        PNCounter<K>: CmRDT + Debug,
    {
        fn cmrdt_commutativity(
            a: PNCounter<K>,
            op1: <PNCounter<K> as CmRDT>::Op,
            op2: <PNCounter<K> as CmRDT>::Op,
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

        fn cmrdt_idempotence(a: PNCounter<K>, op1: <PNCounter<K> as CmRDT>::Op) -> bool {
            let mut a1 = a.clone();
            a1.apply(&op1);
            a1.apply(&op1);
            let mut a2 = a.clone();
            a2.apply(&op1);
            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn cmrdt_sequential_consistency(
            a: PNCounter<K>,
            ops: Vec<<PNCounter<K> as CmRDT>::Op>,
        ) -> bool {
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

    impl<K> DeltaValidation<PNCounter<K>> for PNCounter<K>
    where
        K: Eq + Hash + Clone,
        PNCounter<K>: Delta<De = (HashMap<K, u64>, HashMap<K, u64>)> + Debug,
    {
        fn delta_associativity(
            a: PNCounter<K>,
            de1: <PNCounter<K> as Delta>::De,
            de2: <PNCounter<K> as Delta>::De,
            de3: <PNCounter<K> as Delta>::De,
        ) -> bool {
            let mut a1 = a.clone();
            a1.apply_delta(&de1.clone());
            a1.apply_delta(&de2.clone());
            a1.apply_delta(&de3.clone());

            let mut a2 = a.clone();
            let mut combined_delta = (HashMap::new(), HashMap::new());
            for (k, v) in de2.0.into_iter() {
                *combined_delta.0.entry(k).or_insert(0) += v;
            }
            for (k, v) in de2.1.into_iter() {
                *combined_delta.1.entry(k).or_insert(0) += v;
            }
            for (k, v) in de3.0.into_iter() {
                *combined_delta.0.entry(k).or_insert(0) += v;
            }
            for (k, v) in de3.1.into_iter() {
                *combined_delta.1.entry(k).or_insert(0) += v;
            }
            a2.apply_delta(&de1);
            a2.apply_delta(&combined_delta);

            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn delta_commutativity(
            a: PNCounter<K>,
            de1: <PNCounter<K> as Delta>::De,
            de2: <PNCounter<K> as Delta>::De,
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

        fn delta_idempotence(a: PNCounter<K>, de1: <PNCounter<K> as Delta>::De) -> bool {
            let mut a1 = a.clone();
            a1.apply_delta(&de1.clone());
            a1.apply_delta(&de1.clone());
            let mut a2 = a.clone();
            a2.apply_delta(&de1.clone());
            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }
    }
}

#[cfg(test)]
mod correctness_tests {
    use crate::crdt_data_type_impl::counter::pncounter::{Operation, PNCounter};
    use crate::crdt_sync_type::Delta;
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use crate::get_current_timestamp;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = PNCounter::<String>::new();
        let mut b = PNCounter::<String>::new();
        let mut c = PNCounter::<String>::new();

        a.increment("a".to_string());
        b.increment("b".to_string());
        c.increment("c".to_string());

        assert!(PNCounter::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(PNCounter::<String>::cvrdt_commutativity(
            a.clone(),
            b.clone()
        ));
        assert!(PNCounter::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let a = PNCounter::<String>::new();
        let op1 = Operation::Increment {
            key: "a".to_string(),
            timestamp: get_current_timestamp(),
        };
        let op2 = Operation::Increment {
            key: "b".to_string(),
            timestamp: get_current_timestamp() + 1,
        };
        assert!(PNCounter::<String>::cmrdt_commutativity(
            a.clone(),
            op1.clone(),
            op2.clone()
        ));
        assert!(PNCounter::<String>::cmrdt_idempotence(
            a.clone(),
            op1.clone()
        ));
        assert!(PNCounter::<String>::cmrdt_sequential_consistency(
            a.clone(),
            vec![op1.clone(), op2.clone()]
        ));
    }

    #[test]
    fn test_delta_validation() {
        let mut a = PNCounter::<String>::new();
        let mut b = PNCounter::<String>::new();
        let mut c = PNCounter::<String>::new();

        a.increment("x".to_string());
        a.increment("x".to_string());
        b.increment("x".to_string());
        b.increment("y".to_string());
        c.increment("z".to_string());

        let d1 = a.generate_delta();
        let d2 = b.generate_delta();
        let d3 = c.generate_delta();

        assert!(PNCounter::<String>::delta_associativity(
            a.clone(),
            d1.clone(),
            d2.clone(),
            d3.clone()
        ));
        assert!(PNCounter::<String>::delta_commutativity(
            a.clone(),
            d1.clone(),
            d2.clone()
        ));
        assert!(PNCounter::<String>::delta_idempotence(
            a.clone(),
            d1.clone()
        ));
    }
}
