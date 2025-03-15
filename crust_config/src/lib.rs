pub mod instance;
pub mod k8s_discovery;

pub fn get_crdt_service_yaml() -> &'static str {
    include_str!("k8s/service.yaml")
}

pub fn get_crdt_deployment_yaml() -> &'static str {
    include_str!("k8s/deployment.yaml")
}

pub fn get_crdt_service_account_yaml() -> &'static str {
    include_str!("k8s/service_account.yaml")
}
