use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
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
    fn apply(&mut self, other: &Self) {
        self.vertices.extend(other.vertices.iter().cloned());

        for ((from, to), edge) in &other.edges {
            if !self.edges.contains_key(&(from.clone(), to.clone())) {
                self.add_edge(from.clone(), to.clone(), edge.clone());
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph_is_empty() {
        let graph: GGraph<i32, String> = GGraph::new();
        assert!(graph.vertices.is_empty());
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_add_vertex() {
        let mut graph: GGraph<i32, String> = GGraph::new();
        graph.add_vertex(1);
        assert!(graph.vertices.contains(&1));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = GGraph::new();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_edge(1, 2, "edge".to_string());
        assert!(graph.edges.contains_key(&(1, 2)));
    }

    #[test]
    fn test_add_edge_without_vertices() {
        let mut graph = GGraph::new();
        graph.add_edge(1, 2, "edge".to_string());
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_merge() {
        let mut graph1 = GGraph::new();
        let mut graph2 = GGraph::new();

        graph1.add_vertex(1);
        graph1.add_vertex(2);
        graph1.add_edge(1, 2, "edge1".to_string());

        graph2.add_vertex(2);
        graph2.add_vertex(3);
        graph2.add_edge(2, 3, "edge2".to_string());

        graph1.merge(&graph2);
        assert!(graph1.vertices.contains(&1));
        assert!(graph1.vertices.contains(&2));
        assert!(graph1.vertices.contains(&3));
        assert!(graph1.edges.contains_key(&(1, 2)));
        assert!(graph1.edges.contains_key(&(2, 3)));
    }

    #[test]
    fn test_delta() {
        let mut graph = GGraph::new();
        let empty = GGraph::new();

        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_edge(1, 2, "edge".to_string());

        let delta = graph.generate_delta(&empty);
        assert_eq!(delta.vertices.len(), 2);
        assert_eq!(delta.edges.len(), 1);

        let mut new_graph = GGraph::new();
        new_graph.apply_delta(&delta);
        assert!(new_graph.vertices.contains(&1));
        assert!(new_graph.vertices.contains(&2));
        assert!(new_graph.edges.contains_key(&(1, 2)));
    }

    #[test]
    fn test_partial_delta() {
        let mut graph1 = GGraph::new();
        let mut graph2 = GGraph::new();

        graph1.add_vertex(1);
        graph1.add_vertex(2);
        graph1.add_edge(1, 2, "edge".to_string());

        graph2.add_vertex(1);

        let delta = graph1.generate_delta(&graph2);
        assert_eq!(delta.vertices.len(), 1);
        assert_eq!(delta.edges.len(), 1);
        assert!(delta.vertices.contains(&2));
        assert!(delta.edges.contains_key(&(1, 2)));
    }
}
