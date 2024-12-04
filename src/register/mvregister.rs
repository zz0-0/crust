use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct MVRegister<T> {
    value: Option<T>,
    timestamp: u64,
}

impl<K> MVRegister<K> {
    pub fn new() -> Self {
        MVRegister {
            value: None,
            timestamp: 0,
        }
    }
}

impl<K> CmRDT for MVRegister<K> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> CvRDT for MVRegister<K> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for MVRegister<K> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
