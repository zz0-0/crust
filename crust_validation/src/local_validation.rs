use crust_core::sync::{DeltaBased, OperationBased, StateBased};

pub trait StateBasedValidation<K>
where
    K: StateBased,
{
    fn state_associativity() -> bool;
    fn state_commutativity() -> bool;
    fn state_idempotence() -> bool;
    fn state_monotonicity() -> bool;
}

pub trait OperationBasedValidation<K>
where
    K: OperationBased,
{
    fn operation_commutativity() -> bool;
    fn operation_delivery_precondition() -> bool;
    fn operation_effect_relation() -> bool;
}

pub trait DeltaBasedValidation<K>
where
    K: DeltaBased,
{
    fn delta_associativity() -> bool;
    fn delta_commutativity() -> bool;
    fn delta_idempotence() -> bool;
    fn delta_state_composability() -> bool;
}
