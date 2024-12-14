use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq)]
pub struct AWGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    added_vertices: HashSet<K>,
    removed_vertices: HashSet<K>,
    added_edges: HashSet<(K, K)>,
    removed_edges: HashSet<(K, K)>,
}

pub enum Operation<K> {
    AddVertex { vertex: K },
    AddEdge { from: K, to: K },
    RemoveVertex { vertex: K },
    RemoveEdge { from: K, to: K },
}

impl<K> AWGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    pub fn new() -> Self {
        AWGraph {
            added_vertices: HashSet::new(),
            removed_vertices: HashSet::new(),
            added_edges: HashSet::new(),
            removed_edges: HashSet::new(),
        }
    }

    pub fn value(&self) -> (Vec<K>, Vec<(K, K)>, Vec<K>, Vec<(K, K)>) {
        let mut added_vertices: Vec<K> = self.added_vertices.iter().cloned().collect();
        let mut removed_vertices: Vec<K> = self.removed_vertices.iter().cloned().collect();
        let mut added_edges: Vec<(K, K)> = self.added_edges.iter().cloned().collect();
        let mut removed_edges: Vec<(K, K)> = self.removed_edges.iter().cloned().collect();
        added_vertices.sort();
        removed_vertices.sort();
        added_edges.sort();
        removed_edges.sort();
        (added_vertices, added_edges, removed_vertices, removed_edges)
    }

    pub fn add_vertex(&mut self, vertex: K) {
        self.added_vertices.insert(vertex);
    }

    pub fn add_edge(&mut self, from: K, to: K) {
        if self.added_vertices.contains(&from.clone()) && self.added_vertices.contains(&to.clone())
        {
            self.added_edges.insert((from.clone(), to.clone()));
        }
    }

    pub fn remove_vertex(&mut self, vertex: K) {
        self.removed_vertices.insert(vertex);
    }

    pub fn remove_edge(&mut self, from: K, to: K) {
        self.removed_edges.insert((from, to));
    }
}

impl<K> CmRDT for AWGraph<K>
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
            Operation::RemoveVertex { vertex } => {
                self.remove_vertex(vertex);
            }
            Operation::RemoveEdge { from, to } => {
                self.remove_edge(from, to);
            }
        }
    }
}

impl<K> CvRDT for AWGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    fn merge(&mut self, other: &Self) {
        self.added_vertices.extend(other.added_vertices.clone());
        self.removed_vertices.extend(other.removed_vertices.clone());
        self.added_edges.extend(other.added_edges.clone());
        self.removed_edges.extend(other.removed_edges.clone());
        self.added_vertices
            .retain(|v| !self.removed_vertices.contains(v));
        self.added_edges
            .retain(|(from, to)| !self.removed_edges.contains(&(from.clone(), to.clone())));
    }
}

