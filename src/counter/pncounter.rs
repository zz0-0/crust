use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{collections::HashMap, hash::Hash};
#[derive(Clone, PartialEq)]
pub struct PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    p: HashMap<K, u64>,
    n: HashMap<K, u64>,
}

pub enum Operation<K> {
    Increment { key: K },
    Decrement { key: K },
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
    type Op = Operation<K>;
    fn apply(&mut self, op: Self::Op) {
        todo!();
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
        self.merge(other);
    }
}

impl<K> Semilattice<PNCounter<K>> for PNCounter<K>
where
    K: Eq + Hash + Clone,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: PNCounter<K>, b: PNCounter<K>, c: PNCounter<K>) -> bool
    where
        PNCounter<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: PNCounter<K>, b: PNCounter<K>) -> bool
    where
        PNCounter<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: PNCounter<K>) -> bool
    where
        PNCounter<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: PNCounter<K>, b: PNCounter<K>, c: PNCounter<K>) -> bool
    where
        PNCounter<K>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c);
        a.clone().merge(&b_c);
        a_b.value() == a.value()
    }

    fn cvrdt_commutative(a: PNCounter<K>, b: PNCounter<K>) -> bool
    where
        PNCounter<K>: CvRDT,
    {
        a.clone().merge(&b);
        b.clone().merge(&a);
        a.value() == b.value()
    }

    fn cvrdt_idempotent(a: PNCounter<K>) -> bool
    where
        PNCounter<K>: CvRDT,
    {
        a.clone().merge(&a);
        a.value() == a.value()
    }

    fn delta_associative(a: PNCounter<K>, b: PNCounter<K>, c: PNCounter<K>) -> bool
    where
        PNCounter<K>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c);
        a.clone().apply_delta(&b_c);
        a_b.value() == a.value()
    }

    fn delta_commutative(a: PNCounter<K>, b: PNCounter<K>) -> bool
    where
        PNCounter<K>: Delta,
    {
        a.clone().apply_delta(&b);
        b.clone().apply_delta(&a);
        a.value() == b.value()
    }

    fn delta_idempotent(a: PNCounter<K>) -> bool
    where
        PNCounter<K>: Delta,
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
        let mut a = PNCounter::new();
        let mut b = PNCounter::new();
        let mut c = PNCounter::new();
        assert!(PNCounter::<i32>::cmrdt_associative(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(PNCounter::<i32>::cmrdt_commutative(a.clone(), b.clone()));
        assert!(PNCounter::<i32>::cmrdt_idempotent(a.clone()));
        assert!(PNCounter::<i32>::cvrdt_associative(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(PNCounter::<i32>::cvrdt_commutative(a.clone(), b.clone()));
        assert!(PNCounter::<i32>::cvrdt_idempotent(a.clone()));
        assert!(PNCounter::<i32>::delta_associative(
            a.clone(),
            b.clone(),
            c.clone()
        ));
        assert!(PNCounter::<i32>::delta_commutative(a.clone(), b.clone()));
        assert!(PNCounter::<i32>::delta_idempotent(a.clone()));
    }
}
