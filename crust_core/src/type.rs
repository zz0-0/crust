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
    security::{self, SecurityHook},
    sync::{Crdt, DeltaBased, OperationBased, StateBased, SyncConfig, SyncMode},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CrdtTypeVariant<K>
where
    K: Eq + Hash,
{
    GCounter(GCounter<K>),
}

#[cfg(feature = "constraints")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConstraintRule<K> {
    MaxValue(u64),
    MinValue(u64),
    RangeValue(u64, u64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrdtType<K>
where
    K: Eq + Hash,
{
    pub variant: CrdtTypeVariant<K>,
    #[cfg(feature = "batch")]
    pub operations_buffer: Vec<CrdtOperation<K>>,
    #[cfg(feature = "batch")]
    pub deltas_buffer: Vec<CrdtDelta<K>>,
    #[cfg(any(
        feature = "byzantine",
        feature = "confidentiality",
        feature = "integrity",
        feature = "access_control"
    ))]
    #[serde(skip)]
    security: Option<Box<dyn SecurityHook<K> + Send + Sync>>,
    #[cfg(feature = "constraints")]
    pub constraints: Option<Vec<ConstraintRule<K>>>,
    #[cfg(feature = "reversible")]
    pub operation_history: Vec<(CrdtOperation<K>, i64)>,
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
                #[cfg(feature = "batch")]
                operations_buffer: Vec::new(),
                #[cfg(feature = "batch")]
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

    #[cfg(any(
        feature = "byzantine",
        feature = "confidentiality",
        feature = "integrity",
        feature = "access_control"
    ))]
    pub fn with_security(mut self, security: Box<dyn SecurityHook<K>>) -> Self {
        self.security = Some(security);
        self
    }

    #[cfg(any(
        feature = "byzantine",
        feature = "confidentiality",
        feature = "integrity",
        feature = "access_control"
    ))]
    fn get_security(&self) -> &dyn SecurityHook<K> {
        match &self.security {
            Some(sec) => sec.as_ref(),
            None => &NoSecurity(PhantomData),
        }
    }

    #[cfg(feature = "constraints")]
    pub fn check_constraints(&self, command: &CrdtInnerCommand<K>) -> bool {
        match &self.variant {
            CrdtTypeVariant::GCounter(gcounter) => match command {
                CrdtInnerCommand::Counter(CounterInnerCommand::Increment { value }) => {
                    gcounter.check_constraints(value)
                }
            },
        }
    }

    #[cfg(feature = "constraints")]
    pub fn set_constraint_rules(&mut self) -> Result<(), String> {
        let rules = match &self.variant {
            CrdtTypeVariant::GCounter(_) => {
                vec![ConstraintRule::MaxValue(1000), ConstraintRule::MinValue(0)]
            }
        };

        for rule in &rules {
            if !self.is_rule_valid_for_type(rule) {
                return Err(format!(
                    "Rule {:?} is not valid for CRDT type {}",
                    rule,
                    self.name()
                ));
            }
        }

        self.constraints = Some(rules);
        Ok(())
    }

    #[cfg(feature = "constraints")]
    fn is_rule_valid_for_type(&self, rule: &ConstraintRule<K>) -> bool {
        match &self.variant {
            CrdtTypeVariant::GCounter(_) => match rule {
                ConstraintRule::MaxValue(_) => true,
                ConstraintRule::MinValue(_) => true,
                ConstraintRule::RangeValue(_, _) => true,
            },
        }
    }

    #[cfg(feature = "constraints")]
    pub fn get_constraint_rules(&self) -> Option<&Vec<ConstraintRule<K>>> {
        self.constraints.as_ref()
    }

    #[cfg(feature = "reversible")]
    pub fn compute_inverse_operation(&self, operation: &CrdtOperation<K>) -> CrdtOperation<K> {
        match &self.variant {
            CrdtTypeVariant::GCounter(gcounter) => match operation {
                CrdtOperation::Counter(CounterOperation::Increment { value }) => {
                    CrdtOperation::Counter(CounterOperation::Decrement {
                        value: value.clone(),
                    })
                }
            },
        }
    }

    #[cfg(feature = "reversible")]
    pub fn revert_operation(&mut self, operation_id: usize) -> Result<(), String> {
        if operation_id >= self.operation_history.len() {
            return Err("Invalid operation ID".to_string());
        }

        let (operation, _) = &self.operation_history[operation_id];
        if let Some(inverse) = self.compute_inverse_operation(operation) {
            self.apply(&inverse);
            Ok(())
        } else {
            Err("Operation cannot be reversed".to_string())
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
        #[cfg(any(
            feature = "byzantine",
            feature = "confidentiality",
            feature = "integrity",
            feature = "access_control"
        ))]
        let security = self.get_security();

        #[cfg(feature = "access_control")]
        security.check_access(&self);

        match (&mut self.variant, &other.variant) {
            (CrdtTypeVariant::GCounter(gcounter1), CrdtTypeVariant::GCounter(gcounter2)) => {
                #[cfg(feature = "byzantine")]
                security.validate_state(&self);
                let _ = gcounter1.merge(&gcounter2);
                #[cfg(feature = "access_control")]
                security.audit_log(&self);
            }
        }
    }

    pub fn apply(&mut self, operation: &CrdtOperation<K>) {
        #[cfg(any(
            feature = "byzantine",
            feature = "confidentiality",
            feature = "integrity",
            feature = "access_control"
        ))]
        let security = self.get_security();

        #[cfg(feature = "access_control")]
        security.check_access(&self);

        match &mut self.variant {
            CrdtTypeVariant::GCounter(gcounter) => {
                #[cfg(feature = "byzantine")]
                security.validate_operation(&self);

                if let CrdtOperation::Counter(op) = operation {
                    let _ = gcounter.apply(op);
                    #[cfg(feature = "reversible")]
                    self.operation_history
                        .push((operation.clone(), self.get_unix_timestamp_seconds()));
                }

                #[cfg(feature = "access_control")]
                security.audit_log(&self);
            }
        }
    }

    pub fn merge_delta(&mut self, delta: &CrdtDelta<K>) {
        #[cfg(any(
            feature = "byzantine",
            feature = "confidentiality",
            feature = "integrity",
            feature = "access_control"
        ))]
        let security = self.get_security();

        #[cfg(feature = "access_control")]
        security.check_access(&self);

        match &mut self.variant {
            CrdtTypeVariant::GCounter(gcounter) => {
                #[cfg(feature = "byzantine")]
                security.validate_delta(&self);

                let CrdtDelta::GCounter(delta) = delta;
                let _ = gcounter.merge_delta(delta);

                #[cfg(feature = "access_control")]
                security.audit_log(&self);
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

        #[cfg(feature = "constraints")]
        if !self.check_constraints(command) {
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

    #[cfg(feature = "batch")]
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

    #[cfg(feature = "batch")]
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

    #[cfg(feature = "batch")]
    fn get_unix_timestamp_seconds(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    #[cfg(feature = "batch")]
    fn elapsed_duration_since_timestamp(&self, timestamp_seconds: i64) -> Duration {
        let timestamp_system_time = UNIX_EPOCH + Duration::from_secs(timestamp_seconds as u64);
        SystemTime::now()
            .duration_since(timestamp_system_time)
            .unwrap_or(Duration::ZERO) 
    }

    #[cfg(feature = "batch")]
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
                    config.last_batch_check_timestamp = Some(self.get_unix_timestamp_seconds()); 
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

    #[cfg(feature = "batch")]
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

    #[cfg(feature = "batch")]
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

    #[cfg(feature = "batch")]
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
                    config.last_batch_check_timestamp = Some(self.get_unix_timestamp_seconds()); 
                    return None;
                }
            }
        }
        None
    }
}
