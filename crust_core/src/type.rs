use std::{
    collections::HashMap,
    hash::Hash,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{de::value, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    command::{CounterInnerCommand, CrdtInnerCommand},
    core::counter::gcounter::GCounter,
    delta::CrdtDelta,
    operation::{CounterOperation, CrdtOperation},
    sync::{Crdt, DeltaBased, OperationBased, StateBased, SyncConfig, SyncMode},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CrdtTypeVariant<K>
where
    K: Eq + Hash,
{
    GCounter(GCounter<K>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrdtType<K>
where
    K: Eq + Hash,
{
    pub variant: CrdtTypeVariant<K>,
    pub operations_buffer: Vec<CrdtOperation<K>>,
    pub deltas_buffer: Vec<CrdtDelta<K>>,
}

impl<K> CrdtType<K>
where
    CrdtType<K>: Clone,
    K: Eq + Hash + Clone + Serialize + for<'a> Deserialize<'a>,
{
    pub fn new(name: String) -> Option<Self> {
        match name.as_str() {
            "gcounter" => Some(CrdtType {
                variant: CrdtTypeVariant::GCounter(GCounter::new()),
                operations_buffer: Vec::new(),
                deltas_buffer: Vec::new(),
            }),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match &self.variant {
            CrdtTypeVariant::GCounter(_) => "gcounter".to_string(),
        }
    }

    pub fn get_state(&self) -> Value {
        match self.variant {
            CrdtTypeVariant::GCounter(ref gcounter) => {
                let counter_state = gcounter.get_state();
                json!({
                    "value": counter_state.counter.values().sum::<u64>().to_string(),
                    "state": counter_state.counter.iter()
                        .map(|(k, v)| (k.clone(), *v))
                        .collect::<HashMap<K, u64>>()
                })
            }
        }
    }

    pub fn merge(&mut self, other: &Self) {
        match (&mut self.variant, &other.variant) {
            (CrdtTypeVariant::GCounter(gcounter1), CrdtTypeVariant::GCounter(gcounter2)) => {
                let _ = gcounter1.merge(&gcounter2);
            }
        }
    }

    pub fn apply(&mut self, operation: &CrdtOperation<K>) {
        match &mut self.variant {
            CrdtTypeVariant::GCounter(gcounter) => {
                if let CrdtOperation::Counter(op) = operation {
                    let _ = gcounter.apply(op);
                }
            }
        }
    }

    pub fn merge_delta(&mut self, delta: &CrdtDelta<K>) {
        match &mut self.variant {
            CrdtTypeVariant::GCounter(gcounter) => {
                let CrdtDelta::GCounter(delta) = delta;
                let _ = gcounter.merge_delta(delta);
            }
        }
    }

    pub fn validate_command(&self, value: K) -> Vec<CrdtInnerCommand<K>> {
        match &self.variant {
            CrdtTypeVariant::GCounter(_) => {
                vec![CrdtInnerCommand::Counter(CounterInnerCommand::Increment {
                    value: value,
                })]
            }
        }
    }

    pub fn is_command_valid(&self, command: &CrdtInnerCommand<K>) -> bool {
        match (&self.variant, command) {
            (CrdtTypeVariant::GCounter(_), CrdtInnerCommand::Counter(_)) => true,
            _ => false,
        }
    }

    pub fn apply_command(&mut self, command: &CrdtInnerCommand<K>) -> Option<CrdtOperation<K>> {
        if !self.is_command_valid(command) {
            return None;
        }

        match (&mut self.variant, command) {
            (
                CrdtTypeVariant::GCounter(gcounter),
                CrdtInnerCommand::Counter(CounterInnerCommand::Increment { value }),
            ) => {
                gcounter.increment(value.clone());
                Some(CrdtOperation::Counter(CounterOperation::Increment {
                    value: value.clone(),
                }))
            }
            _ => None,
        }
    }

    fn generate_operation_helper(&mut self) -> Option<CrdtOperation<K>> {
        match &mut self.variant {
            CrdtTypeVariant::GCounter(gcounter) => {
                let operations = self
                    .operations_buffer
                    .iter()
                    .filter_map(|crdt_op| {
                        if let CrdtOperation::Counter(counter_op) = crdt_op {
                            Some(counter_op.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                let aggregate_operations = gcounter.aggregate_operations(operations);
                if let Some(aggregate_operation) = aggregate_operations {
                    self.operations_buffer.clear();
                    Some(CrdtOperation::Counter(aggregate_operation))
                } else {
                    None
                }
            }
        }
    }

    pub fn generate_operation_count_based(
        &mut self,
        config: &SyncConfig,
    ) -> Option<CrdtOperation<K>> {
        if let (SyncMode::BatchCountBased, Some(batch_times)) =
            (&config.sync_mode, config.batch_times)
        {
            if self.operations_buffer.len() >= batch_times {
                return self.generate_operation_helper();
            }
        }
        None
    }

    fn get_unix_timestamp_seconds(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    fn elapsed_duration_since_timestamp(&self, timestamp_seconds: i64) -> Duration {
        let timestamp_system_time = UNIX_EPOCH + Duration::from_secs(timestamp_seconds as u64);
        SystemTime::now()
            .duration_since(timestamp_system_time)
            .unwrap_or(Duration::ZERO) // Handle potential errors
    }

    pub fn generate_operation_time_based(
        &mut self,
        config: &mut SyncConfig,
    ) -> Option<CrdtOperation<K>> {
        if let SyncMode::BatchTimeBased = config.sync_mode {
            if let Some(batching_interval) = config.batching_interval {
                if let Some(last_check) = config.last_batch_check_timestamp {
                    let elapsed_duration = self.elapsed_duration_since_timestamp(last_check);
                    if elapsed_duration >= batching_interval {
                        config.last_batch_check_timestamp = Some(self.get_unix_timestamp_seconds());
                        return self.generate_operation_helper();
                    } else {
                        return None;
                    }
                } else {
                    config.last_batch_check_timestamp = Some(self.get_unix_timestamp_seconds()); // Initialize if None
                    return None;
                }
            }
        }
        None
    }

    pub fn generate_delta(&self) -> CrdtDelta<K> {
        match &self.variant {
            CrdtTypeVariant::GCounter(gcounter) => CrdtDelta::GCounter(gcounter.generate_delta()),
        }
    }

    fn generate_delta_helper(&mut self) -> Option<CrdtDelta<K>> {
        match &mut self.variant {
            CrdtTypeVariant::GCounter(gcounter) => {
                let deltas = self
                    .deltas_buffer
                    .iter()
                    .filter_map(|crdt_delta| {
                        let CrdtDelta::GCounter(gcounter_delta) = crdt_delta;
                        Some(gcounter_delta.clone())
                    })
                    .collect();
                let aggregate_deltas = gcounter.aggregate_deltas(deltas);
                if let Some(aggregate_delta) = aggregate_deltas {
                    self.deltas_buffer.clear();
                    Some(CrdtDelta::GCounter(aggregate_delta))
                } else {
                    None
                }
            }
        }
    }

    pub fn generate_delta_count_based(&mut self, config: &SyncConfig) -> Option<CrdtDelta<K>> {
        if let (SyncMode::BatchCountBased, Some(batch_times)) =
            (&config.sync_mode, config.batch_times)
        {
            if self.operations_buffer.len() >= batch_times {
                return self.generate_delta_helper();
            }
        }
        None
    }

    pub fn generate_delta_time_based(&mut self, config: &mut SyncConfig) -> Option<CrdtDelta<K>> {
        if let SyncMode::BatchTimeBased = config.sync_mode {
            if let Some(batching_interval) = config.batching_interval {
                if let Some(last_check) = config.last_batch_check_timestamp {
                    let elapsed_duration = self.elapsed_duration_since_timestamp(last_check);
                    if elapsed_duration >= batching_interval {
                        config.last_batch_check_timestamp = Some(self.get_unix_timestamp_seconds());
                        return self.generate_delta_helper();
                    } else {
                        return None;
                    }
                } else {
                    config.last_batch_check_timestamp = Some(self.get_unix_timestamp_seconds()); // Initialize if None
                    return None;
                }
            }
        }
        None
    }
}
