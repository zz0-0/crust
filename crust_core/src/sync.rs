use std::time::Duration;

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
    BatchTimeBased,
    BatchCountBased,
}

impl SyncMode {
    pub fn new(name: String) -> Self {
        match name.as_str() {
            "immediate" => SyncMode::Immediate,
            "batch_time_based" => SyncMode::BatchTimeBased,
            "batch_count_based" => SyncMode::BatchCountBased,
            _ => panic!("Unknown sync mode"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SyncMode::Immediate => "immediate".to_string(),
            SyncMode::BatchTimeBased => "batch_time_based".to_string(),
            SyncMode::BatchCountBased => "batch_count_based".to_string(),
        }
    }
}

// #[derive(Clone, Debug)]
pub struct SyncConfig {
    pub sync_type: SyncType,
    pub sync_mode: SyncMode,
    pub batch_times: Option<usize>,
    pub batching_interval: Option<Duration>,
    pub last_batch_check_timestamp: Option<i64>,
}
