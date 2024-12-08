use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, PartialEq)]
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

impl<V, E> CvRDT for TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn merge(&mut self, other: &Self) -> Self {
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

impl<V, E> Semilattice<TPGraph<V, E>> for TPGraph<V, E>
where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone,
{
    fn cmrdt_associative(a: TPGraph<V, E>, b: TPGraph<V, E>, c: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CmRDT,
    {
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: TPGraph<V, E>, b: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: TPGraph<V, E>, b: TPGraph<V, E>, c: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: TPGraph<V, E>, b: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: TPGraph<V, E>, b: TPGraph<V, E>, c: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: TPGraph<V, E>, b: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: TPGraph<V, E>) -> bool
    where
        TPGraph<V, E>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
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
