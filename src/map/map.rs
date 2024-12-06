use std::collections::HashMap;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct Map<K, V> {
    entries: HashMap<K, V>,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<K, V> CmRDT for Map<K, V> {
    fn apply(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V> CvRDT for Map<K, V> {
    fn merge(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V> Delta for Map<K, V> {
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::crdt_prop::Semilattice;

    use super::*;

    impl Semilattice for Map<String, String> {
        fn associative() {}
        fn commutative() {}
        fn idempotent() {}
    }

    #[test]
    fn test_semilattice_properties() {
        Map::<String, String>::associative();
        Map::<String, String>::commutative();
        Map::<String, String>::idempotent();
    }
}
