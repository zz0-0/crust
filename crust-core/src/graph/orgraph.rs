use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ORGraph<K>
where
    K: Eq + Hash,
{
    vertices: HashSet<(K, u128)>,
    edges: HashSet<(K, K, u128)>,
}

pub enum Operation<K> {
    AddVertex { vertex: K, timestamp: u128 },
    AddEdge { from: K, to: K, timestamp: u128 },
    RemoveVertex { vertex: K, timestamp: u128 },
    RemoveEdge { from: K, to: K, timestamp: u128 },
}

impl<K> ORGraph<K>
where
    K: Eq + Hash + Clone + Ord + Serialize,
{
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn value(&self) -> (Vec<(K, u128)>, Vec<(K, K, u128)>) {
        let mut vertices: Vec<(K, u128)> = self.vertices.iter().cloned().collect();
        let mut edges: Vec<(K, K, u128)> = self.edges.iter().cloned().collect();
        vertices.sort();
        edges.sort();
        (vertices, edges)
    }

    pub fn add_vertex(&mut self, vertex: K, timestamp: u128) {
        self.vertices.insert((vertex, timestamp));
    }

    pub fn add_edge(&mut self, from: K, to: K, timestamp: u128) {
        self.edges.insert((from, to, timestamp));
    }

    pub fn remove_vertex(&mut self, vertex: K, timestamp: u128) {
        self.vertices.remove(&(vertex, timestamp));
    }

    pub fn remove_edge(&mut self, from: K, to: K, timestamp: u128) {
        self.edges.remove(&(from, to, timestamp));
    }
}

impl<K> CmRDT for ORGraph<K>
where
    K: Eq + Hash + Clone + Ord + Serialize,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::AddVertex { vertex, timestamp } => {
                self.add_vertex(vertex, timestamp);
            }
            Operation::AddEdge {
                from,
                to,
                timestamp,
            } => {
                self.add_edge(from, to, timestamp);
            }
            Operation::RemoveVertex { vertex, timestamp } => {
                self.remove_vertex(vertex, timestamp);
            }
            Operation::RemoveEdge {
                from,
                to,
                timestamp,
            } => {
                self.remove_edge(from, to, timestamp);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for ORGraph<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn merge(&mut self, other: &Self) {
        for (k, timestamp) in &other.vertices {
            let current = self.vertices.iter().find(|(key, _)| key == k);
            match current {
                Some((_, current_timestamp)) => {
                    if current_timestamp < timestamp {
                        self.vertices.remove(&(k.clone(), *current_timestamp));
                        self.vertices.insert((k.clone(), *timestamp));
                    }
                }
                None => {
                    self.vertices.insert((k.clone(), *timestamp));
                }
            }
        }

        for (from, to, timestamp) in &other.edges {
            let current = self.edges.iter().find(|(f, t, _)| f == from && t == to);
            match current {
                Some((_, _, current_timestamp)) => {
                    if current_timestamp < timestamp {
                        self.edges
                            .remove(&(from.clone(), to.clone(), *current_timestamp));
                        self.edges.insert((from.clone(), to.clone(), *timestamp));
                    }
                }
                None => {
                    self.edges.insert((from.clone(), to.clone(), *timestamp));
                }
            }
        }
    }

    fn convert_state(&self, op: TextOperation<K>) {
        todo!()
    }
}

impl<K> Delta for ORGraph<K>
where
    K: Eq + Hash + Clone,
{
    type Value = K;

    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            vertices: self.vertices.difference(&since.vertices).cloned().collect(),
            edges: self.edges.difference(&since.edges).cloned().collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }

    fn convert_delta(&self, op: TextOperation<K>) {
        todo!()
    }
}