impl<K> Delta for AWGraph<K>
where
    K: Hash + Eq + Clone + Ord,
{
    fn generate_delta(&self, since: &Self) -> Self {
        Self {
            added_vertices: self
                .added_vertices
                .difference(&since.added_vertices)
                .cloned()
                .collect(),
            removed_vertices: self
                .removed_vertices
                .difference(&since.removed_vertices)
                .cloned()
                .collect(),
            added_edges: self
                .added_edges
                .difference(&since.added_edges)
                .cloned()
                .collect(),
            removed_edges: self
                .removed_edges
                .difference(&since.removed_edges)
                .cloned()
                .collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> Semilattice<AWGraph<K>> for AWGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: AWGraph<K>, b: AWGraph<K>, c: AWGraph<K>) -> bool
    where
        AWGraph<K>: CmRDT,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        for v in b.added_vertices.iter() {
            ab_c.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in b.added_edges.iter() {
            ab_c.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in b.removed_vertices.iter() {
            ab_c.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in b.removed_edges.iter() {
            ab_c.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in c.added_vertices.iter() {
            bc.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in c.added_edges.iter() {
            bc.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in c.removed_vertices.iter() {
            bc.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in c.removed_edges.iter() {
            bc.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in bc.added_vertices.iter() {
            ab_c.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in bc.added_edges.iter() {
            ab_c.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in bc.removed_vertices.iter() {
            ab_c.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in bc.removed_edges.iter() {
            ab_c.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        let mut a_bc = a.clone();
        for v in bc.added_vertices.iter() {
            a_bc.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in bc.added_edges.iter() {
            a_bc.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in bc.removed_vertices.iter() {
            a_bc.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in bc.removed_edges.iter() {
            a_bc.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        ab_c.value() == a_bc.value()
    }

    fn cmrdt_commutative(a: AWGraph<K>, b: AWGraph<K>) -> bool
    where
        AWGraph<K>: CmRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        for v in b.added_vertices.iter() {
            ab.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in b.added_edges.iter() {
            ab.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in b.removed_vertices.iter() {
            ab.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in b.removed_edges.iter() {
            ab.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.added_vertices.iter() {
            ba.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.added_edges.iter() {
            ba.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.removed_vertices.iter() {
            ba.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in a.removed_edges.iter() {
            ba.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        ab.value() == ba.value()
    }

    fn cmrdt_idempotent(a: AWGraph<K>) -> bool
    where
        AWGraph<K>: CmRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();

        for v in a.added_vertices.iter() {
            once.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.added_edges.iter() {
            once.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.removed_vertices.iter() {
            once.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in a.removed_edges.iter() {
            once.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.added_vertices.iter() {
            twice.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.added_edges.iter() {
            twice.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.removed_vertices.iter() {
            twice.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in a.removed_edges.iter() {
            twice.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.added_vertices.iter() {
            twice.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.added_edges.iter() {
            twice.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.removed_vertices.iter() {
            twice.apply(Operation::RemoveVertex { vertex: v.clone() });
        }
        for (from, to) in a.removed_edges.iter() {
            twice.apply(Operation::RemoveEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        once.value() == twice.value()
    }

    fn cvrdt_associative(a: AWGraph<K>, b: AWGraph<K>, c: AWGraph<K>) -> bool
    where
        AWGraph<K>: CvRDT,
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

    fn cvrdt_commutative(a: AWGraph<K>, b: AWGraph<K>) -> bool
    where
        AWGraph<K>: CvRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.merge(&b);
        ba.merge(&a);
        ab.value() == ba.value()
    }

    fn cvrdt_idempotent(a: AWGraph<K>) -> bool
    where
        AWGraph<K>: CvRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        once.merge(&a);
        twice.merge(&a);
        twice.merge(&a);
        once.value() == twice.value()
    }

    fn delta_associative(a: AWGraph<K>, b: AWGraph<K>, c: AWGraph<K>) -> bool
    where
        AWGraph<K>: Delta,
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

    fn delta_commutative(a: AWGraph<K>, b: AWGraph<K>) -> bool
    where
        AWGraph<K>: Delta,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        ab.apply_delta(&b);
        ba.apply_delta(&a);
        ab.value() == ba.value()
    }

    fn delta_idempotent(a: AWGraph<K>) -> bool
    where
        AWGraph<K>: Delta,
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
        let mut a = AWGraph::new();
        let mut b = AWGraph::new();
        let mut c = AWGraph::new();
        a.add_vertex(1);
        a.add_vertex(2);
        a.add_edge(1, 2);
        b.add_vertex(2);
        b.add_vertex(3);
        b.add_edge(2, 3);
        c.add_vertex(3);
        c.add_vertex(4);
        c.add_edge(3, 4);
        assert!(AWGraph::cmrdt_associative(a.clone(), b.clone(), c.clone()));
        assert!(AWGraph::cmrdt_commutative(a.clone(), b.clone()));
        assert!(AWGraph::cmrdt_idempotent(a.clone()));
        assert!(AWGraph::cvrdt_associative(a.clone(), b.clone(), c.clone()));
        assert!(AWGraph::cvrdt_commutative(a.clone(), b.clone()));
        assert!(AWGraph::cvrdt_idempotent(a.clone()));
        assert!(AWGraph::delta_associative(a.clone(), b.clone(), c.clone()));
        assert!(AWGraph::delta_commutative(a.clone(), b.clone()));
        assert!(AWGraph::delta_idempotent(a.clone()));
    }
}
