use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    crdt_validation::{CmRDTValidation, CvRDTValidation, DeltaValidation},
    text_operation::TextOperation,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use std::{collections::HashSet, hash::Hash};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct TPGraph<K>
where
    K: Eq + Hash,
{
    node_id: Uuid,
    vertices: HashMap<K, (u128, VertexState)>,
    edges: HashMap<(K, K), (u128, EdgeState)>,
    tombstones: HashMap<K, HashSet<u128>>,
    removal_candidates: HashMap<K, (u128, HashSet<Uuid>)>,
    previous_vertices: HashMap<K, (u128, VertexState)>,
    previous_edges: HashMap<(K, K), (u128, EdgeState)>,
    previous_tombstones: HashMap<K, HashSet<u128>>,
    previous_removal_candidates: HashMap<K, (u128, HashSet<Uuid>)>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VertexState {
    Active,
    MarkedForRemoval,
    Removed,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum EdgeState {
    Active,
    Removed,
}

#[derive(Clone)]
pub enum Operation<K> {
    AddVertex { vertex: K, timestamp: u128 },
    AddEdge { from: K, to: K, timestamp: u128 },
    RemoveVertex { vertex: K, timestamp: u128 },
    RemoveEdge { from: K, to: K, timestamp: u128 },
}

impl<K> TPGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            node_id: Uuid::new_v4(),
            vertices: HashMap::new(),
            edges: HashMap::new(),
            tombstones: HashMap::new(),
            removal_candidates: HashMap::new(),
            previous_vertices: HashMap::new(),
            previous_edges: HashMap::new(),
            previous_tombstones: HashMap::new(),
            previous_removal_candidates: HashMap::new(),
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
            Some((ts, state)) => {
                if timestamp > *ts && *state != VertexState::Removed {
                    self.vertices
                        .insert(vertex, (timestamp, VertexState::Active));
                }
            }
            None => {
                self.vertices
                    .insert(vertex, (timestamp, VertexState::Active));
            }
        }
    }

    pub fn is_vertex_active(&self, vertex: &K) -> bool {
        matches!(self.vertices.get(vertex), Some((_, VertexState::Active)))
    }

    pub fn add_edge(&mut self, from: K, to: K, timestamp: u128) {
        if self.is_vertex_active(&from) && self.is_vertex_active(&to) {
            self.edges
                .insert((from, to), (timestamp, EdgeState::Active));
        }
    }

    pub fn mark_vertex_for_removal(&mut self, vertex: K, timestamp: u128) {
        if let Some((ts, state)) = self.vertices.get(&vertex) {
            if timestamp > *ts && *state == VertexState::Active {
                self.vertices
                    .insert(vertex.clone(), (timestamp, VertexState::MarkedForRemoval));
                self.removal_candidates
                    .insert(vertex, (timestamp, HashSet::new()));
            }
        }
    }

    pub fn acknowledge_removal(&mut self, vertex: K, from_node: Uuid, timestamp: u128) {
        if let Some((ts, acks)) = self.removal_candidates.get_mut(&vertex) {
            if timestamp > *ts {
                acks.insert(from_node);
                if self.has_majority_acks(&vertex) {
                    self.complete_removal(vertex, timestamp);
                }
            }
        }
    }

    pub fn has_majority_acks(&mut self, vertex: &K) -> bool {
        if let Some((_, acks)) = self.removal_candidates.get(vertex) {
            let total_nodes = acks.len();
            if total_nodes >= 3 {
                return true;
            }
        }
        false
    }

    pub fn complete_removal(&mut self, vertex: K, timestamp: u128) {
        if let Some((_, state)) = self.vertices.get(&vertex) {
            if *state == VertexState::MarkedForRemoval {
                self.vertices
                    .insert(vertex.clone(), (timestamp, VertexState::Removed));
                self.tombstones.insert(vertex.clone(), HashSet::new());
                self.removal_candidates.remove(&vertex);
                self.remove_connected_edge(&vertex, timestamp);
            }
        }
    }

    pub fn remove_edge(&mut self, from: K, to: K, timestamp: u128) {
        if let Some((ts, state)) = self.edges.get(&(from.clone(), to.clone())) {
            if timestamp > *ts && *state == EdgeState::Active {
                self.edges
                    .insert((from, to), (timestamp, EdgeState::Removed));
            }
        }
    }

    pub fn remove_connected_edge(&mut self, vertex: &K, timestamp: u128) {
        let remove_edges: Vec<(K, K)> = self
            .edges
            .iter()
            .filter(|((from, to), (ts, state))| {
                (*from == *vertex || *to == *vertex) && *state == EdgeState::Active
            })
            .map(|(k, _)| k.clone())
            .collect();
        for edge in remove_edges {
            self.edges.insert(edge, (timestamp, EdgeState::Removed));
        }
    }

    pub fn name(&self) -> String {
        "TPGraph".to_string()
    }
}

