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

pub enum Operation<K> {
    Increment { key: K },
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
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Self::Op::Increment { key } => self.increment(key),
        }
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (replica, &count) in &other.counter {
            let current_count = self.counter.entry(replica.clone()).or_insert(0);
            *current_count = (*current_count).max(count);
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
        self.merge(delta);
    }
}

impl<K> Semilattice<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();

        if let Some(k) = b.counter.keys().next() {
            ab_c.apply(Self::Op::Increment { key: k.clone() });
        }

        if let Some(k) = c.counter.keys().next() {
            bc.apply(Self::Op::Increment { key: k.clone() });
        }

        if let Some(k) = c.counter.keys().next() {
            ab_c.apply(Self::Op::Increment { key: k.clone() });
        }

        let mut a_bc = a.clone();
        if let Some(k) = bc.counter.keys().next() {
            a_bc.apply(Self::Op::Increment { key: k.clone() });
        }

        ab_c.value() == a_bc.value()
    }

    fn cmrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        if let Some(k) = b.counter.keys().next() {
            ab.apply(Self::Op::Increment { key: k.clone() });
        }
        if let Some(k) = a.counter.keys().next() {
            ba.apply(Self::Op::Increment { key: k.clone() });
        }
        ab.value() == ba.value()
    }

    fn cmrdt_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        let mut a1 = a.clone();
        if let Some(k) = a.counter.keys().next() {
            a1.apply(Operation::Increment { key: k.clone() });
        }
        a1.value() == a.value()
    }

    fn cvrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c);
        a.clone().merge(&b_c);
        a_b.value() == a.value()
    }

    fn cvrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        a.clone().merge(&b);
        b.clone().merge(&a);
        a.value() == b.value()
    }

    fn cvrdt_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        a.clone().merge(&a);
        a.value() == a.value()
    }

    fn delta_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c);
        a.clone().apply_delta(&b_c);
        a_b.value() == a.value()
    }

    fn delta_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        a.clone().apply_delta(&b);
        b.clone().apply_delta(&a);
        a.value() == b.value()
    }

    fn delta_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        a.clone().apply_delta(&a);
        a.value() == a.value()
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
