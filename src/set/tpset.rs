use std::collections::BTreeSet;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct TPSet<T> {
    set: BTreeSet<T>,
}

impl<T> TPSet<T> {
    pub fn new() -> Self {
        TPSet {
            set: BTreeSet::new(),
        }
    }
}

impl<T> CmRDT for TPSet<T> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> CvRDT for TPSet<T> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<T> Delta for TPSet<T> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
