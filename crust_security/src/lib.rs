#[cfg(feature = "byzantine")]
pub mod byzantine;

#[cfg(feature = "confidentiality")]
pub mod confidentiality;

#[cfg(feature = "integrity")]
pub mod integrity;

#[cfg(feature = "access_control")]
pub mod access;


pub mod hook;

use crust_core::security::SecurityHook;

use hook::CrustSecurityHook;
use std::hash::Hash;

pub struct SecurityManager {
    #[cfg(feature = "byzantine")]
    byzantine: byzantine::ByzantineSecurity,

    #[cfg(feature = "confidentiality")]
    confidentiality: confidentiality::ConfidentialitySecurity,

    #[cfg(feature = "integrity")]
    integrity: integrity::IntegritySecurity,

    #[cfg(feature = "access_control")]
    access: access::AccessControl,
}

impl SecurityManager {
    pub fn new() -> Self {
        SecurityManager {
            #[cfg(feature = "byzantine")]
            byzantine: byzantine::ByzantineSecurity::new(),

            #[cfg(feature = "confidentiality")]
            confidentiality: confidentiality::ConfidentialitySecurity::new(),

            #[cfg(feature = "integrity")]
            integrity: integrity::IntegritySecurity::new(),

            #[cfg(feature = "access_control")]
            access: access::AccessControl::new(),
        }
    }

    
    pub fn create_security_hook<K>(&self) -> impl SecurityHook<K>
    where
        K: Eq + Hash + Clone + 'static,
    {
        CrustSecurityHook::new()
    }

    #[cfg(feature = "byzantine")]
    pub fn configure_byzantine(&mut self, threshold: f64) {
        self.byzantine = byzantine::ByzantineSecurity::with_threshold(threshold);
    }

    #[cfg(feature = "confidentiality")]
    pub fn configure_confidentiality(&mut self, enabled: bool) {
        self.confidentiality = confidentiality::ConfidentialitySecurity::with_encryption(enabled);
    }

    #[cfg(feature = "integrity")]
    pub fn configure_integrity(&mut self, enabled: bool) {
        self.integrity = integrity::IntegritySecurity::with_signature(enabled);
    }
}
