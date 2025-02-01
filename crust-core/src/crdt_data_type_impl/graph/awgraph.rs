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
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{collections::HashMap, fmt::Debug};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct AWGraph<K>
where
    K: Eq + Hash,
{
    pub vertices: HashMap<K, u128>,
    pub edges: HashMap<(K, K), u128>,
    pub previous_vertices: HashMap<K, u128>,
    pub previsou_edges: HashMap<(K, K), u128>,
    pub removed_vertices: HashMap<K, u128>,
    pub removed_edges: HashMap<(K, K), u128>,
    pub previous_removed_vertices: HashMap<K, u128>,
    pub previous_removed_edges: HashMap<(K, K), u128>,
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

    pub fn name(&self) -> String {
        "awgraph".to_string()
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

    fn apply_delta(&mut self, delta: &Self::De) {
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
}
