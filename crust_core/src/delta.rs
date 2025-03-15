use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::core::counter::gcounter::GCounterDelta;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CrdtDelta<K>
where
    K: Eq + Hash,
{
    GCounter(GCounterDelta<K>),
}
