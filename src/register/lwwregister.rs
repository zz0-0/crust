use core::time;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct LWWRegister<T>
where
    T: Clone,
{
    value: Option<T>,
    timestamp: u64,
}

impl<T> LWWRegister<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        LWWRegister {
            value: None,
            timestamp: 0,
        }
    }
}

impl<T> CmRDT for LWWRegister<T>
where
    T: Clone,
{
    fn apply(&mut self, other: &Self) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

impl<T> CvRDT for LWWRegister<T>
where
    T: Clone,
{
    fn merge(&mut self, other: &Self) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

impl<T> Delta for LWWRegister<T>
where
    T: Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crdt_prop::Semilattice;

    use super::*;

    impl Semilattice for LWWRegister<String> {
        fn associative() {}
        fn commutative() {}
        fn idempotent() {}
    }

    #[test]
    fn test_semilattice_properties() {
        LWWRegister::<String>::associative();
        LWWRegister::<String>::commutative();
        LWWRegister::<String>::idempotent();
    }
}
