use crust_core::crdt_data_type::DataType;
use crust_core::text_operation::TextOperation;
use k8s_openapi::api::core::v1::Pod;
use serde::Serialize;
use std::hash::Hash;
use std::marker::PhantomData;
use std::{collections::HashMap, time::Duration};
pub struct ConcurrentCRDTMetrics<K> {
    convergence_time: Duration,
    size: u128,
    throughput: u128,
    latency: u128,
    memory_usage: u128,
    cpu_usage: u128,
    network_latency: u128,
    _phantom: PhantomData<K>,
}

impl<K> ConcurrentCRDTMetrics<K>
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> serde::Deserialize<'a>,
{
    pub fn benchmark_remote_single_merge(crdt_type: String, iterations: u128) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let a2 = DataType::<K>::new(crdt_type.clone());
        for _ in 0..iterations {
            a1.merge(&a2.clone());
        }
    }
    pub fn benchmark_remote_single_apply(
        crdt_type: String,
        iterations: u128,
        text_operation: TextOperation<K>,
    ) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let ops = a1.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                a1.apply_operation(op);
            }
        }
    }
    pub fn benchmark_remote_single_apply_delta(crdt_type: String, iterations: u128) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let mut a2 = DataType::<K>::new(crdt_type.clone());
        let delta = a2.generate_delta();
        for _ in 0..iterations {
            a1.apply_delta(&delta);
        }
    }

    pub fn benchmark_remote_multiple_with_conflict_merge(crdt_type: String, iterations: u128) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let a2 = DataType::<K>::new(crdt_type.clone());
        for _ in 0..iterations {
            a1.merge(&a2.clone());
        }
    }
    pub fn benchmark_remote_multiple_with_conflict_apply(
        crdt_type: String,
        iterations: u128,
        text_operation: TextOperation<K>,
    ) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let ops = a1.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                a1.apply_operation(op);
            }
        }
    }
    pub fn benchmark_remote_multiple_with_conflict_apply_delta(
        crdt_type: String,
        iterations: u128,
    ) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let mut a2 = DataType::<K>::new(crdt_type.clone());
        let delta = a2.generate_delta();
        for _ in 0..iterations {
            a1.apply_delta(&delta);
        }
    }

    pub fn benchmark_remote_concurrent_multiple_with_conflict_merge(
        crdt_type: String,
        iterations: u128,
    ) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let a2 = DataType::<K>::new(crdt_type.clone());
        for _ in 0..iterations {
            a1.merge(&a2.clone());
        }
    }
    pub fn benchmark_remote_concurrent_multiple_with_conflict_apply(
        crdt_type: String,
        iterations: u128,
        text_operation: TextOperation<K>,
    ) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let ops = a1.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                a1.apply_operation(op);
            }
        }
    }
    pub fn benchmark_remote_concurrent_multiple_with_conflict_apply_delta(
        crdt_type: String,
        iterations: u128,
    ) {
        let mut a1 = DataType::<K>::new(crdt_type.clone());
        let mut a2 = DataType::<K>::new(crdt_type.clone());
        let delta = a2.generate_delta();
        for _ in 0..iterations {
            a1.apply_delta(&delta);
        }
    }
}
