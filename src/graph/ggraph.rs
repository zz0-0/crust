use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, PartialEq)]
pub struct GGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    vertices: HashSet<V>,
    edges: HashMap<(V, V), E>,
}

impl<V, E> GGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        GGraph {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: V) {
        self.vertices.insert(vertex);
    }

    pub fn add_edge(&mut self, from: V, to: V, edge: E) {
        if self.vertices.contains(&from) && self.vertices.contains(&to) {
            self.edges.insert((from, to), edge);
        }
    }
}

impl<V, E> CmRDT for GGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn apply(&mut self, other: &Self) -> Self {
        self.vertices.extend(other.vertices.iter().cloned());
        for ((from, to), edge) in &other.edges {
            if !self.edges.contains_key(&(from.clone(), to.clone())) {
                self.add_edge(from.clone(), to.clone(), edge.clone());
            }
        }
        self.clone()
    }
}

impl<V, E> CvRDT for GGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn merge(&mut self, other: &Self) -> Self {
        self.vertices.extend(other.vertices.iter().cloned());
        self.edges.extend(
            other
                .edges
                .iter()
                .map(|((f, t), e)| ((f.clone(), t.clone()), e.clone())),
        );
        self.clone()
    }
}

impl<V, E> Delta for GGraph<V, E>
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
        GGraph { vertices, edges }
    }

    fn apply_delta(&mut self, other: &Self) -> Self {
        self.vertices.extend(other.vertices.iter().cloned());
        self.edges.extend(
            other
                .edges
                .iter()
                .map(|((f, t), e)| ((f.clone(), t.clone()), e.clone())),
        );
        self.clone()
    }
}

impl<V, E> Semilattice<GGraph<V, E>> for GGraph<V, E>
where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone,
{
    fn cmrdt_associative(a: GGraph<V, E>, b: GGraph<V, E>, c: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CmRDT,
    {
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: GGraph<V, E>, b: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: GGraph<V, E>, b: GGraph<V, E>, c: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: GGraph<V, E>, b: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: GGraph<V, E>, b: GGraph<V, E>, c: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: GGraph<V, E>, b: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = GGraph::new();
        // let mut b = GGraph::new();
        // let mut c = GGraph::new();
        // assert!(GGraph::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GGraph::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(GGraph::cmrdt_idempotent(a.clone()));
        // assert!(GGraph::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GGraph::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(GGraph::cvrdt_idempotent(a.clone()));
        // assert!(GGraph::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(GGraph::delta_commutative(a.clone(), b.clone()));
        // assert!(GGraph::delta_idempotent(a.clone()));
    }
}
