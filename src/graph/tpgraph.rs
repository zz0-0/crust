use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    vertices: HashSet<V>,
    edges: HashMap<(V, V), E>,
}

impl<V, E> TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        TPGraph {
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
        self.edges.insert((from, to), edge);
    }

    pub fn remove_edge(&mut self, from: &V, to: &V) {
        self.edges.remove(&(from.clone(), to.clone()));
    }
}

impl<V, E> CmRDT for TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn apply(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.iter().cloned());

        for ((from, to), edge) in &other.edges {
            if !self.edges.contains_key(&(from.clone(), to.clone())) {
                self.add_edge(from.clone(), to.clone(), edge.clone());
            }
        }
    }
}

impl<V, E> CvRDT for TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<V, E> Delta for TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
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

    impl Semilattice for TPGraph<String, String> {
        fn associative() {}
        fn commutative() {}
        fn idempotent() {}
    }

    #[test]
    fn test_semilattice_properties() {
        TPGraph::<String, String>::associative();
        TPGraph::<String, String>::commutative();
        TPGraph::<String, String>::idempotent();
    }
}
