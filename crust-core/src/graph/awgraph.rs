use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AWGraph<K>
where
    K: Eq + Hash,
{
    vertices: HashSet<K>,
    edges: HashSet<(K, K)>,
}

pub enum Operation<K> {
    AddVertex { vertex: K },
    AddEdge { from: K, to: K },
    RemoveVertex { vertex: K },
    RemoveEdge { from: K, to: K },
}

impl<K> AWGraph<K>
where
    K: Eq + Hash + Clone + Ord + Serialize + for<'a> Deserialize<'a>,
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

    pub fn to_crdt(str: String) -> Self {
        serde_json::from_str(&str).unwrap()
    }

    pub fn value(&self) -> (Vec<K>, Vec<(K, K)>) {
        let mut vertices: Vec<K> = self.vertices.iter().cloned().collect();
        let mut edges: Vec<(K, K)> = self.edges.iter().cloned().collect();
        vertices.sort();
        edges.sort();
        (vertices, edges)
    }

    pub fn add_vertex(&mut self, vertex: K) {
        self.vertices.insert(vertex);
    }

    pub fn add_edge(&mut self, from: K, to: K) {
        self.edges.insert((from, to));
    }

    pub fn remove_vertex(&mut self, vertex: K) {
        self.vertices.remove(&vertex);
    }

    pub fn remove_edge(&mut self, from: K, to: K) {
        self.edges.remove(&(from, to));
    }
}

impl<K> CmRDT for AWGraph<K>
where
    K: Eq + Hash + Clone + Ord + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::AddVertex { vertex } => {
                self.add_vertex(vertex);
            }
            Operation::AddEdge { from, to } => {
                self.add_edge(from, to);
            }
            Operation::RemoveVertex { vertex } => {
                self.remove_vertex(vertex);
            }
            Operation::RemoveEdge { from, to } => {
                self.remove_edge(from, to);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for AWGraph<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.clone());
        self.edges.extend(other.edges.clone());
    }
}

impl<K> Delta for AWGraph<K>
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
