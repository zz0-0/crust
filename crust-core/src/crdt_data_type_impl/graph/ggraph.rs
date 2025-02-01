use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{collections::HashMap, fmt::Debug};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GGraph<K>
where
    K: Eq + Hash,
{
    pub vertices: HashMap<K, u128>,
    pub edges: HashMap<(K, K), u128>,
    pub previous_vertices: HashMap<K, u128>,
    pub previsou_edges: HashMap<(K, K), u128>,
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

    pub fn name(&self) -> String {
        "ggraph".to_string()
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

    fn apply_delta(&mut self, delta: &Self::De) {
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
}
