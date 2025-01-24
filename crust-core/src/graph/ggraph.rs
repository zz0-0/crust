use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{collections::HashMap, fmt::Debug};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GGraph<K>
where
    K: Eq + Hash,
{
    vertices: HashMap<K, u128>,
    edges: HashMap<(K, K), u128>,
    previous_vertices: HashMap<K, u128>,
    previsou_edges: HashMap<(K, K), u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    AddVertex { vertex: K, timestamp: u128 },
    AddEdge { from: K, to: K, timestamp: u128 },
}

impl<K> GGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            previous_vertices: HashMap::new(),
            previsou_edges: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn to_crdt(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_delta(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn add_vertex(&mut self, vertex: K, timestamp: u128) {
        match self.vertices.get(&vertex) {
            Some(&ts) if ts >= timestamp => return,
            _ => {
                self.vertices.insert(vertex.clone(), timestamp);
            }
        };
    }

    pub fn add_edge(&mut self, from: K, to: K, timestamp: u128) {
        if self.vertices.contains_key(&from) && self.vertices.contains_key(&to) {
            match self.edges.get(&(from.clone(), to.clone())) {
                Some(&edge_ts) if edge_ts >= timestamp => return,
                _ => {
                    self.edges.insert((from.clone(), to.clone()), timestamp);
                }
            };
        }
    }
}

impl<K> CmRDT for GGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Operation::AddVertex {
                ref vertex,
                timestamp,
            } => {
                self.add_vertex(vertex.clone(), timestamp);
            }
            Operation::AddEdge {
                ref from,
                ref to,
                timestamp,
            } => {
                self.add_edge(from.clone(), to.clone(), timestamp);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert {
                position: _,
                value: _,
            } => vec![],
            TextOperation::Delete {
                position: _,
                value: _,
            } => vec![],
        }
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> CvRDT for GGraph<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (vertex, &other_ts) in &other.vertices {
            match self.vertices.get(vertex) {
                Some(&self_ts) if self_ts >= other_ts => (),
                _ => {
                    self.vertices.insert(vertex.clone(), other_ts);
                }
            }
        }
        for ((from, to), &other_ts) in &other.edges {
            if self.vertices.contains_key(from) && self.vertices.contains_key(to) {
                match self.edges.get(&(from.clone(), to.clone())) {
                    Some(&self_ts) if self_ts >= other_ts => (),
                    _ => {
                        self.edges.insert((from.clone(), to.clone()), other_ts);
                    }
                }
            }
        }
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> Delta for GGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = GGraph<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = GGraph::new();
        for (k, ts) in &self.vertices {
            match self.previous_vertices.get(k) {
                Some(&since_ts) if since_ts >= *ts => continue,
                _ => {
                    delta.vertices.insert(k.clone(), *ts);
                }
            };
        }
        for ((from, to), ts) in &self.edges {
            match self.previsou_edges.get(&(from.clone(), to.clone())) {
                Some(&since_ts) if since_ts >= *ts => continue,
                _ => {
                    delta.edges.insert((from.clone(), to.clone()), *ts);
                }
            };
        }
        delta
    }

    fn merge_delta(&mut self, delta: &Self::De) {
        for (k, ts) in &delta.vertices {
            match self.vertices.get(k) {
                Some(current_ts) if current_ts >= ts => continue,
                _ => {
                    self.vertices.insert(k.clone(), ts.clone());
                }
            };
        }
        for ((from, to), timestamp) in &delta.edges {
            if self.vertices.contains_key(from) && self.vertices.contains_key(to) {
                match self.edges.get(&(from.clone(), to.clone())) {
                    Some(current_ts) if current_ts >= timestamp => continue,
                    _ => {
                        self.edges.insert((from.clone(), to.clone()), *timestamp);
                    }
                };
            }
        }
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> CvRDTValidation<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Debug,
{
    fn cvrdt_associativity(a: GGraph<K>, b: GGraph<K>, c: GGraph<K>) -> bool {
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

    fn cvrdt_commutativity(a: GGraph<K>, b: GGraph<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: GGraph<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
{
    fn cmrdt_commutativity(
        a: GGraph<K>,
        op1: <GGraph<K> as CmRDT>::Op,
        op2: <GGraph<K> as CmRDT>::Op,
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

    fn cmrdt_idempotence(a: GGraph<K>, op1: <GGraph<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(a: GGraph<K>, ops: Vec<<GGraph<K> as CmRDT>::Op>) -> bool {
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

impl<K> DeltaValidation<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
{
    fn delta_associativity(
        a: GGraph<K>,
        de1: <GGraph<K> as Delta>::De,
        de2: <GGraph<K> as Delta>::De,
        de3: <GGraph<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de2.clone());
        a1.merge_delta(&de3.clone());

        let mut a2 = a.clone();
        let mut combined_delta = GGraph {
            vertices: de2.vertices.clone(),
            edges: de2.edges.clone(),
            previous_vertices: de2.previous_vertices.clone(),
            previsou_edges: de2.previsou_edges.clone(),
        };

        // Merge de3 into combined_delta following AWGraph merge rules
        for (k, v) in de3.vertices {
            match combined_delta.vertices.get(&k) {
                Some(&existing_ts) if existing_ts >= v => continue,
                _ => {
                    combined_delta.vertices.insert(k, v);
                }
            }
        }

        for ((from, to), v) in de3.edges {
            match combined_delta.edges.get(&(from.clone(), to.clone())) {
                Some(&existing_ts) if existing_ts >= v => continue,
                _ => {
                    combined_delta.edges.insert((from, to), v);
                }
            }
        }

        a2.merge_delta(&de1);
        a2.merge_delta(&combined_delta);

        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_commutativity(
        a: GGraph<K>,
        de1: <GGraph<K> as Delta>::De,
        de2: <GGraph<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de2.clone());
        let mut a2 = a.clone();
        a2.merge_delta(&de2);
        a2.merge_delta(&de1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_idempotence(a: GGraph<K>, de1: <GGraph<K> as Delta>::De) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de1.clone());
        let mut a2 = a.clone();
        a2.merge_delta(&de1.clone());
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }
}

#[cfg(test)]
mod tests {
    use crate::get_current_timestamp;

    use super::*;

    #[test]
    fn test_cvrdt_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = GGraph::<String>::new();
        let mut g2 = GGraph::<String>::new();
        let mut g3 = GGraph::<String>::new();

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

        assert!(GGraph::<String>::cvrdt_associativity(
            g1.clone(),
            g2.clone(),
            g3.clone()
        ));
        assert!(GGraph::<String>::cvrdt_commutativity(
            g1.clone(),
            g2.clone()
        ));
        assert!(GGraph::<String>::cvrdt_idempotence(g1.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = GGraph::<String>::new();
        let mut g2 = GGraph::<String>::new();
        let mut g3 = GGraph::<String>::new();

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
        let op3 = Operation::AddVertex {
            vertex: "s".to_string(),
            timestamp: timpstamp + 8,
        };

        assert!(GGraph::<String>::cmrdt_commutativity(
            g1.clone(),
            op1.clone(),
            op2.clone()
        ));

        // Test idempotence of operations
        assert!(GGraph::<String>::cmrdt_idempotence(g1.clone(), op1.clone()));

        // Test sequential consistency
        let ops = vec![op1, op2, op3];
        assert!(GGraph::<String>::cmrdt_sequential_consistency(
            g1.clone(),
            ops
        ));
    }

    #[test]
    fn test_delta_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = GGraph::<String>::new();
        let mut g2 = GGraph::<String>::new();
        let mut g3 = GGraph::<String>::new();

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

        assert!(GGraph::<String>::delta_associativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone(),
            delta3.clone()
        ));

        assert!(GGraph::<String>::delta_commutativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone()
        ));

        assert!(GGraph::<String>::delta_idempotence(
            g1.clone(),
            delta1.clone()
        ));
    }
}
