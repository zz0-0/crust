use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    vertices: HashSet<V>,
    edges: HashMap<(V, V), E>,
}

impl<V, E> ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        ORGraph {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: V) {
        self.vertices.insert(vertex);
    }

    pub fn remove_vertex(&mut self, vertex: &V) {
        self.vertices.remove(vertex);
    }

    pub fn add_edge(&mut self, from: V, to: V, edge: E) {
        if self.vertices.contains(&from) && self.vertices.contains(&to) {
            self.edges.insert((from, to), edge);
        }
    }

    pub fn remove_edge(&mut self, from: &V, to: &V) {
        self.edges.remove(&(from.clone(), to.clone()));
    }
}

impl<V, E> CmRDT for ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn apply(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.iter().cloned());

        for ((from, to), edge) in &other.edges {
            if self.vertices.contains(from) && self.vertices.contains(to) {
                self.edges.insert((from.clone(), to.clone()), edge.clone());
            }
        }
    }
}

impl<V, E> CvRDT for ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.iter().cloned());

        for ((from, to), edge) in &other.edges {
            if self.vertices.contains(from) && self.vertices.contains(to) {
                self.edges.insert((from.clone(), to.clone()), edge.clone());
            }
        }
    }
}

impl<V, E> Delta for ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        let mut vertices = HashSet::new();
        let mut edges = HashMap::new();

        for vertex in &self.vertices {
            if !since.vertices.contains(vertex) {
                vertices.insert(vertex.clone());
            }
        }

        for ((from, to), edge) in &self.edges {
            if !since.edges.contains_key(&(from.clone(), to.clone())) {
                edges.insert((from.clone(), to.clone()), edge.clone());
            }
        }

        ORGraph { vertices, edges }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.iter().cloned());
        self.edges.extend(
            other
                .edges
                .iter()
                .map(|((f, t), e)| ((f.clone(), t.clone()), e.clone())),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::crdt_prop::Semilattice;

    use super::*;

    impl Semilattice for ORGraph<String, String> {
        fn associative() {}
        fn commutative() {}
        fn idempotent() {}
    }

    #[test]
    fn test_semilattice_properties() {
        ORGraph::<String, String>::associative();
        ORGraph::<String, String>::commutative();
        ORGraph::<String, String>::idempotent();
    }
}
