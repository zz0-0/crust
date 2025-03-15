use std::{collections::HashMap, time::Duration};

use crust_core::r#type::CrdtType;

pub struct BenchmarkResults {
    pub name: String,
    pub crdt_type: CrdtType<String>,
    pub duration: Duration,
    // pub metrics: HashMap<String, Metr>
}

pub struct BenchmarkReporter {}

impl BenchmarkReporter {
    pub fn new() -> Self {
        BenchmarkReporter {}
    }

    pub fn generate_report(&self, results: Vec<BenchmarkResults>) {
        for result in results {
            println!("Benchmark: {}", result.name);
            println!("Crdt Type: {:?}", result.crdt_type);
            println!("Duration: {:?}", result.duration);
        }
    }
}
