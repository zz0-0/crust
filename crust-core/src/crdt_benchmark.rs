use std::time::Instant;

use tracing::{info, span, Level};

use crate::crdt_type::{CmRDT, CvRDT, Delta};

trait CvRDTBenchmark<K: CvRDT> {
    fn benchmark_name(&self) -> String;
    fn benchmark_result(&self, crdt: &mut K, iterations: u32) {
        let mut system = sysinfo::System::new_all();
        let span = span!(
            Level::INFO,
            "Benchmark Type",
            operation = crdt.name() + self.benchmark_name().as_str()
        );

        let _enter = span.enter();
        let start = Instant::now();
        self.run_benchmark(crdt, iterations);
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
    fn run_benchmark(&self, crdt: &mut K, iterations: u32);
}

trait CmRDTBenchmark<K: CmRDT> {
    fn benchmark_name(&self) -> String;
    fn benchmark_result(&self, crdt: &mut K, iterations: u32) {
        let mut system = sysinfo::System::new_all();
        let span = span!(
            Level::INFO,
            "Benchmark Type",
            operation = crdt.name() + self.benchmark_name().as_str()
        );

        let _enter = span.enter();
        let start = Instant::now();
        self.run_benchmark(crdt, iterations);
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
    fn run_benchmark(&self, crdt: &mut K, iterations: u32);
}

trait DeltaBenchmark<K: Delta> {
    fn benchmark_name(&self) -> String;
    fn benchmark_result(&self, crdt: &mut K, iterations: u32) {
        let mut system = sysinfo::System::new_all();
        let span = span!(
            Level::INFO,
            "Benchmark Type",
            operation = crdt.name() + self.benchmark_name().as_str()
        );

        let _enter = span.enter();
        let start = Instant::now();
        self.run_benchmark(crdt, iterations);
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
    fn run_benchmark(&self, crdt: &mut K, iterations: u32);
}

struct SingleInsertEnd;
struct SingleDeleteEnd;
struct LongStringInsert;
struct LongStringDelete;
struct SingleInsertMiddle;
struct SingleDeleteMiddle;
struct ConcurrentInsertSame;
struct ConcurrentInsertDifferent;
struct ConcurrentDeleteSame;
struct ConcurrentDeleteDifferent;
struct ConcurrentInsertDeleteSame;
struct ConcurrentInsertDeleteDifferent;

impl<K: CvRDT> CvRDTBenchmark<K> for SingleInsertEnd {
    fn run_benchmark(&self, crdt: &mut K, iterations: u32) {
        todo!()
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}

impl<K: CmRDT> CmRDTBenchmark<K> for SingleInsertEnd {
    fn run_benchmark(&self, crdt: &mut K, iterations: u32) {
        todo!()
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}

impl<K: Delta> DeltaBenchmark<K> for SingleInsertEnd {
    fn run_benchmark(&self, crdt: &mut K, iterations: u32) {
        todo!()
    }

    fn benchmark_name(&self) -> String {
        "Single Insert End".to_string()
    }
}

struct BenchmarkRunner<K> {
    cv_scenarios: Vec<Box<dyn CvRDTBenchmark<K>>>,
    cm_scenarios: Vec<Box<dyn CmRDTBenchmark<K>>>,
    delta_scenarios: Vec<Box<dyn DeltaBenchmark<K>>>,
}

impl<K> BenchmarkRunner<K> {
    fn new() -> Self {
        BenchmarkRunner {
            cv_scenarios: Vec::new(),
            cm_scenarios: Vec::new(),
            delta_scenarios: Vec::new(),
        }
    }

    pub fn add_cv_benchmark(&mut self, benchmark: Box<dyn CvRDTBenchmark<K>>) {
        self.cv_scenarios.push(benchmark);
    }

    pub fn add_cm_benchmark(&mut self, benchmark: Box<dyn CmRDTBenchmark<K>>) {
        self.cm_scenarios.push(benchmark);
    }

    pub fn add_delta_benchmark(&mut self, benchmark: Box<dyn DeltaBenchmark<K>>) {
        self.delta_scenarios.push(benchmark);
    }

    pub fn run_cv_benchmarks(&self, crdt: &mut K)
    where
        K: CvRDT,
    {
        for scenario in &self.cv_scenarios {
            scenario.benchmark_result(crdt, 1000);
        }
    }

    pub fn run_cm_benchmarks(&self, crdt: &mut K)
    where
        K: CmRDT,
    {
        for scenario in &self.cm_scenarios {
            scenario.benchmark_result(crdt, 1000);
        }
    }

    pub fn run_delta_benchmarks(&self, crdt: &mut K)
    where
        K: Delta,
    {
        for scenario in &self.delta_scenarios {
            scenario.benchmark_result(crdt, 1000);
        }
    }
}
