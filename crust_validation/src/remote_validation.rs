use crust_core::sync::{DeltaBased, OperationBased, StateBased};

pub trait StateBasedDistributedConvergenceValidation<K>
where
    K: StateBased,
{
    async fn state_based_converge_concurrent_operations() -> bool;
    async fn state_based_converge_delayed_deliveries() -> bool;
    async fn state_based_converge_mixed_operations() -> bool;
    async fn state_based_converge_under_load() -> bool;
}

pub trait OperationBasedDistributedConvergenceValidation<K>
where
    K: OperationBased,
{
    async fn operation_based_converge_concurrent_operations() -> bool;
    async fn operation_based_converge_delayed_deliveries() -> bool;
    async fn operation_based_converge_mixed_operations() -> bool;
    async fn operation_based_converge_under_load() -> bool;
}

pub trait DeltaBasedDistributedConvergenceValidation<K>
where
    K: DeltaBased,
{
    async fn delta_based_converge_concurrent_operations() -> bool;
    async fn delta_based_converge_delayed_deliveries() -> bool;
    async fn delta_based_converge_mixed_operations() -> bool;
    async fn delta_based_converge_under_load() -> bool;
}

pub trait OperationBasedDistributedCausalConsistencyValidation<K>
where
    K: OperationBased,
{
    async fn operation_based_causal_order_simple_dependency() -> bool;
    async fn operation_based_causal_order_complex_dependency() -> bool;
    async fn operation_based_causal_order_concurrent_dependency() -> bool;
    async fn operation_based_causal_order_delayed_delivery() -> bool;
}

pub trait StateBasedDistributedNetworkRobustnessValidation<K>
where
    K: StateBased,
{
    async fn state_based_robustness_message_loss() -> bool;
    async fn state_based_robustness_network_partition() -> bool;
}

pub trait OperationBasedDistributedNetworkRobustnessValidation<K>
where
    K: OperationBased,
{
    async fn operation_based_robustness_message_loss() -> bool;
    async fn operation_based_robustness_message_reordering() -> bool;
    async fn operation_based_robustness_network_partition() -> bool;
}

pub trait DeltaBasedDistributedNetworkRobustnessValidation<K>
where
    K: DeltaBased,
{
    async fn delta_based_robustness_message_loss() -> bool;
    async fn delta_based_robustness_message_reordering() -> bool;
    async fn delta_based_robustness_network_partition() -> bool;
}
