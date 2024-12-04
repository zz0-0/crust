use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct LWWRegister<T> {
    value: Option<T>,
    timestamp: u64,
}

impl<K> LWWRegister<K> {
    pub fn new() -> Self {
        LWWRegister {
            value: None,
            timestamp: 0,
        }
    }
}

impl<K> CmRDT for LWWRegister<K> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> CvRDT for LWWRegister<K> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K> Delta for LWWRegister<K> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}
