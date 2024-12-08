use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, PartialEq)]
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
    fn apply(&mut self, other: &Self) -> Self {
        for (replica, &count) in &other.counter {
            let current_count = *self.counter.entry(replica.clone()).or_insert(0);
            self.counter
                .insert(replica.clone(), current_count.max(count));
        }
        self.clone()
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) -> Self {
        for (replica, &count) in &other.counter {
            let current_count = *self.counter.entry(replica.clone()).or_insert(0);
            self.counter
                .insert(replica.clone(), current_count.max(count));
        }
        self.clone()
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
    fn apply_delta(&mut self, delta: &Self) -> Self {
        self.apply(delta);
        self.clone()
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
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
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
