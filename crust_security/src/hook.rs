use std::hash::Hash;
use std::marker::PhantomData;

use crust_core::security::SecurityHook;

#[cfg(feature = "access_control")]
use crate::access::AccessControl;
#[cfg(feature = "byzantine")]
use crate::byzantine::ByzantineSecurity;
#[cfg(feature = "confidentiality")]
use crate::confidentiality::ConfidentialitySecurity;
#[cfg(feature = "integrity")]
use crate::integrity::IntegritySecurity;

pub struct CrustSecurityHook<K>
where
    K: Eq + Hash + Clone,
{
    #[cfg(feature = "byzantine")]
    byzantine: ByzantineSecurity,
    #[cfg(feature = "confidentiality")]
    confidentiality: ConfidentialitySecurity,
    #[cfg(feature = "integrity")]
    integrity: IntegritySecurity,
    #[cfg(feature = "access_control")]
    access: AccessControl,
    _marker: PhantomData<K>,
}

impl<K> CrustSecurityHook<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        CrustSecurityHook {
            #[cfg(feature = "byzantine")]
            byzantine: ByzantineSecurity::new(),
            #[cfg(feature = "confidentiality")]
            confidentiality: ConfidentialitySecurity::new(),
            #[cfg(feature = "integrity")]
            integrity: IntegritySecurity::new(),
            #[cfg(feature = "access_control")]
            access: AccessControl::new(),
            _marker: PhantomData,
        }
    }
}

impl<K> SecurityHook<K> for CrustSecurityHook<K>
where
    K: Eq + Hash + Clone,
{
    #[cfg(feature = "byzantine")]
    fn validate_state(&self, state: &CrdtType<K>) -> bool {
        self.byzantine.validate_state(state)
    }

    #[cfg(feature = "byzantine")]
    fn validate_operation(&self, operation: &CrdtOperation<K>) -> bool {
        self.byzantine.validate_operation(operation)
    }

    #[cfg(feature = "byzantine")]
    fn validate_delta(&self, delta: &CrdtDelta<K>) -> bool {
        self.byzantine.validate_delta(delta)
    }

    #[cfg(feature = "confidentiality")]
    fn encrypt_data(&self, data: &CrdtType<K>) -> CrdtType<K> {
        self.confidentiality.encrypt_data(data)
    }

    #[cfg(feature = "confidentiality")]
    fn decrypt_data(&self, data: &CrdtType<K>) -> CrdtType<K> {
        self.confidentiality.decrypt_data(data)
    }

    #[cfg(feature = "integrity")]
    fn sign_data(&self, data: &CrdtType<K>) -> CrdtType<K> {
        self.integrity.sign_data(data)
    }

    #[cfg(feature = "integrity")]
    fn verify_data(&self, data: &CrdtType<K>) -> bool {
        self.integrity.verify_data(data)
    }

    #[cfg(feature = "access_control")]
    fn check_access(&self, data: &CrdtType<K>) -> bool {
        
        self.access.check_access_security(data)
    }

    #[cfg(feature = "access_control")]
    fn audit_log(&self, data: &CrdtType<K>) {
        
        
        

        
        println!("AUDIT: Access to {} (from SecurityHook)", data.name());
    }
}
