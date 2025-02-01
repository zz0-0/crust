#[cfg(test)]
mod tests {
    use crate::crdt_data_type_impl::graph::orgraph::ORGraph;
    use crate::crdt_sync_type::{CmRDT, CvRDT, Delta};
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use rand::seq::SliceRandom;
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;

    impl<K> CvRDTValidation<ORGraph<K>> for ORGraph<K>
    where
        K: Eq + Hash + Clone + Debug,
    {
        fn cvrdt_associativity(a: ORGraph<K>, b: ORGraph<K>, c: ORGraph<K>) -> bool {
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

        fn cvrdt_commutativity(a: ORGraph<K>, b: ORGraph<K>) -> bool {
            let mut a_b = a.clone();
            a_b.merge(&b);
            let mut b_a = b.clone();
            b_a.merge(&a);
            println!("{:?} {:?}", a_b, b_a);
            a_b == b_a
        }

        fn cvrdt_idempotence(a: ORGraph<K>) -> bool {
            let mut a_a = a.clone();
            a_a.merge(&a);
            println!("{:?} {:?}", a_a, a);
            a_a == a
        }
    }

    impl<K> CmRDTValidation<ORGraph<K>> for ORGraph<K>
    where
        K: Eq + Hash + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
    {
        fn cmrdt_commutativity(
            a: ORGraph<K>,
            op1: <ORGraph<K> as CmRDT>::Op,
            op2: <ORGraph<K> as CmRDT>::Op,
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

        fn cmrdt_idempotence(a: ORGraph<K>, op1: <ORGraph<K> as CmRDT>::Op) -> bool {
            let mut a1 = a.clone();
            a1.apply(&op1);
            a1.apply(&op1);
            let mut a2 = a.clone();
            a2.apply(&op1);
            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn cmrdt_sequential_consistency(
            a: ORGraph<K>,
            ops: Vec<<ORGraph<K> as CmRDT>::Op>,
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

    impl<K> DeltaValidation<ORGraph<K>> for ORGraph<K>
    where
        K: Eq + Hash + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
    {
        fn delta_associativity(
            a: ORGraph<K>,
            de1: <ORGraph<K> as Delta>::De,
            de2: <ORGraph<K> as Delta>::De,
            de3: <ORGraph<K> as Delta>::De,
        ) -> bool {
            let mut a1 = a.clone();
            a1.apply_delta(&de1.clone());
            a1.apply_delta(&de2.clone());
            a1.apply_delta(&de3.clone());

            let mut a2 = a.clone();
            let mut combined_delta = ORGraph {
                vertices: de2.vertices.clone(),
                edges: de2.edges.clone(),
                previous_vertices: de2.previous_vertices.clone(),
                previous_edges: de2.previous_edges.clone(),
            };

            for (vertex, history) in de3.vertices {
                combined_delta
                    .vertices
                    .entry(vertex)
                    .or_insert(HashSet::new())
                    .extend(history);
            }

            for (edge, history) in de3.edges {
                combined_delta
                    .edges
                    .entry(edge)
                    .or_insert(HashSet::new())
                    .extend(history);
            }

            a2.apply_delta(&de1);
            a2.apply_delta(&combined_delta);

            println!("{:?} {:?}", a1, a2);
            a1 == a2
        }

        fn delta_commutativity(
            a: ORGraph<K>,
            de1: <ORGraph<K> as Delta>::De,
            de2: <ORGraph<K> as Delta>::De,
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

        fn delta_idempotence(a: ORGraph<K>, de1: <ORGraph<K> as Delta>::De) -> bool {
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
    use crate::crdt_data_type_impl::graph::orgraph::{ORGraph, Operation};
    use crate::crdt_sync_type::Delta;
    use crate::crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation};
    use crate::get_current_timestamp;

    #[test]
    fn test_cvrdt_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = ORGraph::<String>::new();
        let mut g2 = ORGraph::<String>::new();
        let mut g3 = ORGraph::<String>::new();

        // Test vertex operations
        g1.add_vertex("a".to_string(), timpstamp);
        g1.add_vertex("b".to_string(), timpstamp + 1);
        g2.add_vertex("b".to_string(), timpstamp + 2);
        g2.add_vertex("c".to_string(), timpstamp + 3);
        g3.add_vertex("c".to_string(), timpstamp + 4);
        g3.add_vertex("d".to_string(), timpstamp + 5);

        // Test edge operations
        g1.add_edge("a".to_string(), "b".to_string(), timpstamp + 6);
        g2.add_edge("b".to_string(), "c".to_string(), timpstamp + 7);
        g3.add_edge("c".to_string(), "d".to_string(), timpstamp + 8);

        g1.remove_vertex("a".to_string(), timpstamp + 9);
        g2.remove_edge("b".to_string(), "c".to_string(), timpstamp + 10);

        assert!(ORGraph::<String>::cvrdt_associativity(
            g1.clone(),
            g2.clone(),
            g3.clone()
        ));
        assert!(ORGraph::<String>::cvrdt_commutativity(
            g1.clone(),
            g2.clone()
        ));
        assert!(ORGraph::<String>::cvrdt_idempotence(g1.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = ORGraph::<String>::new();
        let mut g2 = ORGraph::<String>::new();
        let mut g3 = ORGraph::<String>::new();

        g1.add_vertex("a".to_string(), timpstamp);
        g1.add_vertex("b".to_string(), timpstamp + 1);
        g2.add_vertex("b".to_string(), timpstamp + 2);
        g2.add_vertex("c".to_string(), timpstamp + 3);
        g3.add_vertex("c".to_string(), timpstamp + 4);
        g3.add_vertex("d".to_string(), timpstamp + 5);

        let op1 = Operation::AddVertex {
            vertex: "x".to_string(),
            timestamp: timpstamp + 6,
        };
        let op2 = Operation::AddEdge {
            from: "x".to_string(),
            to: "b".to_string(),
            timestamp: timpstamp + 7,
        };
        let op3 = Operation::RemoveVertex {
            vertex: "x".to_string(),
            timestamp: timpstamp + 8,
        };

        assert!(ORGraph::<String>::cmrdt_commutativity(
            g1.clone(),
            op1.clone(),
            op2.clone()
        ));

        // Test idempotence of operations
        assert!(ORGraph::<String>::cmrdt_idempotence(
            g1.clone(),
            op1.clone()
        ));

        // Test sequential consistency
        let ops = vec![op1, op2, op3];
        assert!(ORGraph::<String>::cmrdt_sequential_consistency(
            g1.clone(),
            ops
        ));
    }

    #[test]
    fn test_delta_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = ORGraph::<String>::new();
        let mut g2 = ORGraph::<String>::new();
        let mut g3 = ORGraph::<String>::new();

        g1.add_vertex("a".to_string(), timpstamp);
        g1.add_vertex("b".to_string(), timpstamp + 1);
        g2.add_vertex("b".to_string(), timpstamp + 2);
        g2.add_vertex("c".to_string(), timpstamp + 3);
        g3.add_vertex("c".to_string(), timpstamp + 4);
        g3.add_vertex("d".to_string(), timpstamp + 5);

        let mut delta_graph = g1.clone();
        delta_graph.add_vertex("x".to_string(), timpstamp + 6);
        delta_graph.add_edge("x".to_string(), "b".to_string(), timpstamp + 7);
        let delta1 = delta_graph.generate_delta();
        delta_graph.add_vertex("y".to_string(), timpstamp + 8);
        let delta2 = delta_graph.generate_delta();
        delta_graph.add_edge("y".to_string(), "x".to_string(), timpstamp + 9);
        let delta3 = delta_graph.generate_delta();

        assert!(ORGraph::<String>::delta_associativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone(),
            delta3.clone()
        ));

        assert!(ORGraph::<String>::delta_commutativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone()
        ));

        assert!(ORGraph::<String>::delta_idempotence(
            g1.clone(),
            delta1.clone()
        ));
    }
}
