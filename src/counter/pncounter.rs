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
    Increment { key: K, value: u64 },
    Decrement { key: K, value: u64 },
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
        match op {
            Self::Op::Increment { key, value } => {
                let current_count = self.p.entry(key.clone()).or_insert(0);
                *current_count = (*current_count).max(value);
            }
            Self::Op::Decrement { key, value } => {
                let current_count = self.n.entry(key.clone()).or_insert(0);
                *current_count = (*current_count).max(value);
            }
        }
    }
}

impl<K> CvRDT for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
        for (k, v) in &other.p {
            let current_count = self.p.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
        for (k, v) in &other.n {
            let current_count = self.n.entry(k.clone()).or_insert(0);
            *current_count = (*current_count).max(*v);
        }
    }
}

impl<K> Delta for PNCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!();
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
        let mut ab_c = a.clone();
        let mut bc = b.clone();
        if let Some((k, v)) = b.p.iter().next() {
            ab_c.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = b.n.iter().next() {
            ab_c.apply(Self::Op::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = c.p.iter().next() {
            bc.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = c.n.iter().next() {
            bc.apply(Self::Op::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = c.p.iter().next() {
            ab_c.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = c.n.iter().next() {
            ab_c.apply(Self::Op::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        let mut a_bc = a.clone();
        for (k, v) in bc.p.iter() {
            a_bc.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        for (k, v) in bc.n.iter() {
            a_bc.apply(Self::Op::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        ab_c.value() == a_bc.value()
    }

    fn cmrdt_commutative(a: PNCounter<K>, b: PNCounter<K>) -> bool
    where
        PNCounter<K>: CmRDT,
    {
        let mut ab = a.clone();
        let mut ba = b.clone();
        if let Some((k, v)) = b.p.iter().next() {
            ab.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = b.n.iter().next() {
            ab.apply(Self::Op::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.p.iter().next() {
            ba.apply(Self::Op::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.n.iter().next() {
            ba.apply(Self::Op::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        ab.value() == ba.value()
    }

    fn cmrdt_idempotent(a: PNCounter<K>) -> bool
    where
        PNCounter<K>: CmRDT,
    {
        let mut once = a.clone();
        let mut twice = a.clone();
        if let Some((k, v)) = a.p.iter().next() {
            once.apply(Operation::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.n.iter().next() {
            once.apply(Operation::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.p.iter().next() {
            twice.apply(Operation::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.p.iter().next() {
            twice.apply(Operation::Increment {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.n.iter().next() {
            twice.apply(Operation::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        if let Some((k, v)) = a.n.iter().next() {
            twice.apply(Operation::Decrement {
                key: k.clone(),
                value: *v,
            });
        }
        once.value() == twice.value()
    }

    fn cvrdt_associative(a: PNCounter<K>, b: PNCounter<K>, c: PNCounter<K>) -> bool
    where
        PNCounter<K>: CvRDT,
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
        let mut once = a.clone();
        let mut twice = a.clone();
        once.merge(&a);
        twice.merge(&a);
        twice.merge(&a);
        once.value() == twice.value()
    }

    fn delta_associative(a: PNCounter<K>, b: PNCounter<K>, c: PNCounter<K>) -> bool
    where
        PNCounter<K>: Delta,
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
        let a = PNCounter::new();
        let b = PNCounter::new();
        let c = PNCounter::new();
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
