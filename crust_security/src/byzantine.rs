use crust_core::{delta::CrdtDelta, operation::CrdtOperation, r#type::CrdtType};
use serde_json::Value;
use std::collections::HashMap;
use std::hash::Hash;

pub struct ByzantineSecurity {
    
    tolerance_threshold: f64,
    max_operation_size: usize,
    max_value_size: u64,
    recent_operations: HashMap<String, Vec<OperationContext>>,
}

struct OperationContext {
    timestamp: u64,
    source_id: String,
    operation_type: String,
}

impl ByzantineSecurity {
    pub fn new() -> Self {
        ByzantineSecurity {
            tolerance_threshold: 0.67,       
            max_operation_size: 1024 * 1024, 
            max_value_size: u64::MAX / 2,    
            recent_operations: HashMap::new(),
        }
    }

    pub fn with_threshold(threshold: f64) -> Self {
        let mut security = Self::new();
        security.tolerance_threshold = threshold.max(0.5).min(1.0); 
        security
    }

    pub fn with_max_sizes(mut self, max_op_size: usize, max_val_size: u64) -> Self {
        self.max_operation_size = max_op_size;
        self.max_value_size = max_val_size;
        self
    }

    pub fn validate_state<K>(&self, state: &CrdtType<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        
        match state.name().as_str() {
            "gcounter" => self.validate_gcounter_state(state),
            "pncounter" => self.validate_pncounter_state(state),
            "lwwregister" => self.validate_lwwregister_state(state),
            "orset" => self.validate_orset_state(state),
            _ => false, 
        }
    }

    fn validate_gcounter_state<K>(&self, state: &CrdtType<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        let state_json = state.get_state();

        if let Some(counter_map) = state_json.get("state").and_then(|v| v.as_object()) {
            
            for (_key, value) in counter_map {
                if let Some(count) = value.as_u64() {
                    
                    if count > self.max_value_size {
                        return false;
                    }
                } else {
                    
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn validate_pncounter_state<K>(&self, state: &CrdtType<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        let state_json = state.get_state();

        
        if let (Some(p_map), Some(n_map)) = (
            state_json.get("p").and_then(|v| v.as_object()),
            state_json.get("n").and_then(|v| v.as_object()),
        ) {
            
            for (_key, value) in p_map {
                if let Some(count) = value.as_u64() {
                    if count > self.max_value_size {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            
            for (_key, value) in n_map {
                if let Some(count) = value.as_u64() {
                    if count > self.max_value_size {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    fn validate_lwwregister_state<K>(&self, state: &CrdtType<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        let state_json = state.get_state();

        
        if let (Some(_value), Some(timestamp)) = (
            state_json.get("value"),
            state_json.get("timestamp").and_then(|v| v.as_u64()),
        ) {
            
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            
            if timestamp > current_time + 3600 {
                return false;
            }

            true
        } else {
            false
        }
    }

    fn validate_orset_state<K>(&self, state: &CrdtType<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        let state_json = state.get_state();

        
        if let Some(elements) = state_json.get("elements").and_then(|v| v.as_object()) {
            
            for (_elem, tags) in elements {
                if let Some(tags_array) = tags.as_array() {
                    
                    if tags_array.len() > 1000 {
                        return false;
                    }

                    
                    for tag in tags_array {
                        match tag {
                            Value::String(_) => (), 
                            Value::Object(obj) => {
                                
                                if !obj.contains_key("node") || !obj.contains_key("counter") {
                                    return false;
                                }
                            }
                            _ => return false, 
                        }
                    }
                } else {
                    return false; 
                }
            }

            true
        } else {
            false
        }
    }

    pub fn validate_operation<K>(&self, operation: &CrdtOperation<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        
        let operation_size = serde_json::to_string(operation)
            .map(|s| s.len())
            .unwrap_or(usize::MAX);

        if operation_size > self.max_operation_size {
            return false;
        }

        
        match operation {
            CrdtOperation::Counter(counter_op) => {
                
                if let Some(increment) = counter_op.increment {
                    increment > 0 && increment < self.max_value_size
                } else {
                    true 
                }
            }
            CrdtOperation::Register(register_op) => {
                
                if let Some(timestamp) = register_op.timestamp {
                    
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    timestamp <= current_time + 3600 
                } else {
                    true 
                }
            }
            CrdtOperation::Set(set_op) => {
                
                match &set_op.action {
                    crust_core::operation::SetAction::Add(_) => true, 
                    crust_core::operation::SetAction::Remove(_) => true, 
                    _ => false,                                       
                }
            }
            
            _ => true, 
        }
    }

    pub fn validate_delta<K>(&self, delta: &CrdtDelta<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        
        let delta_size = serde_json::to_string(delta)
            .map(|s| s.len())
            .unwrap_or(usize::MAX);

        if delta_size > self.max_operation_size {
            return false;
        }

        
        match delta {
            CrdtDelta::GCounter(gcounter_delta) => {
                
                
                gcounter_delta
                    .increment_map
                    .values()
                    .all(|&v| v > 0 && v < self.max_value_size)
            }
            CrdtDelta::PNCounter(pncounter_delta) => {
                
                pncounter_delta.p.values().all(|&v| v < self.max_value_size)
                    && pncounter_delta.n.values().all(|&v| v < self.max_value_size)
            }
            CrdtDelta::LWWRegister(_) => {
                
                
                true
            }
            CrdtDelta::ORSet(orset_delta) => {
                
                let add_valid = orset_delta.added.values().all(|tags| tags.len() < 1000);
                let remove_valid = orset_delta.removed.values().all(|tags| tags.len() < 1000);
                add_valid && remove_valid
            }
            
            _ => true, 
        }
    }

    
    pub fn record_operation(&mut self, source_id: &str, operation_type: &str, timestamp: u64) {
        let op_context = OperationContext {
            timestamp,
            source_id: source_id.to_string(),
            operation_type: operation_type.to_string(),
        };

        self.recent_operations
            .entry(source_id.to_string())
            .or_insert_with(Vec::new)
            .push(op_context);

        
        let max_history = 1000;
        if let Some(ops) = self.recent_operations.get_mut(source_id) {
            if ops.len() > max_history {
                ops.drain(0..ops.len() - max_history);
            }
        }
    }

    
    pub fn detect_malicious_patterns(&self, source_id: &str, time_window_secs: u64) -> bool {
        if let Some(ops) = self.recent_operations.get(source_id) {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            let recent_ops_count = ops
                .iter()
                .filter(|op| current_time - op.timestamp < time_window_secs)
                .count();

            
            recent_ops_count > 100
        } else {
            false 
        }
    }
}
