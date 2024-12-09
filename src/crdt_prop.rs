use crate::crdt_type::{CmRDT, CvRDT, Delta};
pub trait Semilattice<K>
where
    K: CmRDT + CvRDT + Delta,
{
    type Op;
    fn cmrdt_associative(a: K, b: K, c: K) -> bool
    where
        K: CmRDT;
    fn cmrdt_commutative(a: K, b: K) -> bool
    where
        K: CmRDT;
    fn cmrdt_idempotent(a: K) -> bool
    where
        K: CmRDT;
    fn cvrdt_associative(a: K, b: K, c: K) -> bool
    where
        K: CvRDT;
    fn cvrdt_commutative(a: K, b: K) -> bool
    where
        K: CvRDT;
    fn cvrdt_idempotent(a: K) -> bool
    where
        K: CvRDT;
    fn delta_associative(a: K, b: K, c: K) -> bool
    where
        K: Delta;
    fn delta_commutative(a: K, b: K) -> bool
    where
        K: Delta;
    fn delta_idempotent(a: K) -> bool
    where
        K: Delta;
}
