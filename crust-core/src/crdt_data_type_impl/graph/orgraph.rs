use crate::{
    crdt_sync_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ORGraph<K>
where
    K: Eq + Hash,
{
    pub vertices: HashMap<K, HashSet<(u128, bool)>>,
    pub edges: HashMap<(K, K), HashSet<(u128, bool)>>,
    pub previous_vertices: HashMap<K, HashSet<(u128, bool)>>,
    pub previous_edges: HashMap<(K, K), HashSet<(u128, bool)>>,
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
        "orgraph".to_string()
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
