use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, PartialEq)]
pub struct MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    values: HashSet<(T, u64)>,
}

impl<T> MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        MVRegister {
            values: HashSet::new(),
        }
    }

    pub fn write(&mut self, value: T, timestamp: u64) {
        self.values.clear();
        self.values.insert((value, timestamp));
    }

    pub fn timestamps(&self) -> Vec<u64> {
        self.values.iter().map(|(_, ts)| *ts).collect()
    }
}

impl<T> CmRDT for MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    fn apply(&mut self, other: &Self) -> Self {
        for (value, timestamp) in &other.values {
            self.values.insert((value.clone(), *timestamp));
        }
        self.clone()
    }
}

impl<T> CvRDT for MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) -> Self {
        let max_timestamp = self
            .values
            .iter()
            .chain(other.values.iter())
            .map(|(_, timestamp)| *timestamp)
            .max()
            .unwrap_or(0);
        self.values
            .retain(|(_, timestamp)| *timestamp == max_timestamp);
        other.values.iter().for_each(|(value, timestamp)| {
            self.values.insert((value.clone(), *timestamp));
        });
        self.clone()
    }
}

impl<T> Delta for MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    fn generate_delta(&self, since: &Self) -> Self {
        let since_max = since.timestamps().into_iter().max().unwrap_or(0);
        MVRegister {
            values: self
                .values
                .iter()
                .filter(|(_, ts)| *ts > since_max)
                .cloned()
                .collect(),
        }
    }

    fn apply_delta(&mut self, other: &Self) -> Self {
        self.merge(other);
        self.clone()
    }
}

impl<T> Semilattice<MVRegister<T>> for MVRegister<T>
where
    T: Eq + Hash + Clone,
{
    fn cmrdt_associative(a: MVRegister<T>, b: MVRegister<T>, c: MVRegister<T>) -> bool
    where
        MVRegister<T>: CmRDT,
    {
        let mut a_b = a.clone();
        a_b.apply(&b);
        let mut b_c = b.clone();
        b_c.apply(&c);
        a_b.apply(&c) == a.clone().apply(&b_c)
    }

    fn cmrdt_commutative(a: MVRegister<T>, b: MVRegister<T>) -> bool
    where
        MVRegister<T>: CmRDT,
    {
        a.clone().apply(&b) == b.clone().apply(&a)
    }

    fn cmrdt_idempotent(a: MVRegister<T>) -> bool
    where
        MVRegister<T>: CmRDT,
    {
        a.clone().apply(&a) == a.clone()
    }

    fn cvrdt_associative(a: MVRegister<T>, b: MVRegister<T>, c: MVRegister<T>) -> bool
    where
        MVRegister<T>: CvRDT,
    {
        let mut a_b = a.clone();
        a_b.merge(&b);
        let mut b_c = b.clone();
        b_c.merge(&c);
        a_b.merge(&c) == a.clone().merge(&b_c)
    }

    fn cvrdt_commutative(a: MVRegister<T>, b: MVRegister<T>) -> bool
    where
        MVRegister<T>: CvRDT,
    {
        a.clone().merge(&b) == b.clone().merge(&a)
    }

    fn cvrdt_idempotent(a: MVRegister<T>) -> bool
    where
        MVRegister<T>: CvRDT,
    {
        a.clone().merge(&a) == a.clone()
    }

    fn delta_associative(a: MVRegister<T>, b: MVRegister<T>, c: MVRegister<T>) -> bool
    where
        MVRegister<T>: Delta,
    {
        let mut a_b = a.clone();
        a_b.apply_delta(&b);
        let mut b_c = b.clone();
        b_c.apply_delta(&c);
        a_b.apply_delta(&c) == a.clone().apply_delta(&b_c)
    }

    fn delta_commutative(a: MVRegister<T>, b: MVRegister<T>) -> bool
    where
        MVRegister<T>: Delta,
    {
        a.clone().apply_delta(&b) == b.clone().apply_delta(&a)
    }

    fn delta_idempotent(a: MVRegister<T>) -> bool
    where
        MVRegister<T>: Delta,
    {
        a.clone().apply_delta(&a) == a.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semilattice() {
        // let mut a = MVRegister::new();
        // let mut b = MVRegister::new();
        // let mut c = MVRegister::new();
        // assert!(MVRegister::cmrdt_associative(
        //     a.clone(),
        //     b.clone(),
        //     c.clone()
        // ));
        // assert!(MVRegister::cmrdt_commutative(a.clone(), b.clone()));
        // assert!(MVRegister::cmrdt_idempotent(a.clone()));
        // assert!(MVRegister::cvrdt_associative(
        //     a.clone(),
        //     b.clone(),
        //     c.clone()
        // ));
        // assert!(MVRegister::cvrdt_commutative(a.clone(), b.clone()));
        // assert!(MVRegister::cvrdt_idempotent(a.clone()));
        // assert!(MVRegister::delta_associative(
        //     a.clone(),
        //     b.clone(),
        //     c.clone()
        // ));
        // assert!(MVRegister::delta_commutative(a.clone(), b.clone()));
        // assert!(MVRegister::delta_idempotent(a.clone()));
    }
}
