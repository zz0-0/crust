use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TPGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    vertices: HashSet<K>,
    edges: HashSet<(K, K)>,
}

pub enum Operation<K> {
    AddVertex { vertex: K },
    AddEdge { from: K, to: K },
}

impl<K> TPGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    pub fn new() -> Self {
        TPGraph {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub fn value(&self) -> (Vec<K>, Vec<(K, K)>) {
        let mut vertices: Vec<K> = self.vertices.iter().cloned().collect();
        let mut edges: Vec<(K, K)> = self.edges.iter().cloned().collect();
        vertices.sort();
        edges.sort();
        (vertices, edges)
    }

    pub fn add_vertex(&mut self, vertex: K) {
        self.vertices.insert(vertex);
    }

    pub fn add_edge(&mut self, from: K, to: K) {
        if self.vertices.contains(&from.clone()) && self.vertices.contains(&to.clone()) {
            self.edges.insert((from.clone(), to.clone()));
        }
    }
}

impl<K> CmRDT for TPGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Operation::AddVertex { vertex } => {
                self.add_vertex(vertex);
            }
            Operation::AddEdge { from, to } => {
                self.add_edge(from, to);
            }
        }
    }
}

impl<K> CvRDT for TPGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    fn merge(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.clone());
        self.edges.extend(other.edges.clone());
    }
}

impl<K> Delta for TPGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!();
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!();
    }
}

impl<K> Semilattice<TPGraph<K>> for TPGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: TPGraph<K>, b: TPGraph<K>, c: TPGraph<K>) -> bool
    where
        TPGraph<K>: CmRDT,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        if let Some(k) = b.vertices.iter().next() {
            ab_c.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = b.edges.iter().next() {
            ab_c.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        if let Some(k) = c.vertices.iter().next() {
            bc.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = c.edges.iter().next() {
            bc.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        if let Some(k) = c.vertices.iter().next() {
            ab_c.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = c.edges.iter().next() {
            ab_c.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        let mut a_bc = a.clone();
        for v in bc.vertices.iter() {
            a_bc.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in bc.edges.iter() {
            a_bc.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        ab_c.value() == a_bc.value()
    }

    fn cmrdt_commutative(a: TPGraph<K>, b: TPGraph<K>) -> bool
    where
        TPGraph<K>: CmRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        if let Some(k) = b.vertices.iter().next() {
            ab.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = b.edges.iter().next() {
            ab.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        if let Some(k) = a.vertices.iter().next() {
            ba.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = a.edges.iter().next() {
            ba.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        ab.value() == ba.value()
    }

    fn cmrdt_idempotent(a: TPGraph<K>) -> bool
    where
        TPGraph<K>: CmRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        if let Some(k) = a.vertices.iter().next() {
            once.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = a.edges.iter().next() {
            once.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        if let Some(k) = a.vertices.iter().next() {
            twice.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = a.edges.iter().next() {
            twice.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        if let Some(k) = a.vertices.iter().next() {
            twice.apply(Operation::AddVertex { vertex: k.clone() });
        }
        if let Some((from, to)) = a.edges.iter().next() {
            twice.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        once.value() == twice.value()
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
        todo!();
    }

    fn delta_commutative(a: TPGraph<K>, b: TPGraph<K>) -> bool
    where
        TPGraph<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: TPGraph<K>) -> bool
    where
        TPGraph<K>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        let mut a = TPGraph::new();
        let mut b = TPGraph::new();
        let mut c = TPGraph::new();
        a.add_vertex(1);
        a.add_vertex(2);
        a.add_edge(1, 2);
        b.add_vertex(2);
        b.add_vertex(3);
        b.add_edge(2, 3);
        c.add_vertex(3);
        c.add_vertex(4);
        c.add_edge(3, 4);
        assert!(TPGraph::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        assert!(TPGraph::cmrdt_commutative(a.clone(), b.clone()));
        assert!(TPGraph::cmrdt_idempotent(a.clone()));
        assert!(TPGraph::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        assert!(TPGraph::cvrdt_commutative(a.clone(), b.clone()));
        assert!(TPGraph::cvrdt_idempotent(a.clone()));
        // assert!(TPGraph::delta_associative(a.clone(), b.clone(), c.clone()));
        // assert!(TPGraph::delta_commutative(a.clone(), b.clone()));
        // assert!(TPGraph::delta_idempotent(a.clone()));
    }
}
