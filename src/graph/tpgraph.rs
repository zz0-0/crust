use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
    text_operation::{
        TextOperation, TextOperationToCmRDT, TextOperationToCvRDT, TextOperationToDelta,
    },
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

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
}

impl<K> CvRDT for TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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

impl<K> TextOperationToCmRDT for TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT for TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta for TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<TPGraph<K>> for TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: TPGraph<K>, b: TPGraph<K>, c: TPGraph<K>) -> bool
    where
        TPGraph<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: TPGraph<K>, b: TPGraph<K>) -> bool
    where
        TPGraph<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: TPGraph<K>) -> bool
    where
        TPGraph<K>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: TPGraph<K>, b: TPGraph<K>, c: TPGraph<K>) -> bool
    where
        TPGraph<K>: CvRDT,
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

    fn cvrdt_commutative(a: TPGraph<K>, b: TPGraph<K>) -> bool
    where
        TPGraph<K>: CvRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.merge(&b);
        ba.merge(&a);
        ab.value() == ba.value()
    }

    fn cvrdt_idempotent(a: TPGraph<K>) -> bool
    where
        TPGraph<K>: CvRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        once.merge(&a);
        twice.merge(&a);
        twice.merge(&a);
        once.value() == twice.value()
    }

    fn delta_associative(a: TPGraph<K>, b: TPGraph<K>, c: TPGraph<K>) -> bool
    where
        TPGraph<K>: Delta,
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

    fn delta_commutative(a: TPGraph<K>, b: TPGraph<K>) -> bool
    where
        TPGraph<K>: Delta,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.apply_delta(&b);
        ba.apply_delta(&a);
        ab.value() == ba.value()
    }

    fn delta_idempotent(a: TPGraph<K>) -> bool
    where
        TPGraph<K>: Delta,
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
        // let mut a = TPGraph::new();
        // let mut b = TPGraph::new();
        // let mut c = TPGraph::new();
        // a.add_vertex(1);
        // a.add_vertex(2);
        // a.add_edge(1, 2);
        // b.add_vertex(2);
        // b.add_vertex(3);
        // b.add_edge(2, 3);
        // c.add_vertex(3);
        // c.add_vertex(4);
        // c.add_edge(3, 4);
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