impl<K> CmRDT for TPGraph<K>
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
                // self.remove_vertex(vertex.clone(), timestamp);
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
}

impl<K> CvRDT for TPGraph<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (vertex, (ts, state)) in &other.vertices {
            match self.vertices.get(vertex) {
                Some((self_ts, self_state)) => {
                    if ts > self_ts {
                        self.vertices
                            .insert(vertex.clone(), (ts.clone(), state.clone()));
                    }
                }
                None => {
                    self.vertices
                        .insert(vertex.clone(), (ts.clone(), state.clone()));
                }
            }
        }

        for (edge, (ts, state)) in &other.edges {
            match self.edges.get(edge) {
                Some((self_ts, self_state)) => {
                    if ts > self_ts {
                        self.edges.insert(edge.clone(), (ts.clone(), state.clone()));
                    }
                }
                None => {
                    self.edges.insert(edge.clone(), (ts.clone(), state.clone()));
                }
            }
        }

        for (vertex, acks) in &other.removal_candidates {
            match self.removal_candidates.get(vertex) {
                Some((self_ts, self_acks)) => {
                    if acks.1.len() > self_acks.len() {
                        self.removal_candidates
                            .insert(vertex.clone(), (acks.0.clone(), acks.1.clone()));
                    }
                }
                None => {
                    self.removal_candidates
                        .insert(vertex.clone(), (acks.0.clone(), acks.1.clone()));
                }
            }
        }

        for (vertex, ts) in &other.tombstones {
            match self.tombstones.get(vertex) {
                Some(self_ts) => {
                    if ts.len() > self_ts.len() {
                        self.tombstones.insert(vertex.clone(), ts.clone());
                    }
                }
                None => {
                    self.tombstones.insert(vertex.clone(), ts.clone());
                }
            }
        }
    }
}

impl<K> Delta for TPGraph<K>
where
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    type De = TPGraph<K>;

    fn generate_delta(&self) -> Self::De {
        TPGraph {
            node_id: self.node_id,
            vertices: self
                .vertices
                .iter()
                .filter(|(k, (ts, _))| {
                    self.previous_vertices
                        .get(k)
                        .map_or(true, |(ts2, _)| ts > ts2)
                })
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            edges: self
                .edges
                .iter()
                .filter(|(k, (ts, _))| self.previous_edges.get(k).map_or(true, |(ts2, _)| ts > ts2))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            tombstones: self
                .tombstones
                .iter()
                .filter(|(k, v)| {
                    self.previous_tombstones
                        .get(k)
                        .map_or(true, |v2| v.len() > v2.len())
                })
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            removal_candidates: self
                .removal_candidates
                .iter()
                .filter(|(k, (ts, _))| {
                    self.previous_removal_candidates
                        .get(k)
                        .map_or(true, |(ts2, _)| ts > ts2)
                })
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            previous_edges: self.edges.clone(),
            previous_vertices: self.vertices.clone(),
            previous_tombstones: self.tombstones.clone(),
            previous_removal_candidates: self.removal_candidates.clone(),
        }
    }

    fn apply_delta(&mut self, delta: &Self::De) {
        self.merge(&delta);
    }
}

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

    fn cmrdt_sequential_consistency(a: TPGraph<K>, ops: Vec<<TPGraph<K> as CmRDT>::Op>) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

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
