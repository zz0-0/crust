use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use core::time;

#[derive(Clone, PartialEq)]
pub struct LWWRegister<T>
where
    T: Clone,
{
    value: Option<T>,
    timestamp: u64,
}

impl<T> LWWRegister<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        LWWRegister {
            value: None,
            timestamp: 0,
        }
    }
}

impl<T> CmRDT for LWWRegister<T>
where
    T: Clone,
{
    fn apply(&mut self, other: &Self) -> Self {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
        self.clone()
    }
}

impl<T> CvRDT for LWWRegister<T>
where
    T: Clone,
{
    fn merge(&mut self, other: &Self) -> Self {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
        self.clone()
    }
}

impl<T> Delta for LWWRegister<T>
where
    T: Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        todo!()
    }

    fn apply_delta(&mut self, other: &Self) -> Self {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
        self.clone()
    }
}

impl<T> Semilattice<LWWRegister<T>> for LWWRegister<T>
where
    T: Clone + PartialEq,
{
    fn cmrdt_associative(a: LWWRegister<T>, b: LWWRegister<T>, c: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CmRDT,
    {
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: LWWRegister<T>, b: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: LWWRegister<T>, b: LWWRegister<T>, c: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: LWWRegister<T>, b: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: LWWRegister<T>, b: LWWRegister<T>, c: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: LWWRegister<T>, b: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
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
