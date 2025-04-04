use std::{collections::HashMap, time::Duration};

use crust_core::r#type::CrdtType;

use crate::collector::metrics_collector::BenchmarkMetricsCollector;
use crate::config::benchmark_config::BenchmarkConfig;
pub struct BenchmarkResults {
    pub name: String,
    pub crdt_type: CrdtType<String>,
    pub duration: Duration,
    
}

pub struct BenchmarkReporter {
    pub config: BenchmarkConfig,
    pub metrics_collector: BenchmarkMetricsCollector,
    pub results: Vec<BenchmarkResults>,
}

impl BenchmarkReporter {
    pub fn new(config: BenchmarkConfig, metrics_collector: BenchmarkMetricsCollector) -> Self {
        Self {
            config,
            metrics_collector,
            results: Vec::new(),
        }
    }

    pub fn report(&self) {
        for result in &self.results {
            println!("Benchmark Name: {}", result.name);
            println!("CRDT Type: {:?}", result.crdt_type);
            println!("Duration: {:?}", result.duration);
        }
    }

    pub fn add_result(&mut self, result: BenchmarkResults) {
        self.results.push(result);
    }
}
