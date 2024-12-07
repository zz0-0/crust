use core::time;

use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};

#[derive(Clone)]
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
    fn apply(&mut self, other: &Self) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

impl<T> CvRDT for LWWRegister<T>
where
    T: Clone,
{
    fn merge(&mut self, other: &Self) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

impl<T> Delta for LWWRegister<T>
where
    T: Clone,
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

impl<T> Semilattice<LWWRegister<T>> for LWWRegister<T>
where
    T: Clone,
{
    fn cmrdt_associative(a: LWWRegister<T>, b: LWWRegister<T>, c: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_commutative(a: LWWRegister<T>, b: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CmRDT,
    {
        todo!()
    }

    fn cmrdt_idempotent(a: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CmRDT,
    {
        todo!()
    }

    fn cvrdt_associative(a: LWWRegister<T>, b: LWWRegister<T>, c: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_commutative(a: LWWRegister<T>, b: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CvRDT,
    {
        todo!()
    }

    fn cvrdt_idempotent(a: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: CvRDT,
    {
        todo!()
    }

    fn delta_associative(a: LWWRegister<T>, b: LWWRegister<T>, c: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: Delta,
    {
        todo!()
    }

    fn delta_commutative(a: LWWRegister<T>, b: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: Delta,
    {
        todo!()
    }

    fn delta_idempotent(a: LWWRegister<T>) -> bool
    where
        LWWRegister<T>: Delta,
    {
        todo!()
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
