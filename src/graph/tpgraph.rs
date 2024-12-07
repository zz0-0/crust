use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

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

impl<V, E> Semilattice<TPGraph<V, E>> for TPGraph<V, E>
where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone,
{
    fn cmrdt_associative(a: TPGraph<V, E>, b: TPGraph<V, E>, c: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: TPGraph<V, E>, b: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: TPGraph<V, E>, b: TPGraph<V, E>, c: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: TPGraph<V, E>, b: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: TPGraph<V, E>, b: TPGraph<V, E>, c: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: TPGraph<V, E>, b: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: Delta,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = TPGraph::new();
        // let mut b = TPGraph::new();
        // let mut c = TPGraph::new();

        // assert!(TPGraph::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(TPGraph::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(TPGraph::cmrdt_idempotent(a.clone()));
        // assert!(TPGraph::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(TPGraph::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(TPGraph::cvrdt_idempotent(a.clone()));
        // assert!(TPGraph::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(TPGraph::delta_commutative(a.clone(), b.clone()));
        // assert!(TPGraph::delta_idempotent(a.clone()));
    }
}
