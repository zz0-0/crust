#[cfg(test)]
mod tests {

    use crate::crdt_data_type_impl::set::awset::AWSet;
    use crate::crdt_sync_type::{CmRDT, CvRDT, Delta};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    impl<K> CvRDTValidation<AWSet<K>> for AWSet<K>
    where
        K: Eq + Ord + Clone + Debug,
    {
        fn cvrdt_associativity(a: AWSet<K>, b: AWSet<K>, c: AWSet<K>) -> bool {
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

        fn cvrdt_commutativity(a: AWSet<K>, b: AWSet<K>) -> bool {
            let mut a_b = a.clone();
            a_b.merge(&b);
            let mut b_a = b.clone();
            b_a.merge(&a);
            println!("{:?} {:?}", a_b, b_a);
            a_b == b_a
        }

        fn cvrdt_idempotence(a: AWSet<K>) -> bool {
            let mut a_a = a.clone();
            a_a.merge(&a);
            println!("{:?} {:?}", a_a, a);
            a_a == a
        }
    }
}

#[cfg(test)]
mod correctness_tests {
    use crate::crdt_data_type_impl::set::awset::AWSet;
    use crate::crdt_validation::CvRDTValidation;
    use crate::get_current_timestamp;

    #[test]
    fn test_cvrdt_validation() {
        let mut a = AWSet::<String>::new();
        let mut b = AWSet::<String>::new();
        let mut c = AWSet::<String>::new();

        let timestamp = get_current_timestamp();

        a.insert("a".to_string(), timestamp);
        b.insert("b".to_string(), timestamp + 1);
        c.insert("c".to_string(), timestamp + 2);
        a.remove("a".to_string(), timestamp + 3);

        assert!(AWSet::<String>::cvrdt_associativity(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(AWSet::<String>::cvrdt_commutativity(a.clone(), b.clone()));
        assert!(AWSet::<String>::cvrdt_idempotence(a.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let mut a = AWSet::<String>::new();
        let mut b = AWSet::<String>::new();
        let mut c = AWSet::<String>::new();

        let timestamp = get_current_timestamp();

        a.insert("a".to_string(), timestamp);
        b.insert("b".to_string(), timestamp + 1);
        c.insert("c".to_string(), timestamp + 2);
        a.remove("a".to_string(), timestamp + 3);
    }

    #[test]
    fn test_delta_validation() {
        let mut a = AWSet::<String>::new();
        let mut b = AWSet::<String>::new();
        let mut c = AWSet::<String>::new();

        let timestamp = get_current_timestamp();

        a.insert("a".to_string(), timestamp);
        b.insert("b".to_string(), timestamp + 1);
        c.insert("c".to_string(), timestamp + 2);
        a.remove("a".to_string(), timestamp + 3);
    }
}
