use std::{collections::HashMap, time::Instant};

use crust_core::sync::SyncType;

use crate::metrics::metrics_definition::{Metric, MetricType, MetricValue};

pub struct BenchmarkMetricsCollector {
    pub metrics: HashMap<String, Metric>,
    pub sync_type: SyncType,
    pub start_time: Option<Instant>,
}

pub trait MetricsCollector {
    fn record(&mut self, metric_type: MetricType, value: f64);
    fn start_timing(&mut self);
    fn stop_timing(&mut self, metric_type: MetricType);
    fn get_metrics(&self) -> &HashMap<String, Metric>;
    fn add_metric(&mut self, metric_type: MetricType);
    fn reset(&mut self);
}

impl BenchmarkMetricsCollector {
    pub fn new(sync_type: SyncType) -> Self {
        BenchmarkMetricsCollector {
            metrics: HashMap::new(),
            sync_type,
            start_time: None,
        }
    }

    pub fn with_default_metrics(sync_type: SyncType) -> Self {
        let mut collector = Self::new(sync_type);
        collector.add_metric(MetricType::CommandExecutionTime);
        collector.add_metric(MetricType::CommandSize);
        collector.add_metric(MetricType::CommandRate);

        match sync_type {
            SyncType::Delta => {
                collector.add_metric(MetricType::CommandQueueLength);
                collector.add_metric(MetricType::MessageSize);
                collector.add_metric(MetricType::MessageMergeTime);
                collector.add_metric(MetricType::MessageRate);
            }
            SyncType::State => {
                collector.add_metric(MetricType::NetworkLatency);
            }
            SyncType::Operation => {
                collector.add_metric(MetricType::NetworkLatency);
            }
        }

        collector
    }
}

impl MetricsCollector for BenchmarkMetricsCollector {
    fn record(&mut self, metric_type: MetricType, value: f64) {
        let key = metric_type.name();
        if let Some(metric) = self.metrics.get_mut(&key) {
            metric.values.push(MetricValue {
                value,
                timestamp: Instant::now(),
            });
        }
    }

    fn start_timing(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn stop_timing(&mut self, metric_type: MetricType) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f64();
            self.record(metric_type, elapsed);
        }
    }

    fn get_metrics(&self) -> &HashMap<String, Metric> {
        &self.metrics
    }

    fn add_metric(&mut self, metric_type: MetricType) {
        let key = metric_type.name();
        self.metrics.insert(
            key,
            Metric {
                metric_type,
                values: Vec::new(),
            },
        );
    }

    fn reset(&mut self) {
        self.metrics.clear();
        self.start_time = None;
    }
}
