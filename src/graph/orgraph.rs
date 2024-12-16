use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ORGraph<K>
where
    K: Hash + Eq + Clone + Ord,
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
    K: Hash + Eq + Clone + Ord,
{
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
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
    K: Hash + Eq + Clone + Ord,
{
    type Op = Operation<K>;

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
}

impl<K> CvRDT for ORGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
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
}

impl<K> Delta for ORGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            vertices: self.vertices.difference(&since.vertices).cloned().collect(),
            edges: self.edges.difference(&since.edges).cloned().collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> Semilattice<ORGraph<K>> for ORGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: ORGraph<K>, b: ORGraph<K>, c: ORGraph<K>) -> bool
    where
        ORGraph<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: ORGraph<K>, b: ORGraph<K>) -> bool
    where
        ORGraph<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: ORGraph<K>) -> bool
    where
        ORGraph<K>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: ORGraph<K>, b: ORGraph<K>, c: ORGraph<K>) -> bool
    where
        ORGraph<K>: CvRDT,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        ab_c.merge(&b);
        bc.merge(&c);
        ab_c.merge(&c);
        let mut a_bc = a.clone();
        a_bc.merge(&bc);
        ab_c.value() == a_bc.value()
    }

    fn cvrdt_commutative(a: ORGraph<K>, b: ORGraph<K>) -> bool
    where
        ORGraph<K>: CvRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.merge(&b);
        ba.merge(&a);
        ab.value() == ba.value()
    }

    fn cvrdt_idempotent(a: ORGraph<K>) -> bool
    where
        ORGraph<K>: CvRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        once.merge(&a);
        twice.merge(&a);
        twice.merge(&a);
        once.value() == twice.value()
    }

    fn delta_associative(a: ORGraph<K>, b: ORGraph<K>, c: ORGraph<K>) -> bool
    where
        ORGraph<K>: Delta,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        ab_c.apply_delta(&b);
        bc.apply_delta(&c);
        ab_c.apply_delta(&c);
        let mut a_bc = a.clone();
        a_bc.apply_delta(&bc);
        ab_c.value() == a_bc.value()
    }

    fn delta_commutative(a: ORGraph<K>, b: ORGraph<K>) -> bool
    where
        ORGraph<K>: Delta,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.apply_delta(&b);
        ba.apply_delta(&a);
        ab.value() == ba.value()
    }

    fn delta_idempotent(a: ORGraph<K>) -> bool
    where
        ORGraph<K>: Delta,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        once.apply_delta(&a);
        twice.apply_delta(&a);
        twice.apply_delta(&a);
        once.value() == twice.value()
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
        // a.add_vertex(1);
        // a.add_vertex(2);
        // a.add_edge(1, 2);
        // b.add_vertex(2);
        // b.add_vertex(3);
        // b.add_edge(2, 3);
        // c.add_vertex(3);
        // c.add_vertex(4);
        // c.add_edge(3, 4);
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
