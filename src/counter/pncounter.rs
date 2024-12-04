use crate::crdt_type::{CmRDT, CvRDT, Delta};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    p: HashMap<K, u64>,
    n: HashMap<K, u64>,
}

impl<K> PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        PNCounter {
            p: HashMap::new(),
            n: HashMap::new(),
        }
    }

    pub fn value(&self) -> i64 {
        let p: u64 = self.p.values().sum();
        let n: u64 = self.n.values().sum();
        p as i64 - n as i64
    }

    pub fn increment(&mut self, key: K) {
        *self.p.entry(key).or_insert(0) += 1;
    }

    pub fn decrement(&mut self, key: K) {
        *self.n.entry(key).or_insert(0) += 1;
    }
}

impl<K> CmRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn apply(&mut self, other: &Self) {
        for (replica, &count) in &other.p {
            if let Some(&current) = self.p.get(replica) {
                if count > current {
                    self.p.insert(replica.clone(), count);
                }
            } else {
                self.p.insert(replica.clone(), count);
            }
        }

        for (replica, &count) in &other.n {
            if let Some(&current) = self.n.get(replica) {
                if count > current {
                    self.n.insert(replica.clone(), count);
                }
            } else {
                self.n.insert(replica.clone(), count);
            }
        }
    }
}

impl<K> CvRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (replica, &count) in &other.p {
            let current_count = *self.p.entry(replica.clone()).or_insert(0);
            self.p.insert(replica.clone(), current_count.max(count));
        }

        for (replica, &count) in &other.n {
            let current_count = *self.n.entry(replica.clone()).or_insert(0);
            self.n.insert(replica.clone(), current_count.max(count));
        }
    }
}

impl<K> Delta for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        let mut p_delta = HashMap::new();
        let mut n_delta = HashMap::new();

        for (replica, &count) in &self.p {
            let since_count = *since.p.get(replica).unwrap_or(&0);
            if count > since_count {
                p_delta.insert(replica.clone(), count - since_count);
            }
        }

        for (replica, &count) in &self.n {
            let since_count = *since.n.get(replica).unwrap_or(&0);
            if count > since_count {
                n_delta.insert(replica.clone(), count - since_count);
            }
        }

        PNCounter {
            p: p_delta,
            n: n_delta,
        }
    }

    fn apply_delta(&mut self, other: &Self) {
        self.apply(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_counter_is_zero() {
        let counter: PNCounter<String> = PNCounter::new();
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = PNCounter::new();
        counter.increment("replica1".to_string());
        assert_eq!(counter.value(), 1);
        counter.increment("replica1".to_string());
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_decrement() {
        let mut counter = PNCounter::new();
        counter.decrement("replica1".to_string());
        assert_eq!(counter.value(), -1);
        counter.decrement("replica1".to_string());
        assert_eq!(counter.value(), -2);
    }

    #[test]
    fn test_increment_and_decrement() {
        let mut counter = PNCounter::new();
        counter.increment("replica1".to_string());
        counter.decrement("replica1".to_string());
        assert_eq!(counter.value(), 0);
        counter.increment("replica2".to_string());
        counter.decrement("replica3".to_string());
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_merge() {
        let mut counter1 = PNCounter::new();
        let mut counter2 = PNCounter::new();

        counter1.increment("replica1".to_string());
        counter2.decrement("replica2".to_string());

        counter1.merge(&counter2);
        assert_eq!(counter1.value(), 0);
    }

    #[test]
    fn test_delta() {
        let mut counter = PNCounter::new();
        let empty = PNCounter::new();

        counter.increment("replica1".to_string());
        counter.decrement("replica2".to_string());

        let delta = counter.generate_delta(&empty);
        assert_eq!(delta.value(), 0);

        let mut new_counter = PNCounter::new();
        new_counter.apply_delta(&delta);
        assert_eq!(new_counter.value(), 0);
    }
}
