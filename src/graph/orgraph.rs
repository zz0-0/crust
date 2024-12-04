use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    vertices: HashSet<V>,
    edges: HashMap<(V, V), E>,
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
}

impl<V, E> CmRDT for ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<V, E> CvRDT for ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<V, E> Delta for ORGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
