use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq)]
pub struct GGraph<K>
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

impl<K> GGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    pub fn new() -> Self {
        GGraph {
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
        self.edges.insert((from.clone(), to.clone()));
    }
}

impl<K> CmRDT for GGraph<K>
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

impl<K> CvRDT for GGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    fn merge(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.clone());
        self.edges.extend(other.edges.clone());
    }
}

impl<K> Delta for GGraph<K>
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

impl<K> Semilattice<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: GGraph<K>, b: GGraph<K>, c: GGraph<K>) -> bool
    where
        GGraph<K>: CmRDT,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        for v in b.vertices.iter() {
            ab_c.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in b.edges.iter() {
            ab_c.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in c.vertices.iter() {
            bc.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in c.edges.iter() {
            bc.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in bc.vertices.iter() {
            ab_c.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in bc.edges.iter() {
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
        println!("{:?}", ab_c.value());
        println!("{:?}", a_bc.value());
        ab_c.value() == a_bc.value()
    }

    fn cmrdt_commutative(a: GGraph<K>, b: GGraph<K>) -> bool
    where
        GGraph<K>: CmRDT,
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
        println!("{:?}", ab.value());
        println!("{:?}", ba.value());
        ab.value() == ba.value()
    }

    fn cmrdt_idempotent(a: GGraph<K>) -> bool
    where
        GGraph<K>: CmRDT,
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

    fn cvrdt_associative(a: GGraph<K>, b: GGraph<K>, c: GGraph<K>) -> bool
    where
        GGraph<K>: CvRDT,
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

    fn cvrdt_commutative(a: GGraph<K>, b: GGraph<K>) -> bool
    where
        GGraph<K>: CvRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.merge(&b);
        ba.merge(&a);
        ab.value() == ba.value()
    }

    fn cvrdt_idempotent(a: GGraph<K>) -> bool
    where
        GGraph<K>: CvRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        once.merge(&a);
        twice.merge(&a);
        twice.merge(&a);
        once.value() == twice.value()
    }

    fn delta_associative(a: GGraph<K>, b: GGraph<K>, c: GGraph<K>) -> bool
    where
        GGraph<K>: Delta,
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

    fn delta_commutative(a: GGraph<K>, b: GGraph<K>) -> bool
    where
        GGraph<K>: Delta,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.apply_delta(&b);
        ba.apply_delta(&a);
        ab.value() == ba.value()
    }

    fn delta_idempotent(a: GGraph<K>) -> bool
    where
        GGraph<K>: Delta,
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
        let mut a = GGraph::new();
        let mut b = GGraph::new();
        let mut c = GGraph::new();
        a.add_vertex(1);
        a.add_vertex(2);
        a.add_edge(1, 2);
        b.add_vertex(2);
        b.add_vertex(3);
        b.add_edge(2, 3);
        c.add_vertex(3);
        c.add_vertex(4);
        c.add_edge(3, 4);
        assert!(GGraph::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        assert!(GGraph::cmrdt_commutative(a.clone(), b.clone()));
        assert!(GGraph::cmrdt_idempotent(a.clone()));
        assert!(GGraph::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        assert!(GGraph::cvrdt_commutative(a.clone(), b.clone()));
        assert!(GGraph::cvrdt_idempotent(a.clone()));
        assert!(GGraph::delta_associative(a.clone(), b.clone(), c.clone()));
        assert!(GGraph::delta_commutative(a.clone(), b.clone()));
        assert!(GGraph::delta_idempotent(a.clone()));
    }
}
