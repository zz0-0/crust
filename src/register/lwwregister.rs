use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use core::time;

#[derive(Debug, Clone, PartialEq)]
pub struct LWWRegister<K>
where
    K: Clone,
{
    value: Option<K>,
    timestamp: u64,
}

pub enum Operation<K> {
    Set(K, u64),
}

impl<K> LWWRegister<K>
where
    K: Clone,
{
    pub fn new() -> Self {
        LWWRegister {
            value: None,
            timestamp: 0,
        }
    }
}

impl<K> CmRDT for LWWRegister<K>
where
    K: Clone,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        todo!();
    }
}

impl<K> CvRDT for LWWRegister<K>
where
    K: Clone,
{
    fn merge(&mut self, other: &Self) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

impl<K> Delta for LWWRegister<K>
where
    K: Clone,
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

impl<K> Semilattice<LWWRegister<K>> for LWWRegister<K>
where
    K: Clone + PartialEq,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: LWWRegister<K>, b: LWWRegister<K>, c: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: LWWRegister<K>, b: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: LWWRegister<K>, b: LWWRegister<K>, c: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: LWWRegister<K>, b: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: LWWRegister<K>, b: LWWRegister<K>, c: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: LWWRegister<K>, b: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: LWWRegister<K>) -> bool
    where
        LWWRegister<K>: Delta,
    {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        let mut a = LWWRegister::new();
        let mut b = LWWRegister::new();
        let mut c = LWWRegister::new();
        a.timestamp = 1;
        a.value = Some(1);
        b.timestamp = 2;
        b.value = Some(2);
        c.timestamp = 3;
        c.value = Some(3);
        assert_eq!(
            LWWRegister::cmrdt_associative(a.clone(), b.clone(), c.clone()),
            true
        );
        assert_eq!(LWWRegister::cmrdt_commutative(a.clone(), b.clone()), true);
        assert_eq!(LWWRegister::cmrdt_idempotent(a.clone()), true);
        assert_eq!(
            LWWRegister::cvrdt_associative(a.clone(), b.clone(), c.clone()),
            true
        );
        assert_eq!(LWWRegister::cvrdt_commutative(a.clone(), b.clone()), true);
        assert_eq!(LWWRegister::cvrdt_idempotent(a.clone()), true);
        assert_eq!(
            LWWRegister::delta_associative(a.clone(), b.clone(), c.clone()),
            true
        );
        assert_eq!(LWWRegister::delta_commutative(a.clone(), b.clone()), true);
        assert_eq!(LWWRegister::delta_idempotent(a.clone()), true);
    }
}
