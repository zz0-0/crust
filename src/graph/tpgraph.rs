use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
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
}

impl<V, E> CmRDT for TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<V, E> CvRDT for TPGraph<V, E>
where
    V: Hash + Eq + Clone,
    E: Hash + Eq + Clone,
{
    fn merge(&mut self, other: &Self) {
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

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
