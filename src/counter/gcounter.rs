use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct GCounter<K>
where
    K: Eq + Hash + Clone,
{
    counter: HashMap<K, u64>,
}

pub enum Operation<K> {
    Increment { key: K, value: u64 },
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
            Self::Op::Increment { key, value } => {
                let current_count = self.counter.entry(key.clone()).or_insert(0);
                *current_count = (*current_count).max(value);
            }
        }
    }
}

impl<K> CvRDT for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.counter {
            let current_count = self.counter.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
    }
}

impl<K> Delta for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!();
    }

    fn apply_delta(&mut self, delta: &Self) {
        self.merge(delta);
    }
}

impl<K> Semilattice<GCounter<K>> for GCounter<K>
where
    K: Eq + Hash + Clone + std::fmt::Debug,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        if let Some((k, v)) = b.counter.iter().next() {
            ab_c.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = c.counter.iter().next() {
            bc.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = c.counter.iter().next() {
            ab_c.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        let mut a_bc = a.clone();
        for (k, v) in bc.counter.iter() {
            a_bc.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        ab_c.value() == a_bc.value()
    }

    fn cmrdt_commutative(a: GCounter<K>, b: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        if let Some((k, v)) = b.counter.iter().next() {
            ab.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.counter.iter().next() {
            ba.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        ab.value() == ba.value()
    }

    fn cmrdt_idempotent(a: GCounter<K>) -> bool
    where
        GCounter<K>: CmRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        if let Some((k, v)) = a.counter.iter().next() {
            once.apply(Operation::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.counter.iter().next() {
            twice.apply(Operation::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.counter.iter().next() {
            twice.apply(Operation::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        once.value() == twice.value()
    }

    fn cvrdt_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: CvRDT,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        ab_c.merge(&b);
        bc.merge(&c);
        ab_c.merge(&c);
        let mut a_bc = a.clone();
        a_bc.merge(&bc);
        ab_c.value() == a_bc.value()
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
        let mut once = a.clone();
        let mut twice = a.clone();
        once.merge(&a);
        twice.merge(&a);
        twice.merge(&a);
        once.value() == twice.value()
    }

    fn delta_associative(a: GCounter<K>, b: GCounter<K>, c: GCounter<K>) -> bool
    where
        GCounter<K>: Delta,
    {
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        ab_c.apply_delta(&b);
        bc.apply_delta(&c);
        ab_c.apply_delta(&c);
        let mut a_bc = a.clone();
        a_bc.apply_delta(&bc);
        ab_c.value() == a_bc.value()
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
        let mut once = a.clone();
        let mut twice = a.clone();
        once.apply_delta(&a);
        twice.apply_delta(&a);
        twice.apply_delta(&a);
        once.value() == twice.value()
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
