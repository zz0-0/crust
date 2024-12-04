use std::collections::BTreeSet;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct RWSet<T> {
    set: BTreeSet<T>,
}

impl<T> RWSet<T> {
    pub fn new() -> Self {
        RWSet {
            set: BTreeSet::new(),
        }
    }
}

impl<T> CmRDT for RWSet<T> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for RWSet<T> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for RWSet<T> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
