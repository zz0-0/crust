#[cfg(feature = "byzantine")]
pub mod byzantine;

#[cfg(feature = "confidentiality")]
pub mod confidentiality;

#[cfg(feature = "integrity")]
pub mod integrity;

#[cfg(feature = "access_control")]
pub mod access;

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
}
