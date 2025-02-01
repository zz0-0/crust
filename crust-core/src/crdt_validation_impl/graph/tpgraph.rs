#[cfg(test)]
mod tests {
    use crate::crdt_data_type_impl::graph::tpgraph::TPGraph;
    use crate::crdt_sync_type::{CmRDT, CvRDT, Delta};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use rand::seq::SliceRandom;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::Hash;

    impl<K> CvRDTValidation<TPGraph<K>> for TPGraph<K>
    where
        K: Eq + Hash + Clone + Debug,
    {
        fn cvrdt_associativity(a: TPGraph<K>, b: TPGraph<K>, c: TPGraph<K>) -> bool {
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

        fn cvrdt_commutativity(a: TPGraph<K>, b: TPGraph<K>) -> bool {
            let mut a_b = a.clone();
            a_b.merge(&b);
            let mut b_a = b.clone();
            b_a.merge(&a);
            println!("{:?} {:?}", a_b, b_a);
            a_b == b_a
        }

        fn cvrdt_idempotence(a: TPGraph<K>) -> bool {
            let mut a_a = a.clone();
            a_a.merge(&a);
            println!("{:?} {:?}", a_a, a);
            a_a == a
        }
    }

    impl<K> CmRDTValidation<TPGraph<K>> for TPGraph<K>
    where
        K: Eq + Hash + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
    {
        fn cmrdt_commutativity(
            a: TPGraph<K>,
            op1: <TPGraph<K> as CmRDT>::Op,
            op2: <TPGraph<K> as CmRDT>::Op,
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

        fn cmrdt_idempotence(a: TPGraph<K>, op1: <TPGraph<K> as CmRDT>::Op) -> bool {
            let mut a1 = a.clone();
            a1.apply(&op1);
            a1.apply(&op1);
            let mut a2 = a.clone();
            a2.apply(&op1);
            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn cmrdt_sequential_consistency(
            a: TPGraph<K>,
            ops: Vec<<TPGraph<K> as CmRDT>::Op>,
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

    impl<K> DeltaValidation<TPGraph<K>> for TPGraph<K>
    where
        K: Eq + Hash + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
    {
        fn delta_associativity(
            a: TPGraph<K>,
            de1: <TPGraph<K> as Delta>::De,
            de2: <TPGraph<K> as Delta>::De,
            de3: <TPGraph<K> as Delta>::De,
        ) -> bool {
            let mut a1 = a.clone();
            a1.apply_delta(&de1.clone());
            a1.apply_delta(&de2.clone());
            a1.apply_delta(&de3.clone());

            let mut a2 = a.clone();
            let mut combined_delta = TPGraph {
                node_id: de2.node_id,
                vertices: de2.vertices.clone(),
                edges: de2.edges.clone(),
                tombstones: de2.tombstones.clone(),
                removal_candidates: de2.removal_candidates.clone(),
                previous_vertices: de2.previous_vertices.clone(),
                previous_edges: de2.previous_edges.clone(),
                previous_tombstones: de2.previous_tombstones.clone(),
                previous_removal_candidates: de2.previous_removal_candidates.clone(),
            };

            todo!();

            a2.apply_delta(&de1);
            a2.apply_delta(&combined_delta);

            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn delta_commutativity(
            a: TPGraph<K>,
            de1: <TPGraph<K> as Delta>::De,
            de2: <TPGraph<K> as Delta>::De,
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

        fn delta_idempotence(a: TPGraph<K>, de1: <TPGraph<K> as Delta>::De) -> bool {
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
    use crate::crdt_data_type_impl::graph::tpgraph::TPGraph;
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use crate::get_current_timestamp;

    #[test]
    fn test_cvrdt_validation() {
        let mut g1 = TPGraph::<String>::new();
        let mut g2 = TPGraph::<String>::new();
        let mut g3 = TPGraph::<String>::new();

        todo!();

        assert!(TPGraph::<String>::cvrdt_associativity(
            g1.clone(),
            g2.clone(),
            g3.clone()
        ));
        assert!(TPGraph::<String>::cvrdt_commutativity(
            g1.clone(),
            g2.clone()
        ));
        assert!(TPGraph::<String>::cvrdt_idempotence(g3.clone()));
    }
}
