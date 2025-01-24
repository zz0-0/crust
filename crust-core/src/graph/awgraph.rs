// The problem occurs because operations have dependencies:
// Adding an edge requires vertices to exist
// Order matters: vertices must be added before edges
// For AWGraph, strict commutativity isn't possible due to these dependencies
// Solutions:
// Buffer edge operations until vertices exist
// Track "pending edges" that will be added once vertices become available
// Or acknowledge this is an inherent limitation of AWGraph's design

// due to the nature of the graph , the (vertex, edge) operations are not commutative

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
pub struct AWGraph<K>
where
    K: Eq + Hash,
{
    vertices: HashMap<K, u128>,
    edges: HashMap<(K, K), u128>,
    previous_vertices: HashMap<K, u128>,
    previsou_edges: HashMap<(K, K), u128>,
    removed_vertices: HashMap<K, u128>,
    removed_edges: HashMap<(K, K), u128>,
    previous_removed_vertices: HashMap<K, u128>,
    previous_removed_edges: HashMap<(K, K), u128>,
}

#[derive(Clone)]
pub enum Operation<K> {
    AddVertex { vertex: K, timestamp: u128 },
    AddEdge { from: K, to: K, timestamp: u128 },
    RemoveVertex { vertex: K, timestamp: u128 },
    RemoveEdge { from: K, to: K, timestamp: u128 },
}

impl<K> AWGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            previous_vertices: HashMap::new(),
            previsou_edges: HashMap::new(),
            removed_vertices: HashMap::new(),
            removed_edges: HashMap::new(),
            previous_removed_vertices: HashMap::new(),
            previous_removed_edges: HashMap::new(),
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
        if let Some(&remove_ts) = self.removed_vertices.get(&vertex) {
            if timestamp > remove_ts {
                self.vertices.insert(vertex.clone(), timestamp);
                self.removed_vertices.remove(&vertex);
            }
        } else {
            self.vertices.insert(vertex.clone(), timestamp);
        }
    }

    pub fn add_edge(&mut self, from: K, to: K, timestamp: u128) {
        if self.vertices.contains_key(&from) && self.vertices.contains_key(&to) {
            if let Some(&remove_ts) = self.removed_edges.get(&(from.clone(), to.clone())) {
                if timestamp > remove_ts {
                    self.edges.insert((from.clone(), to.clone()), timestamp);
                    self.removed_edges.remove(&(from.clone(), to.clone()));
                }
            } else {
                self.edges.insert((from.clone(), to.clone()), timestamp);
            }
        }
    }

    pub fn remove_vertex(&mut self, vertex: K, timestamp: u128) {
        if let Some(vertex_ts) = self.vertices.get(&vertex) {
            if timestamp > *vertex_ts {
                self.vertices.remove(&vertex);
                self.removed_vertices.insert(vertex.clone(), timestamp);
                self.edges.retain(|&(ref from, ref to), &mut edge_ts| {
                    from != &vertex && to != &vertex || edge_ts > timestamp
                });
            }
        }
    }

    pub fn remove_edge(&mut self, from: K, to: K, timestamp: u128) {
        if let Some(&add_ts) = self.edges.get(&(from.clone(), to.clone())) {
            if timestamp > add_ts {
                self.edges.remove(&(from.clone(), to.clone()));
                self.removed_edges
                    .insert((from.clone(), to.clone()), timestamp);
            }
        }
    }
}

