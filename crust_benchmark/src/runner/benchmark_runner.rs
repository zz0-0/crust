use std::time::Instant;

use crust_core::{
    command::{CounterInnerCommand, CrdtInnerCommand},
    r#type::{CrdtType, CrdtTypeVariant},
};

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
    pub fn new(config: BenchmarkConfig) -> Self {
        let metrics_collector = BenchmarkMetricsCollector::new(crust_core::sync::SyncType::State);
        let reporter = BenchmarkReporter::new(config.clone(), metrics_collector.clone());

        Self {
            config,
            metrics_collector,
            reporter,
            replicas: Vec::new(),
            start_time: None,
        }
    }

    pub fn setup(&mut self) -> Result<(), BenchmarkError> {
        for _ in 0..self.config.replica_count {
            let replica = CrdtType::new("GCounter".to_string()).unwrap();
            self.replicas.push(replica);
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), BenchmarkError> {
        self.start_time = Some(Instant::now());

        let workload = generate_workload(
            CrdtType::new("GCounter".to_string()).unwrap(),
            self.config.command_count,
        );

        for operation in workload {
            match operation {
                CrdtInnerCommand::Counter(cmd) => {
                    for replica in &mut self.replicas {
                        match cmd {
                            CounterInnerCommand::Increment { ref value } => {
                                replica.apply_command(&CrdtInnerCommand::Counter(
                                    CounterInnerCommand::Increment {
                                        value: value.to_string(),
                                    },
                                ));
                            }
                            CounterInnerCommand::Decrement { ref value } => {
                                replica.apply_command(&CrdtInnerCommand::Counter(
                                    CounterInnerCommand::Decrement {
                                        value: value.to_string(),
                                    },
                                ));
                            }
                        }
                    }
                }
                CrdtInnerCommand::Graph(graph_inner_command) => todo!(),
                CrdtInnerCommand::Set(set_inner_command) => todo!(),
                CrdtInnerCommand::Text(text_inner_command) => todo!(),
            }
        }

        self.metrics_collector.get_metrics();

        Ok(())
    }

    pub fn report(&mut self) -> Result<(), BenchmarkError> {
        if let Some(start_time) = self.start_time {
            let duration = start_time.elapsed();
            let results = BenchmarkResults {
                name: "GCounter".to_string(),
                crdt_type: CrdtType::new("GCounter".to_string()).unwrap(),
                duration,
            };
            self.reporter.add_result(results);
        }

        self.reporter.report();

        Ok(())
    }
}
