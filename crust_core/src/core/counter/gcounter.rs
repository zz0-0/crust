use std::collections::HashMap;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::{
    operation::CounterOperation,
    sync::{Crdt, DeltaBased, OperationBased, StateBased},
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GCounter<K>
where
    K: Eq + Hash,
{
    pub counter: HashMap<K, u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GCounterDelta<K>
where
    K: Eq + Hash,
{
    pub increment_map: HashMap<K, u64>,
}

impl<K> GCounter<K>
where
    K: Eq + Hash,
{
    pub fn increment(&mut self, key: K) {
        let current_value = self.counter.entry(key).or_insert(0);
        *current_value += 1;
    }
}

impl<K> Crdt for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type State = GCounter<K>;

    fn new() -> Self::State {
        GCounter {
            counter: HashMap::new(),
        }
    }

    fn get_state(&self) -> Self::State {
        self.clone()
    }

    fn name() -> String {
        "gcounter".to_string()
    }
}

impl<K> StateBased for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    fn merge(&mut self, other: &Self::State) -> Self::State {
        for (key, value) in &other.counter {
            let current_value = self.counter.entry(key.clone()).or_insert(0);
            *current_value = (*current_value).max(*value);
        }
        self.clone()
    }
}

impl<K> OperationBased for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Op = CounterOperation<K>;
    fn apply(&mut self, op: &Self::Op) -> Self::State {
        match op {
            CounterOperation::Increment { value } => {}
            CounterOperation::Decrement { value } => {}
        }
        self.clone()
    }

    fn aggregate_operations(&mut self, operations: Vec<Self::Op>) -> Option<Self::Op> {
        todo!()
    }
}

impl<K> DeltaBased for GCounter<K>
where
    K: Eq + Hash + Clone,
{
    type Delta = GCounterDelta<K>;
    fn generate_delta(&self) -> Self::Delta {
        unimplemented!()
    }
    fn merge_delta(&mut self, other: &Self::Delta) -> Self::State {
        unimplemented!()
    }

    fn aggregate_deltas(&mut self, deltas: Vec<Self::Delta>) -> Option<Self::Delta> {
        todo!()
    }
}
