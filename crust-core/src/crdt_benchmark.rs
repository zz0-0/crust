use std::time::Instant;

// use crate::crdt_type::{CmRDT, CvRDT, DataType, Delta};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use tracing::{info, span, Level};

use crate::{crdt_type::DataType, text_operation::TextOperation};

pub trait CRDTBenchmark<K>
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn benchmark_name(&self) -> String;

    fn benchmark_cmrdt_result(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut system = sysinfo::System::new_all();
        let span = span!(
            Level::INFO,
            "Benchmark Type",
            operation = crdt.name() + "Operation base" + self.benchmark_name().as_str()
        );

        let _enter = span.enter();
        let start = Instant::now();
        self.run_cmrdt_benchmark(crdt, value, iterations);
        let duration = start.elapsed();
        info!(latency = ?duration, "Duration of iterations");
        system.refresh_cpu_usage();
        let cpu_usage = system.global_cpu_usage();
        info!(cpu_usage = ?cpu_usage, "CPU usage of iterations");
        system.refresh_memory();
        let memory_usage = system.used_memory();
        info!(memory_usage = ?memory_usage, "Memory usage of iterations");
        info!(avg_latency = ?duration / iterations, "Average latency of iterations");
        info!(avg_cpu_usage = ?cpu_usage / iterations as f32, "Average CPU usage of iterations");
        info!(avg_memory_usage = ?memory_usage / iterations as u64, "Average memory usage of iterations");
    }

    fn benchmark_cvrdt_result(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut system = sysinfo::System::new_all();
        let span = span!(
            Level::INFO,
            "Benchmark Type",
            operation = crdt.name() + "State base" + self.benchmark_name().as_str()
        );

        let _enter = span.enter();
        let start = Instant::now();
        self.run_cvrdt_benchmark(crdt, value, iterations);
        let duration = start.elapsed();
        info!(latency = ?duration, "Duration of iterations");
        system.refresh_cpu_usage();
        let cpu_usage = system.global_cpu_usage();
        info!(cpu_usage = ?cpu_usage, "CPU usage of iterations");
        system.refresh_memory();
        let memory_usage = system.used_memory();
        info!(memory_usage = ?memory_usage, "Memory usage of iterations");
        info!(avg_latency = ?duration / iterations, "Average latency of iterations");
        info!(avg_cpu_usage = ?cpu_usage / iterations as f32, "Average CPU usage of iterations");
        info!(avg_memory_usage = ?memory_usage / iterations as u64, "Average memory usage of iterations");
    }

    fn benchmark_delta_result(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut system = sysinfo::System::new_all();
        let span = span!(
            Level::INFO,
            "Benchmark Type",
            operation = crdt.name() + "Delta base" + self.benchmark_name().as_str()
        );

        let _enter = span.enter();
        let start = Instant::now();
        self.run_delta_benchmark(crdt, value, iterations);
        let duration = start.elapsed();
        info!(latency = ?duration, "Duration of iterations");
        system.refresh_cpu_usage();
        let cpu_usage = system.global_cpu_usage();
        info!(cpu_usage = ?cpu_usage, "CPU usage of iterations");
        system.refresh_memory();
        let memory_usage = system.used_memory();
        info!(memory_usage = ?memory_usage, "Memory usage of iterations");
        info!(avg_latency = ?duration / iterations, "Average latency of iterations");
        info!(avg_cpu_usage = ?cpu_usage / iterations as f32, "Average CPU usage of iterations");
        info!(avg_memory_usage = ?memory_usage / iterations as u64, "Average memory usage of iterations");
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32);
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32);
    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32);
}

pub struct SingleInsertEnd;
pub struct SingleDeleteEnd;
pub struct LongStringInsert;
pub struct LongStringDelete;
pub struct SingleInsertMiddle;
pub struct SingleDeleteMiddle;
pub struct ConcurrentInsertSame;
pub struct ConcurrentInsertDifferent;
pub struct ConcurrentDeleteSame;
pub struct ConcurrentDeleteDifferent;
pub struct ConcurrentInsertDeleteSame;
pub struct ConcurrentInsertDeleteDifferent;

impl<K> CRDTBenchmark<K> for SingleInsertEnd
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}

impl<K> CRDTBenchmark<K> for SingleDeleteEnd
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for LongStringInsert
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for LongStringDelete
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for SingleDeleteMiddle
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for ConcurrentInsertSame
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for ConcurrentInsertDifferent
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for ConcurrentDeleteSame
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for ConcurrentDeleteDifferent
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for ConcurrentInsertDeleteSame
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
impl<K> CRDTBenchmark<K> for ConcurrentInsertDeleteDifferent
where
    K: Eq + Clone + Hash + Ord + Serialize + for<'a> Deserialize<'a>,
{
    fn run_cmrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let text_operation = TextOperation::Insert {
            position: usize::MAX,
            value: value,
        };
        let ops = crdt.convert_operation(text_operation);
        for _ in 0..iterations {
            for op in ops.clone().into_iter() {
                crdt.apply_operation(op);
            }
        }
    }

    fn run_cvrdt_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            crdt.merge(&other);
        }
    }

    fn run_delta_benchmark(&self, crdt: &mut DataType<K>, value: K, iterations: u32) {
        let mut other = crdt.clone();
        for _ in 0..iterations {
            other.insert(usize::MAX, value.clone());
            let delta = other.generate_delta();
            crdt.apply_delta(&delta);
        }
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}
