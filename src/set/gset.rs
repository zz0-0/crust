use std::collections::BTreeSet;

use crate::crdt_type::{CmRDT, CvRDT, Delta};

#[derive(Clone)]
pub struct GSet<T: Ord + Clone> {
    set: BTreeSet<T>,
}

impl<T: Ord + Clone> GSet<T> {
    pub fn new() -> Self {
        GSet {
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.set.insert(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    pub fn read(&self) -> BTreeSet<T> {
        self.set.clone()
    }
}

impl<T: Ord + Clone> CmRDT for GSet<T> {
    fn apply(&mut self, other: &Self) {
        for element in other.set.iter() {
            if !self.set.contains(element) {
                self.set.insert(element.clone());
            }
        }
    }
}

impl<T: Ord + Clone> CvRDT for GSet<T> {
    fn merge(&mut self, other: &Self) {
        self.set.extend(other.set.iter().cloned());
    }
}

impl<T: Ord + Clone> Delta for GSet<T> {
    fn generate_delta(&self, since: &Self) -> Self {
        let mut delta = GSet::new();
        for element in self.set.iter() {
            if !since.set.contains(element) {
                delta.insert(element.clone());
            }
        }
        delta
    }

    fn apply_delta(&mut self, other: &Self) {
        self.set.extend(other.set.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_set_is_empty() {
        let set: GSet<i32> = GSet::new();
        assert!(set.read().is_empty());
    }

    #[test]
    fn test_insert() {
        let mut set = GSet::new();
        set.insert(1);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
    }

    #[test]
    fn test_multiple_inserts() {
        let mut set = GSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert_eq!(set.read().len(), 3);
    }

    #[test]
    fn test_duplicate_inserts() {
        let mut set = GSet::new();
        set.insert(1);
        set.insert(1);
        assert_eq!(set.read().len(), 1);
    }

    #[test]
    fn test_merge() {
        let mut set1 = GSet::new();
        let mut set2 = GSet::new();

        set1.insert(1);
        set2.insert(2);

        set1.merge(&set2);
        assert!(set1.contains(&1));
        assert!(set1.contains(&2));
        assert_eq!(set1.read().len(), 2);
    }

    #[test]
    fn test_delta() {
        let mut set = GSet::new();
        let empty = GSet::new();

        set.insert(1);
        set.insert(2);

        let delta = set.generate_delta(&empty);
        assert_eq!(delta.read().len(), 2);

        let mut new_set = GSet::new();
        new_set.apply_delta(&delta);
        assert!(new_set.contains(&1));
        assert!(new_set.contains(&2));
    }

    #[test]
    fn test_partial_delta() {
        let mut set1 = GSet::new();
        let mut set2 = GSet::new();

        set1.insert(1);
        set1.insert(2);
        set2.insert(1);

        let delta = set1.generate_delta(&set2);
        assert_eq!(delta.read().len(), 1);
        assert!(delta.contains(&2));
    }
}