impl<K> CmRDT for AWGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: &Self::Op) {
        match *op {
            Self::Op::AddVertex {
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
            Operation::RemoveVertex {
                ref vertex,
                timestamp,
            } => {
                self.remove_vertex(vertex.clone(), timestamp);
            }
            Operation::RemoveEdge {
                ref from,
                ref to,
                timestamp,
            } => {
                self.remove_edge(from.clone(), to.clone(), timestamp);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        match op {
            TextOperation::Insert {
                position: _,
                value: _,
            } => {
                vec![]
            }
            TextOperation::Delete {
                position: _,
                value: _,
            } => {
                vec![]
            }
        }
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> CvRDT for AWGraph<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (vertex, &add_ts) in &other.vertices {
            if let Some(&remove_ts) = self.removed_vertices.get(vertex) {
                if add_ts > remove_ts {
                    self.vertices.insert(vertex.clone(), add_ts);
                    self.removed_vertices.remove(vertex);
                }
            } else {
                if let Some(&self_add_ts) = self.vertices.get(vertex) {
                    if self_add_ts < add_ts {
                        self.vertices.insert(vertex.clone(), add_ts);
                    }
                } else {
                    self.vertices.insert(vertex.clone(), add_ts);
                }
            }
        }

        // Merge removed_vertices
        for (vertex, &remove_ts) in &other.removed_vertices {
            if let Some(&add_ts) = self.vertices.get(vertex) {
                if add_ts > remove_ts {
                    continue;
                }
            }
            if let Some(&self_remove_ts) = self.removed_vertices.get(vertex) {
                if self_remove_ts < remove_ts {
                    self.removed_vertices.insert(vertex.clone(), remove_ts);
                }
            } else {
                self.removed_vertices.insert(vertex.clone(), remove_ts);
            }
        }

        // Merge edges
        for ((from, to), &add_ts) in &other.edges {
            if self.vertices.contains_key(&from) && self.vertices.contains_key(&to) {
                if let Some(&remove_ts) = self.removed_edges.get(&(from.clone(), to.clone())) {
                    if add_ts > remove_ts {
                        self.edges.insert((from.clone(), to.clone()), add_ts);
                        self.removed_edges.remove(&(from.clone(), to.clone()));
                    }
                } else {
                    if let Some(&self_add_ts) = self.edges.get(&(from.clone(), to.clone())) {
                        if self_add_ts < add_ts {
                            self.edges.insert((from.clone(), to.clone()), add_ts);
                        }
                    } else {
                        self.edges.insert((from.clone(), to.clone()), add_ts);
                    }
                }
            }
        }

        // Merge removed_edges
        for ((from, to), &remove_ts) in &other.removed_edges {
            if let Some(&add_ts) = self.edges.get(&(from.clone(), to.clone())) {
                if add_ts > remove_ts {
                    continue;
                }
            }
            if let Some(&self_remove_ts) = self.removed_edges.get(&(from.clone(), to.clone())) {
                if self_remove_ts < remove_ts {
                    self.removed_edges
                        .insert((from.clone(), to.clone()), remove_ts);
                }
            } else {
                self.removed_edges
                    .insert((from.clone(), to.clone()), remove_ts);
            }
        }
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> Delta for AWGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = AWGraph<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = AWGraph::new();
        for (vertex, &add_ts) in &self.vertices {
            if !self.previous_vertices.contains_key(vertex)
                || self.previous_vertices.get(vertex).unwrap() < &add_ts
            {
                delta.vertices.insert(vertex.clone(), add_ts);
            }
        }

        // Vertices removed since `since`
        for (vertex, &remove_ts) in &self.removed_vertices {
            if !self.previous_removed_vertices.contains_key(vertex)
                || self.previous_removed_vertices.get(vertex).unwrap() < &remove_ts
            {
                delta.removed_vertices.insert(vertex.clone(), remove_ts);
            }
        }

        // Edges added since `since`
        for ((from, to), &add_ts) in &self.edges {
            if !self
                .previsou_edges
                .contains_key(&(from.clone(), to.clone()))
                || self
                    .previsou_edges
                    .get(&(from.clone(), to.clone()))
                    .unwrap()
                    < &add_ts
            {
                delta.edges.insert((from.clone(), to.clone()), add_ts);
            }
        }

        // Edges removed since `since`
        for ((from, to), &remove_ts) in &self.removed_edges {
            if !self
                .previous_removed_edges
                .contains_key(&(from.clone(), to.clone()))
                || self
                    .previous_removed_edges
                    .get(&(from.clone(), to.clone()))
                    .unwrap()
                    < &remove_ts
            {
                delta
                    .removed_edges
                    .insert((from.clone(), to.clone()), remove_ts);
            }
        }
        delta
    }

    fn merge_delta(&mut self, delta: &Self::De) {
        for (k, ts) in &delta.vertices {
            match self.vertices.get(k) {
                Some(current_ts) if current_ts >= ts => continue,
                _ => {
                    if !self.removed_vertices.contains_key(k) {
                        self.vertices.insert(k.clone(), ts.clone());
                    }
                }
            };
        }
        for ((from, to), timestamp) in &delta.edges {
            if self.vertices.contains_key(from)
                && self.vertices.contains_key(to)
                && !self.removed_vertices.contains_key(from)
                && !self.removed_vertices.contains_key(to)
                && !self.removed_edges.contains_key(&(from.clone(), to.clone()))
            {
                match self.edges.get(&(from.clone(), to.clone())) {
                    Some(current_ts) if current_ts >= timestamp => continue,
                    _ => {
                        self.edges.insert((from.clone(), to.clone()), *timestamp);
                    }
                };
            }
        }
        self.removed_vertices.extend(delta.removed_vertices.clone());
        self.removed_edges.extend(delta.removed_edges.clone());
    }

    fn name(&self) -> String {
        "PNCounter".to_string()
    }
}

impl<K> CvRDTValidation<AWGraph<K>> for AWGraph<K>
where
    K: Eq + Hash + Clone + Debug,
{
    fn cvrdt_associativity(a: AWGraph<K>, b: AWGraph<K>, c: AWGraph<K>) -> bool {
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

    fn cvrdt_commutativity(a: AWGraph<K>, b: AWGraph<K>) -> bool {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_a = b.clone();
        b_a.merge(&a);
        println!("{:?} {:?}", a_b, b_a);
        a_b == b_a
    }

    fn cvrdt_idempotence(a: AWGraph<K>) -> bool {
        let mut a_a = a.clone();
        a_a.merge(&a);
        println!("{:?} {:?}", a_a, a);
        a_a == a
    }
}

impl<K> CmRDTValidation<AWGraph<K>> for AWGraph<K>
where
    K: Eq + Hash + Clone + Debug + Serialize + for<'a> Deserialize<'a>,
{
    fn cmrdt_commutativity(
        a: AWGraph<K>,
        op1: <AWGraph<K> as CmRDT>::Op,
        op2: <AWGraph<K> as CmRDT>::Op,
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

    fn cmrdt_idempotence(a: AWGraph<K>, op1: <AWGraph<K> as CmRDT>::Op) -> bool {
        let mut a1 = a.clone();
        a1.apply(&op1);
        a1.apply(&op1);
        let mut a2 = a.clone();
        a2.apply(&op1);
        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn cmrdt_sequential_consistency(a: AWGraph<K>, ops: Vec<<AWGraph<K> as CmRDT>::Op>) -> bool {
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

impl<K> DeltaValidation<AWGraph<K>> for AWGraph<K>
where
    K: Eq + Hash + Clone + Debug,
    AWGraph<K>: Delta<De = AWGraph<K>> + Debug,
{
    fn delta_associativity(
        a: AWGraph<K>,
        de1: <AWGraph<K> as Delta>::De,
        de2: <AWGraph<K> as Delta>::De,
        de3: <AWGraph<K> as Delta>::De,
    ) -> bool {
        let mut a1 = a.clone();
        a1.merge_delta(&de1.clone());
        a1.merge_delta(&de2.clone());
        a1.merge_delta(&de3.clone());

        let mut a2 = a.clone();
        let mut combined_delta = AWGraph {
            vertices: de2.vertices.clone(),
            edges: de2.edges.clone(),
            previous_vertices: de2.previous_vertices.clone(),
            previsou_edges: de2.previsou_edges.clone(),
            removed_vertices: de2.removed_vertices.clone(),
            removed_edges: de2.removed_edges.clone(),
            previous_removed_vertices: de2.previous_removed_vertices.clone(),
            previous_removed_edges: de2.previous_removed_edges.clone(),
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

        combined_delta.removed_vertices.extend(de3.removed_vertices);
        combined_delta.removed_edges.extend(de3.removed_edges);

        a2.merge_delta(&de1);
        a2.merge_delta(&combined_delta);

        println!("{:?} {:?}", a1, a2);
        a1 == a2
    }

    fn delta_commutativity(
        a: AWGraph<K>,
        de1: <AWGraph<K> as Delta>::De,
        de2: <AWGraph<K> as Delta>::De,
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

    fn delta_idempotence(a: AWGraph<K>, de1: <AWGraph<K> as Delta>::De) -> bool {
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
        let mut g1 = AWGraph::<String>::new();
        let mut g2 = AWGraph::<String>::new();
        let mut g3 = AWGraph::<String>::new();

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

        // Remove some elements
        g1.remove_vertex("a".to_string(), timpstamp + 9);
        g2.remove_edge("b".to_string(), "c".to_string(), timpstamp + 10);

        assert!(AWGraph::<String>::cvrdt_associativity(
            g1.clone(),
            g2.clone(),
            g3.clone()
        ));
        assert!(AWGraph::<String>::cvrdt_commutativity(
            g1.clone(),
            g2.clone()
        ));
        assert!(AWGraph::<String>::cvrdt_idempotence(g1.clone()));
    }

    #[test]
    fn test_cmrdt_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = AWGraph::<String>::new();
        let mut g2 = AWGraph::<String>::new();
        let mut g3 = AWGraph::<String>::new();

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

        assert!(AWGraph::<String>::cmrdt_commutativity(
            g1.clone(),
            op1.clone(),
            op2.clone()
        ));

        // Test idempotence of operations
        assert!(AWGraph::<String>::cmrdt_idempotence(
            g1.clone(),
            op1.clone()
        ));

        // Test sequential consistency
        let ops = vec![op1, op2, op3];
        assert!(AWGraph::<String>::cmrdt_sequential_consistency(
            g1.clone(),
            ops
        ));
    }

    #[test]
    fn test_delta_validation() {
        let timpstamp = get_current_timestamp();
        let mut g1 = AWGraph::<String>::new();
        let mut g2 = AWGraph::<String>::new();
        let mut g3 = AWGraph::<String>::new();

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

        assert!(AWGraph::<String>::delta_associativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone(),
            delta3.clone()
        ));

        assert!(AWGraph::<String>::delta_commutativity(
            g1.clone(),
            delta1.clone(),
            delta2.clone()
        ));

        assert!(AWGraph::<String>::delta_idempotence(
            g1.clone(),
            delta1.clone()
        ));
    }
}
