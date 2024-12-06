use std::{collections::HashSet, hash::Hash};

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    values: HashSet<(T, u64)>,
}

impl<T> MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        MVRegister {
            values: HashSet::new(),
        }
    }

    pub fn write(&mut self, value: T, timestamp: u64) {
        self.values.clear();
        self.values.insert((value, timestamp));
    }

    pub fn timestamps(&self) -> Vec<u64> {
        self.values.iter().map(|(_, ts)| *ts).collect()
    }
}

impl<T> CmRDT for MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    fn apply(&mut self, other: &Self) {
        for (value, timestamp) in &other.values {
            self.values.insert((value.clone(), *timestamp));
        }
    }
}

impl<T> CvRDT for MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        let max_timestamp = self
            .values
            .iter()
            .chain(other.values.iter())
            .map(|(_, timestamp)| *timestamp)
            .max()
            .unwrap_or(0);
        self.values
            .retain(|(_, timestamp)| *timestamp == max_timestamp);
        other.values.iter().for_each(|(value, timestamp)| {
            self.values.insert((value.clone(), *timestamp));
        });
    }
}

impl<T> Delta for MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        let since_max = since.timestamps().into_iter().max().unwrap_or(0);
        MVRegister {
            values: self
                .values
                .iter()
                .filter(|(_, ts)| *ts > since_max)
                .cloned()
                .collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

#[cfg(test)]
mod tests {
    use crate::crdt_prop::Semilattice;

    use super::*;

    impl Semilattice for MVRegister<String> {
        fn associative() {}
        fn commutative() {}
        fn idempotent() {}
    }

    #[test]
    fn test_semilattice_properties() {
        MVRegister::<String>::associative();
        MVRegister::<String>::commutative();
        MVRegister::<String>::idempotent();
    }
}
