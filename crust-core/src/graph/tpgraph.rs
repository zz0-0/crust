use crate::{
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::TextOperation,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TPGraph<K>
where
    K: Eq + Hash,
{
    vertices: HashSet<(K, bool)>,
    edges: HashSet<(K, K, bool)>,
}

pub enum Operation<K> {
    AddVertex { vertex: K, tombstone: bool },
    AddEdge { from: K, to: K, tombstone: bool },
    RemoveVertex { vertex: K, tombstone: bool },
    RemoveEdge { from: K, to: K, tombstone: bool },
}

impl<K> TPGraph<K>
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

    pub fn value(&self) -> (Vec<(K, bool)>, Vec<(K, K, bool)>) {
        let mut vertices: Vec<(K, bool)> = self.vertices.iter().cloned().collect();
        let mut edges: Vec<(K, K, bool)> = self.edges.iter().cloned().collect();
        vertices.sort();
        edges.sort();
        (vertices, edges)
    }

    pub fn add_vertex(&mut self, vertex: K, tombstone: bool) {
        self.vertices.insert((vertex, tombstone));
    }

    pub fn add_edge(&mut self, from: K, to: K, tombstone: bool) {
        self.edges.insert((from, to, tombstone));
    }

    pub fn remove_vertex(&mut self, vertex: K, tombstone: bool) {
        self.vertices.remove(&(vertex, tombstone));
    }

    pub fn remove_edge(&mut self, from: K, to: K, tombstone: bool) {
        self.edges.remove(&(from, to, tombstone));
    }
}

impl<K> CmRDT for TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + Serialize + for<'a> Deserialize<'a>,
{
    type Op = Operation<K>;
    type Value = K;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::AddVertex { vertex, tombstone } => {
                self.add_vertex(vertex, tombstone);
            }
            Operation::AddEdge {
                from,
                to,
                tombstone,
            } => {
                self.add_edge(from, to, tombstone);
            }
            Operation::RemoveVertex { vertex, tombstone } => {
                self.remove_vertex(vertex, tombstone);
            }
            Operation::RemoveEdge {
                from,
                to,
                tombstone,
            } => {
                self.remove_edge(from, to, tombstone);
            }
        }
    }

    fn convert_operation(&self, op: TextOperation<K>) -> Vec<Self::Op> {
        todo!()
    }
}

impl<K> CvRDT for TPGraph<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (k, tombstone) in &other.vertices {
            let current = self.vertices.iter().find(|(key, _)| key == k);
            match current {
                Some((_, current_tombstone)) => {
                    if !current_tombstone && *tombstone {
                        self.vertices.remove(&(k.clone(), *current_tombstone));
                        self.vertices.insert((k.clone(), *tombstone));
                    }
                }
                None => {
                    self.vertices.insert((k.clone(), *tombstone));
                }
            }
        }
    }
}

impl<K> Delta for TPGraph<K>
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
