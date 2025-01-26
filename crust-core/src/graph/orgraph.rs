use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ORGraph<K>
where
    K: Eq + Hash,
{
    vertices: HashMap<K, HashSet<(u128, bool)>>,
    edges: HashMap<(K, K), HashSet<(u128, bool)>>,
    previous_vertices: HashMap<K, HashSet<(u128, bool)>>,
    previous_edges: HashMap<(K, K), HashSet<(u128, bool)>>,
}

#[derive(Clone)]
pub enum Operation<K> {
    AddVertex { vertex: K, timestamp: u128 },
    AddEdge { from: K, to: K, timestamp: u128 },
    RemoveVertex { vertex: K, timestamp: u128 },
    RemoveEdge { from: K, to: K, timestamp: u128 },
}

impl<K> ORGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            previous_vertices: HashMap::new(),
            previous_edges: HashMap::new(),
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
        let history = self.vertices.entry(vertex).or_insert(HashSet::new());

        let is_active = history
            .iter()
            .max_by_key(|(ts, _)| ts)
            .map(|(_, active)| *active)
            .unwrap_or(false);

        if !is_active {
            history.insert((timestamp, true));
        };
    }

    pub fn add_edge(&mut self, from: K, to: K, timestamp: u128) {
        let history = self.edges.entry((from, to)).or_insert_with(HashSet::new);

        let is_active = history
            .iter()
            .max_by_key(|(ts, _)| ts)
            .map(|(_, active)| *active)
            .unwrap_or(false);

        if !is_active {
            history.insert((timestamp, true));
        };
    }

    pub fn remove_vertex(&mut self, vertex: K, timestamp: u128) {
        if let Some(history) = self.vertices.get_mut(&vertex) {
            let was_active = history
                .iter()
                .max_by_key(|(ts, _)| ts)
                .map(|(_, active)| *active)
                .unwrap_or(false);

            if was_active {
                history.insert((timestamp, false));

                for (&(ref from, ref to), edge_history) in self.edges.iter_mut() {
                    if from == &vertex || to == &vertex {
                        edge_history.insert((timestamp, false));
                    }
                }
            }
        };
    }

    pub fn remove_edge(&mut self, from: K, to: K, timestamp: u128) {
        if let Some(history) = self.edges.get_mut(&(from, to)) {
            let was_active = history
                .iter()
                .max_by_key(|(ts, _)| ts)
                .map(|(_, active)| *active)
                .unwrap_or(false);

            if was_active {
                history.insert((timestamp, false));
            }
        };
    }

    pub fn name(&self) -> String {
        "ORGraph".to_string()
    }
}

impl<K> CmRDT for ORGraph<K>
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
            } => vec![],
        }
    }
}

impl<K> CvRDT for ORGraph<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (vertex, history) in &other.vertices {
            self.vertices
                .entry(vertex.clone())
                .or_insert(HashSet::new())
                .extend(history.clone());
        }
        for (edge, history) in &other.edges {
            self.edges
                .entry(edge.clone())
                .or_insert(HashSet::new())
                .extend(history.clone());
        }
    }
}

impl<K> Delta for ORGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = ORGraph<K>;

    fn generate_delta(&self) -> Self::De {
        let mut delta = ORGraph::new();

        for (vertex, history) in &self.vertices {
            let since_history = self.previous_vertices.get(vertex);
            let new_history: HashSet<_> = history
                .iter()
                .filter(|h| match since_history {
                    Some(since_history) => !since_history.contains(h),
                    None => true,
                })
                .cloned()
                .collect();
            if !new_history.is_empty() {
                delta.vertices.insert(vertex.clone(), new_history);
            }
        }
        for (edge, history) in &self.edges {
            let since_history = self.previous_edges.get(edge);
            let new_history: HashSet<_> = history
                .iter()
                .filter(|h| match since_history {
                    Some(since_history) => !since_history.contains(h),
                    None => true,
                })
                .cloned()
                .collect();
            if !new_history.is_empty() {
                delta.edges.insert(edge.clone(), new_history);
            }
        }

        delta
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        for (vertex, history) in &delta.vertices {
            self.vertices
                .entry(vertex.clone())
                .or_insert(HashSet::new())
                .extend(history.clone());
        }
        for (edge, history) in &delta.edges {
            self.edges
                .entry(edge.clone())
                .or_insert(HashSet::new())
                .extend(history.clone());
        }
    }
}

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

    fn cmrdt_sequential_consistency(a: ORGraph<K>, ops: Vec<<ORGraph<K> as CmRDT>::Op>) -> bool {
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

#[cfg(test)]
mod tests {
    use crate::get_current_timestamp;

    use super::*;

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
