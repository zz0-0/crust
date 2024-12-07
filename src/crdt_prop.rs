use crate::crdt_type::{CmRDT, CvRDT, Delta};

pub trait Semilattice<T>
where
    T: CmRDT + CvRDT + Delta,
{
    fn cmrdt_associative(a: T, b: T, c: T) -> bool
    where
        T: CmRDT;
    fn cmrdt_commutative(a: T, b: T) -> bool
    where
        T: CmRDT;
    fn cmrdt_idempotent(a: T) -> bool
    where
        T: CmRDT;
    fn cvrdt_associative(a: T, b: T, c: T) -> bool
    where
        T: CvRDT;
    fn cvrdt_commutative(a: T, b: T) -> bool
    where
        T: CvRDT;
    fn cvrdt_idempotent(a: T) -> bool
    where
        T: CvRDT;
    fn delta_associative(a: T, b: T, c: T) -> bool
    where
        T: Delta;
    fn delta_commutative(a: T, b: T) -> bool
    where
        T: Delta;
    fn delta_idempotent(a: T) -> bool
    where
        T: Delta;
}
