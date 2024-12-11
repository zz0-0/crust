use crate::{
    crdt_prop::Semilattice,
    crdt_type::{CmRDT, CvRDT, Delta},
};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, PartialEq)]
pub struct MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    values: HashSet<(K, u64)>,
}

pub enum Operation<K> {
    Write { value: K, timestamp: u64 },
}

impl<K> MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        MVRegister {
            values: HashSet::new(),
        }
    }

    pub fn write(&mut self, value: K, timestamp: u64) {
        self.values.clear();
        self.values.insert((value, timestamp));
    }

    pub fn timestamps(&self) -> Vec<u64> {
        self.values.iter().map(|(_, ts)| *ts).collect()
    }
}

impl<K> CmRDT for MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    type Op = Operation<K>;

    fn apply(&mut self, op: Self::Op) {
        todo!();
    }
}

impl<K> CvRDT for MVRegister<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self) {
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
    }
}

impl<K> Delta for MVRegister<K>
where
    K: Eq + Hash + Clone,
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

    fn apply_delta(&mut self, other: &Self) {
        self.merge(other);
    }
}

impl<K> Semilattice<MVRegister<K>> for MVRegister<K>
where
    K: Eq + Hash + Clone + std::fmt::Debug,
    Self: CmRDT<Op = Operation<K>>,
{
    type Op = Operation<K>;

    fn cmrdt_associative(a: MVRegister<K>, b: MVRegister<K>, c: MVRegister<K>) -> bool
    where
        MVRegister<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_commutative(a: MVRegister<K>, b: MVRegister<K>) -> bool
    where
        MVRegister<K>: CmRDT,
    {
        todo!();
    }

    fn cmrdt_idempotent(a: MVRegister<K>) -> bool
    where
        MVRegister<K>: CmRDT,
    {
        todo!();
    }

    fn cvrdt_associative(a: MVRegister<K>, b: MVRegister<K>, c: MVRegister<K>) -> bool
    where
        MVRegister<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_commutative(a: MVRegister<K>, b: MVRegister<K>) -> bool
    where
        MVRegister<K>: CvRDT,
    {
        todo!();
    }

    fn cvrdt_idempotent(a: MVRegister<K>) -> bool
    where
        MVRegister<K>: CvRDT,
    {
        todo!();
    }

    fn delta_associative(a: MVRegister<K>, b: MVRegister<K>, c: MVRegister<K>) -> bool
    where
        MVRegister<K>: Delta,
    {
        todo!();
    }

    fn delta_commutative(a: MVRegister<K>, b: MVRegister<K>) -> bool
    where
        MVRegister<K>: Delta,
    {
        todo!();
    }

    fn delta_idempotent(a: MVRegister<K>) -> bool
    where
        MVRegister<K>: Delta,
    {
        todo!();
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
