use std::collections::HashMap;
use std::hash::Hash;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

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

impl<K> Semilattice<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn cmrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_semilattice() {
        let mut a = GCounter::new();
        let mut b = GCounter::new();
        let mut c = GCounter::new();

        a.increment("r1".to_string());
        b.increment("r2".to_string());
        c.increment("r3".to_string());

        assert!(GCounter::<String>::cmrdt_associative(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(GCounter::<String>::cmrdt_commutative(a.clone(), b.clone()));
        assert!(GCounter::<String>::cmrdt_idempotent(a.clone()));
        assert!(GCounter::<String>::cvrdt_associative(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(GCounter::<String>::cvrdt_commutative(a.clone(), b.clone()));
        assert!(GCounter::<String>::cvrdt_idempotent(a.clone()));
        assert!(GCounter::<String>::delta_associative(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(GCounter::<String>::delta_commutative(a.clone(), b.clone()));
        assert!(GCounter::<String>::delta_idempotent(a.clone()));
    }
}
