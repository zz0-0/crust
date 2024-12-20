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
pub struct GGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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

impl<K> CmRDT for GGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn merge(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.clone());
        self.edges.extend(other.edges.clone());
    }
}

impl<K> Delta for GGraph<K>
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

impl<K> TextOperationToCmRDT<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    type Op = Operation<K>;

    fn convert_operation(&self, op: TextOperation) -> Vec<<Self as CmRDT>::Op> {
        todo!()
    }
}

impl<K> TextOperationToCvRDT<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> TextOperationToDelta<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
{
    fn convert_operation(&self, op: TextOperation) {
        todo!()
    }
}

impl<K> Semilattice<GGraph<K>> for GGraph<K>
where
    K: Eq + Hash + Clone + Ord + std::fmt::Debug + Serialize,
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
        ab_c.value() == a_bc.value()
    }

    fn cmrdt_commutative(a: GGraph<K>, b: GGraph<K>) -> bool
    where
        GGraph<K>: CmRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        for v in b.vertices.iter() {
            ab.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in b.edges.iter() {
            ab.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.vertices.iter() {
            ba.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.edges.iter() {
            ba.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        ab.value() == ba.value()
    }

    fn cmrdt_idempotent(a: GGraph<K>) -> bool
    where
        GGraph<K>: CmRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();

        for v in a.vertices.iter() {
            once.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.edges.iter() {
            once.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.vertices.iter() {
            twice.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.edges.iter() {
            twice.apply(Operation::AddEdge {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for v in a.vertices.iter() {
            twice.apply(Operation::AddVertex { vertex: v.clone() });
        }
        for (from, to) in a.edges.iter() {
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
        // let mut a = GGraph::new();
        // let mut b = GGraph::new();
        // let mut c = GGraph::new();
        // a.add_vertex(1);
        // a.add_vertex(2);
        // a.add_edge(1, 2);
        // b.add_vertex(2);
        // b.add_vertex(3);
        // b.add_edge(2, 3);
        // c.add_vertex(3);
        // c.add_vertex(4);
        // c.add_edge(3, 4);
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
