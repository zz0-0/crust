use std::hash::Hash;

use crate::{delta::CrdtDelta, operation::CrdtOperation, r#type::CrdtType, sync::Crdt};

pub struct SecurityContext {
    pub request_id: Option<String>,
    pub timestamp: u64,
}

impl SecurityContext {
    pub fn new() -> Self {
        Self {
            request_id: None,
            timestamp: 0,
        }
    }
}

pub trait SecurityHook<K>
where
    K: Eq + Hash + Clone,
{
    #[cfg(feature = "byzantine")]
    fn validate_state(&self, state: &CrdtType<K>) -> bool;
    #[cfg(feature = "byzantine")]
    fn validate_operation(&self, operation: &CrdtOperation<K>) -> bool;
    #[cfg(feature = "byzantine")]
    fn validate_delta(&self, delta: &CrdtDelta<K>) -> bool;

    #[cfg(feature = "confidentiality")]
    fn encrypt_data(&self, data: &CrdtType<K>) -> CrdtType<K>;
    #[cfg(feature = "confidentiality")]
    fn decrypt_data(&self, data: &CrdtType<K>) -> CrdtType<K>;

    #[cfg(feature = "integrity")]
    fn sign_data(&self, data: &CrdtType<K>) -> CrdtType<K>;
    #[cfg(feature = "integrity")]
    fn verify_data(&self, data: &CrdtType<K>) -> bool;

    #[cfg(feature = "access_control")]
    fn check_access(&self, data: &CrdtType<K>) -> bool;
    #[cfg(feature = "access_control")]
    fn audit_log(&self, data: &CrdtType<K>);
}
