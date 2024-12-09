use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, PartialEq)]
pub struct ORGraph<V, E>
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
    type Op = Operation<V, E>;

    fn apply(&mut self, op: Self::Op) {
        todo!();
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

impl<V, E> Semilattice<ORGraph<V, E>> for ORGraph<V, E>
where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone,
    Self: CmRDT<Op = Operation<V, E>>,
{
    type Op = Operation<V, E>;

    fn cmrdt_associative(a: ORGraph<V, E>, b: ORGraph<V, E>, c: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: ORGraph<V, E>, b: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: ORGraph<V, E>, b: ORGraph<V, E>, c: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: ORGraph<V, E>, b: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: ORGraph<V, E>, b: ORGraph<V, E>, c: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: ORGraph<V, E>, b: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: ORGraph<V, E>) -> bool
    where
        ORGraph<V, E>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = ORGraph::new();
        // let mut b = ORGraph::new();
        // let mut c = ORGraph::new();
        // assert!(ORGraph::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(ORGraph::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(ORGraph::cmrdt_idempotent(a.clone()));
        // assert!(ORGraph::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        // assert!(ORGraph::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(ORGraph::cvrdt_idempotent(a.clone()));
        // assert!(ORGraph::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(ORGraph::delta_commutative(a.clone(), b.clone()));
        // assert!(ORGraph::delta_idempotent(a.clone()));
    }
}
