use std::collections::BTreeSet;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct AWSet<T> {
    set: BTreeSet<T>,
}

impl<T> AWSet<T> {
    pub fn new() -> Self {
        AWSet {
            set: BTreeSet::new(),
        }
    }
}

impl<T> CmRDT for AWSet<T> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for AWSet<T> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for AWSet<T> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
