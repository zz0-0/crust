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

pub enum Operation<V, E> {
    AddVertex { vertex: V },
    RemoveVertex { vertex: V },
    AddEdge { from: V, to: V, edge: E },
    RemoveEdge { from: V, to: V },
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
    type Op = Operation<V, E>;

    fn apply(&mut self, op: Self::Op) {
        todo!();
    }
}

impl<V, E> CvRDT for GGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn merge(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.iter().cloned());
        self.edges.extend(
            other
                .edges
                .iter()
                .map(|((f, t), e)| ((f.clone(), t.clone()), e.clone())),
        );
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

impl<V, E> Semilattice<GGraph<V, E>> for GGraph<V, E>
where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone,
    Self: CmRDT<Op = Operation<V, E>>,
{
    type Op = Operation<V, E>;

    fn cmrdt_associative(a: GGraph<V, E>, b: GGraph<V, E>, c: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: GGraph<V, E>, b: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: GGraph<V, E>, b: GGraph<V, E>, c: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: GGraph<V, E>, b: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: GGraph<V, E>, b: GGraph<V, E>, c: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: GGraph<V, E>, b: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: GGraph<V, E>) -> bool
    where
        GGraph<V, E>: Delta,
    {
        todo!();
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
