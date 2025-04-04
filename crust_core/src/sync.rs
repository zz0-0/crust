use std::time::Duration;

use crate::command::CrdtInnerCommand;

pub trait Crdt {
    type State;
    fn new() -> Self::State;
    fn get_state(&self) -> Self::State;
    fn name() -> String;
}
pub trait StateBased: Crdt {
    fn merge(&mut self, other: &Self::State) -> Self::State;
}
pub trait OperationBased: Crdt {
    type Op;
    fn apply(&mut self, op: &Self::Op) -> Self::State;
    fn aggregate_operations(&mut self, operations: Vec<Self::Op>) -> Option<Self::Op>;
}
pub trait DeltaBased: Crdt {
    type Delta;
    fn generate_delta(&self) -> Self::Delta;
    fn merge_delta(&mut self, other: &Self::Delta) -> Self::State;
    fn aggregate_deltas(&mut self, deltas: Vec<Self::Delta>) -> Option<Self::Delta>;
}

pub trait ConstraintEnforcing<K>: Crdt {
    fn check_constraints(&self, command: &CrdtInnerCommand<K>) -> bool;
    fn repair_constraints(&mut self) -> Self::State;
}

#[derive(Clone, Copy)]
pub enum SyncType {
    Delta,
    Operation,
    State,
}

impl SyncType {
    pub fn new(name: String) -> Self {
        match name.as_str() {
            "delta" => SyncType::Delta,
            "operation" => SyncType::Operation,
            "state" => SyncType::State,
            _ => panic!("Unknown sync type"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SyncType::Delta => "delta".to_string(),
            SyncType::Operation => "operation".to_string(),
            SyncType::State => "state".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum SyncMode {
    Immediate,
    #[cfg(feature = "batch")]
    BatchTimeBased,
    #[cfg(feature = "batch")]
    BatchCountBased,
}

impl SyncMode {
    pub fn new(name: String) -> Self {
        match name.as_str() {
            "immediate" => SyncMode::Immediate,
            #[cfg(feature = "batch")]
            "batch_time_based" => SyncMode::BatchTimeBased,
            #[cfg(feature = "batch")]
            "batch_count_based" => SyncMode::BatchCountBased,
            _ => panic!("Unknown sync mode"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SyncMode::Immediate => "immediate".to_string(),
            #[cfg(feature = "batch")]
            SyncMode::BatchTimeBased => "batch_time_based".to_string(),
            #[cfg(feature = "batch")]
            SyncMode::BatchCountBased => "batch_count_based".to_string(),
        }
    }
}


pub struct SyncConfig {
    pub sync_type: SyncType,
    pub sync_mode: SyncMode,
    #[cfg(feature = "batch")]
    pub batch_times: Option<usize>,
    #[cfg(feature = "batch")]
    pub batching_interval: Option<Duration>,
    #[cfg(feature = "batch")]
    pub last_batch_check_timestamp: Option<i64>,
}
