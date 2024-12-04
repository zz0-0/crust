use std::collections::BTreeSet;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct ORSet<T> {
    set: BTreeSet<T>,
}

impl<T> ORSet<T> {
    pub fn new() -> Self {
        ORSet {
            set: BTreeSet::new(),
        }
    }
}

impl<T> CmRDT for ORSet<T> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for ORSet<T> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for ORSet<T> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
