// pub enum MetricType {
//     Latency,
//     Throughput,
//     CpuUsage,
//     MemoryUsage,
//     ErrorRate,
//     ConvergenceTime,
//     Size,
//     Duration,
// }

// pub enum MetricCategory {
//     Performance,
//     Consistency,
//     Resource,
//     Error,
// }

// pub enum MetricAggregation {
//     Average,
//     P50,
//     P90,
//     P99,
//     Min,
//     Max,
//     Total,
//     Value,
// }

// pub enum MetricDataType {
//     Numeric,
//     TimeDuration,
//     Percentage,
//     Integer,
//     Text,
// }

// pub struct Metric {
//     pub metric_type: MetricType,
//     pub category: MetricCategory,
//     pub aggregation: Vec<MetricAggregation>,
//     pub data_type: MetricDataType,
// }

// pub struct Metrics {
//     pub metrics: Vec<Metric>,
// }

use std::time::Instant;

#[derive(Clone, Debug)]
pub enum MetricType {
    // inner command
    CommandExecutionTime,
    CommandSize,
    CommandRate,
    CommandQueueLength,

    MessageSize,
    MessageMergeTime,
    MessageRate,

    NetworkLatency,

    CpuUsage,
    MemoryUsage,
}

pub enum MetricUnit {
    Milliseconds,
    Bytes,
    Operations,
    OperationsPerSecond,
    Percentage,
    Count,
    Seconds,
}

pub enum MetricDataType {
    Numeric,
    TimeDuration,
    Percentage,
    Integer,
    Text,
}

pub enum MetricAggregation {
    Average,
    P50,
    P90,
    P99,
    Min,
    Max,
    Total,
    Count,
}

pub struct MetricValue {
    pub value: f64,
    pub timestamp: Instant,
}

pub struct Metric {
    pub metric_type: MetricType,
    pub values: Vec<MetricValue>,
}

impl MetricType {
    pub fn name(&self) -> String {
        match self {
            MetricType::CommandExecutionTime => "CommandExecutionTime".to_string(),
            MetricType::CommandSize => "CommandSize".to_string(),
            MetricType::CommandRate => "CommandRate".to_string(),
            MetricType::CommandQueueLength => "CommandQueueLength".to_string(),
            MetricType::MessageSize => "MessageSize".to_string(),
            MetricType::MessageMergeTime => "MessageMergeTime".to_string(),
            MetricType::MessageRate => "MessageRate".to_string(),
            MetricType::NetworkLatency => "NetworkLatency".to_string(),
            MetricType::CpuUsage => "CpuUsage".to_string(),
            MetricType::MemoryUsage => "MemoryUsage".to_string(),
        }
    }

    pub fn data_type(&self) -> MetricDataType {
        match self {
            MetricType::CommandExecutionTime => MetricDataType::TimeDuration,
            MetricType::CommandSize => MetricDataType::Numeric,
            MetricType::CommandRate => MetricDataType::Numeric,
            MetricType::CommandQueueLength => MetricDataType::Numeric,
            MetricType::MessageSize => MetricDataType::Numeric,
            MetricType::MessageMergeTime => MetricDataType::TimeDuration,
            MetricType::MessageRate => MetricDataType::Numeric,
            MetricType::NetworkLatency => MetricDataType::TimeDuration,
            MetricType::CpuUsage => MetricDataType::Percentage,
            MetricType::MemoryUsage => MetricDataType::Percentage,
        }
    }

    pub fn unit(&self) -> MetricUnit {
        match self {
            MetricType::CommandExecutionTime => MetricUnit::Milliseconds,
            MetricType::CommandSize => MetricUnit::Bytes,
            MetricType::CommandRate => MetricUnit::OperationsPerSecond,
            MetricType::CommandQueueLength => MetricUnit::Count,
            MetricType::MessageSize => MetricUnit::Bytes,
            MetricType::MessageMergeTime => MetricUnit::Milliseconds,
            MetricType::MessageRate => MetricUnit::OperationsPerSecond,
            MetricType::NetworkLatency => MetricUnit::Milliseconds,
            MetricType::CpuUsage => MetricUnit::Percentage,
            MetricType::MemoryUsage => MetricUnit::Percentage,
        }
    }

    pub fn aggregations(&self) -> Vec<MetricAggregation> {
        match self {
            MetricType::CommandExecutionTime => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::CommandSize => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::CommandRate => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::CommandQueueLength => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::MessageSize => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::MessageMergeTime => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::MessageRate => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::NetworkLatency => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
                MetricAggregation::Max,
            ],
            MetricType::CpuUsage => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
            ],
            MetricType::MemoryUsage => vec![
                MetricAggregation::Average,
                MetricAggregation::P50,
                MetricAggregation::P90,
                MetricAggregation::P99,
                MetricAggregation::Min,
            ],
        }
    }
}
