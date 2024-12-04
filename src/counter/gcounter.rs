use std::collections::HashMap;
use std::hash::Hash;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct GCounter<K>
where
    K: Eq + Hash + Clone,
{
    counter: HashMap<K, u64>,
}

impl<K> GCounter<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        GCounter {
            counter: HashMap::new(),
        }
    }

    pub fn value(&self) -> u64 {
        self.counter.values().sum()
    }

    pub fn increment(&mut self, key: K) {
        *self.counter.entry(key).or_insert(0) += 1;
    }
}

impl<K> CmRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn apply(&mut self, other: &Self) {
        for (replica, &count) in &other.counter {
            let current_count = *self.counter.entry(replica.clone()).or_insert(0);
            self.counter
                .insert(replica.clone(), current_count.max(count));
        }
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (replica, &count) in &other.counter {
            let current_count = *self.counter.entry(replica.clone()).or_insert(0);
            self.counter
                .insert(replica.clone(), current_count.max(count));
        }
    }
}

impl<K> Delta for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        let mut delta = GCounter::new();
        for (replica, &count) in &self.counter {
            let since_count = *since.counter.get(replica).unwrap_or(&0);
            if count > since_count {
                delta.counter.insert(replica.clone(), count - since_count);
            }
        }
        delta
    }

    fn apply_delta(&mut self, delta: &Self) {
        self.apply(delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_counter_is_zero() {
        let counter: GCounter<String> = GCounter::new();
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = GCounter::new();
        counter.increment("replica1".to_string());
        assert_eq!(counter.value(), 1);
        counter.increment("replica1".to_string());
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_merge() {
        let mut counter1 = GCounter::new();
        let mut counter2 = GCounter::new();

        counter1.increment("replica1".to_string());
        counter2.increment("replica2".to_string());

        counter1.merge(&counter2);
        assert_eq!(counter1.value(), 2);
    }

    #[test]
    fn test_delta() {
        let mut counter = GCounter::new();
        let empty = GCounter::new();

        counter.increment("replica1".to_string());
        counter.increment("replica1".to_string());

        let delta = counter.generate_delta(&empty);
        assert_eq!(delta.value(), 2);

        let mut new_counter = GCounter::new();
        new_counter.apply_delta(&delta);
        assert_eq!(new_counter.value(), 2);
    }
}
