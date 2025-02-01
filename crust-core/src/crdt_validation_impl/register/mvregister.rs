#[cfg(test)]
mod tests {
    use crate::crdt_data_type_impl::register::mvregister::MVRegister;
    use crate::crdt_sync_type::{CmRDT, CvRDT, Delta};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use rand::seq::SliceRandom;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::Hash;

    impl<K> CvRDTValidation<MVRegister<K>> for MVRegister<K>
    where
        K: Eq + Hash + Clone + Debug,
    {
        fn cvrdt_associativity(a: MVRegister<K>, b: MVRegister<K>, c: MVRegister<K>) -> bool {
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

        fn cvrdt_commutativity(a: MVRegister<K>, b: MVRegister<K>) -> bool {
            let mut a_b = a.clone();
            a_b.merge(&b);
            let mut b_a = b.clone();
            b_a.merge(&a);
            println!("{:?} {:?}", a_b, b_a);
            a_b == b_a
        }

        fn cvrdt_idempotence(a: MVRegister<K>) -> bool {
            let mut a_a = a.clone();
            a_a.merge(&a);
            println!("{:?} {:?}", a_a, a);
            a_a == a
        }
    }

    impl<K> CmRDTValidation<MVRegister<K>> for MVRegister<K>
    where
        K: Eq + Hash + Clone,
        MVRegister<K>: CmRDT + Debug,
    {
        fn cmrdt_commutativity(
            a: MVRegister<K>,
            op1: <MVRegister<K> as CmRDT>::Op,
            op2: <MVRegister<K> as CmRDT>::Op,
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

        fn cmrdt_idempotence(a: MVRegister<K>, op1: <MVRegister<K> as CmRDT>::Op) -> bool {
            let mut a1 = a.clone();
            a1.apply(&op1);
            a1.apply(&op1);
            let mut a2 = a.clone();
            a2.apply(&op1);
            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn cmrdt_sequential_consistency(
            a: MVRegister<K>,
            ops: Vec<<MVRegister<K> as CmRDT>::Op>,
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

    impl<K> DeltaValidation<MVRegister<K>> for MVRegister<K>
    where
        K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
        MVRegister<K>: Delta<De = MVRegister<K>> + Debug,
    {
        fn delta_associativity(
            a: MVRegister<K>,
            de1: <MVRegister<K> as Delta>::De,
            de2: <MVRegister<K> as Delta>::De,
            de3: <MVRegister<K> as Delta>::De,
        ) -> bool {
            let mut a1 = a.clone();
            a1.apply_delta(&de1.clone());
            a1.apply_delta(&de2.clone());
            a1.apply_delta(&de3.clone());

            let mut a2 = a.clone();
            let mut combined_delta = MVRegister::new();

            // Merge values from de2
            for (replica_id, (value, timestamp)) in &de2.values {
                combined_delta
                    .values
                    .insert(*replica_id, (value.clone(), *timestamp));
            }
            combined_delta
                .tombstones
                .extend(de2.tombstones.iter().cloned());

            // Merge values from de3
            for (replica_id, (value, timestamp)) in &de3.values {
                match combined_delta.values.get(replica_id) {
                    Some((_, current_timestamp)) if timestamp > current_timestamp => {
                        combined_delta
                            .values
                            .insert(*replica_id, (value.clone(), *timestamp));
                    }
                    None => {
                        combined_delta
                            .values
                            .insert(*replica_id, (value.clone(), *timestamp));
                    }
                    _ => {}
                }
            }
            combined_delta
                .tombstones
                .extend(de3.tombstones.iter().cloned());

            a2.apply_delta(&de1);
            a2.apply_delta(&combined_delta);

            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn delta_commutativity(
            a: MVRegister<K>,
            de1: <MVRegister<K> as Delta>::De,
            de2: <MVRegister<K> as Delta>::De,
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

        fn delta_idempotence(a: MVRegister<K>, de1: <MVRegister<K> as Delta>::De) -> bool {
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
    use crate::crdt_data_type_impl::register::mvregister::{MVRegister, Operation};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use crate::get_current_timestamp;
    use uuid::Uuid;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = MVRegister::<String>::new();
        let mut b = MVRegister::<String>::new();
        let mut c = MVRegister::<String>::new();

        a.write(Uuid::new_v4(), "a".to_string());
        b.write(Uuid::new_v4(), "b".to_string());
        c.write(Uuid::new_v4(), "c".to_string());

        assert!(MVRegister::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(MVRegister::<String>::cvrdt_commutativity(
            a.clone(),
            b.clone()
        ));
        assert!(MVRegister::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let mut a = MVRegister::<String>::new();
        a.write(Uuid::new_v4(), "a".to_string());
        let op1 = Operation::Write {
            value: "b".to_string(),
            replica_id: Uuid::new_v4(),
            // Use same timestamp for operation
        };
        let op2 = Operation::Write {
            value: "c".to_string(),
            replica_id: Uuid::new_v4(),
            // Use same timestamp for operation
        };
        assert!(MVRegister::<String>::cmrdt_commutativity(
            a.clone(),
            op1.clone(),
            op2.clone()
        ));
        assert!(MVRegister::<String>::cmrdt_idempotence(
            a.clone(),
            op1.clone()
        ));
        assert!(MVRegister::<String>::cmrdt_sequential_consistency(
            a.clone(),
            vec![op1.clone(), op2.clone()]
        ));
    }

    #[test]
    fn test_delta_validation() {
        let mut a = MVRegister::<String>::new();
        let mut b = MVRegister::<String>::new();

        // Create initial state
        a.write(Uuid::new_v4(), "a".to_string());

        // Generate meaningful deltas
        let de1 = a.clone(); // Use full state as delta

        // Create different state for second delta
        b.write(Uuid::new_v4(), "b".to_string());
        let de2 = b.clone();

        // Create different state for third delta
        let mut c = MVRegister::<String>::new();
        c.write(Uuid::new_v4(), "c".to_string());
        let de3 = c.clone();
        assert!(MVRegister::<String>::delta_associativity(
            a.clone(),
            de1.clone(),
            de2.clone(),
            de3.clone()
        ));
        assert!(MVRegister::<String>::delta_commutativity(
            a.clone(),
            de1.clone(),
            de2.clone()
        ));
        assert!(MVRegister::<String>::delta_idempotence(
            a.clone(),
            de1.clone()
        ));
    }
}
