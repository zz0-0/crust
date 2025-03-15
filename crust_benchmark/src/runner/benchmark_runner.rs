use std::time::Instant;

use crust_core::r#type::{CrdtType, CrdtTypeVariant};

use crate::{
    collector::metrics_collector::{BenchmarkMetricsCollector, MetricsCollector},
    config::benchmark_config::BenchmarkConfig,
    metrics::metrics_definition::MetricType,
    reporter::benchmark_report::{BenchmarkReporter, BenchmarkResults},
    workload::{self, workload_generation::generate_workload},
};

pub enum BenchmarkError {
    SetupError,
    WorkloadError,
    MetricsError,
    ReportError,
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    metrics_collector: BenchmarkMetricsCollector,
    reporter: BenchmarkReporter,
    replicas: Vec<CrdtType<String>>,
    start_time: Option<Instant>,
}

impl BenchmarkRunner {
    // pub fn new(config: BenchmarkConfig) -> Self {
    //     let metrics_collector = BenchmarkMetricsCollector::with_default_metrics(config.sync_type);
    //     let reporter = BenchmarkReporter::new();

    //     BenchmarkRunner {
    //         config,
    //         metrics_collector,
    //         reporter,
    //         replicas: Vec::new(),
    //         start_time: None,
    //     }
    // }

    // pub fn setup_environment(&mut self) -> Result<(), BenchmarkError> {
    //     // Setup environment
    //     for _ in 0..self.config.replica_count {
    //         let crdt = match self.config.crdt_type.variant {
    //             CrdtTypeVariant::GCounter(_) => CrdtType::<String>::new("gcounter".to_string()),
    //         };
    //         self.replicas.push(crdt.unwrap());
    //     }

    //     Ok(())
    // }

    // pub fn execute_workload(&mut self) -> Result<(), BenchmarkError> {
    //     let workload = generate_workload(self.config.crdt_type, self.config.command_count);

    //     self.start_time = Some(Instant::now());

    //     for (i, command) in workload.iter().enumerate() {
    //         let replica = &mut self.replicas[i % self.replicas.len()];

    //         self.metrics_collector.start_timing();

    //         if let Some(command) = replica.apply_command(&command) {
    //             self.metrics_collector
    //                 .stop_timing(MetricType::CommandExecutionTime);
    //         }
    //     }

    //     Ok(())
    // }

    // pub fn collect_results(&mut self) -> Result<Vec<BenchmarkResults>, BenchmarkError> {
    //     let mut results = Vec::new();

    //     let duration = if let Some(start_time) = self.start_time {
    //         start_time.elapsed()
    //     } else {
    //         return Err(BenchmarkError::MetricsError);
    //     };

    //     let result = BenchmarkResults {
    //         name: self.config.crdt_type.name(),
    //         crdt_type: self.config.crdt_type.clone(),
    //         duration: duration,
    //     };

    //     results.push(result);
    //     Ok(results)
    // }

    // pub fn run_benchmark(&mut self) -> Result<(), BenchmarkError> {
    //     self.setup_environment()?;
    //     self.execute_workload()?;
    //     let results = self.collect_results()?;
    //     let report = self.reporter.generate_report(results);

    //     Ok(())
    // }
}
