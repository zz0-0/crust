use std::{
    collections::HashMap,
    process::{Command, Stdio},
    time::Duration,
};

use crust_core::{
    command::CrdtInnerCommand,
    r#type::CrdtType,
    sync::{SyncMode, SyncType},
};
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    api::{Patch, PatchParams},
    Api,
};
use rand::{rng, Rng};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::get_crdt_service_yaml;

#[derive(Clone)]
pub struct DeploymentConfig {
    pub num_replicas: usize,
    pub crdt_type: CrdtType<String>,
    pub sync_type: SyncType,
    pub sync_mode: SyncMode,
    pub batch_times: Option<usize>,
    pub batch_interval: Option<Duration>,
    pub network_scenario: Option<NetworkScenario>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentReplicaPatch {
    replicas: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentPacketLossPatch {
    packet_loss: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentLatencyPatch {
    latency: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeploymentBandwidthPatch {
    bandwidth: Option<u64>,
}

#[derive(Clone)]
pub struct NetworkScenario {
    pub packet_loss: Option<f64>,
    pub latency: Option<u32>,
    pub bandwidth: Option<u64>,
}

impl DeploymentConfig {
    pub fn new(
        num_replicas: usize,
        crdt_type_str: &str,
        sync_type_str: &str,
        sync_mode_str: &str,
        batch_times: Option<usize>,
        batch_interval: Option<Duration>,
    ) -> Self {
        DeploymentConfig {
            num_replicas,
            crdt_type: CrdtType::new(crdt_type_str.to_string()).unwrap(),
            sync_type: SyncType::new(sync_type_str.to_string()),
            sync_mode: SyncMode::new(sync_mode_str.to_string()),
            batch_times,
            batch_interval,
            network_scenario: None,
        }
    }
}

pub async fn setup_remote_test_environement(config: &DeploymentConfig) -> HashMap<String, String> {
    let mut service_base_urls: HashMap<String, String> = HashMap::new();
    let instance_ids = (1..=config.num_replicas)
        .map(|i| format!("replica-{}", i))
        .collect::<Vec<String>>();
    let manifest_content = get_crdt_service_yaml();

    let kubectl_apply_output = Command::new("kubectl")
        .args(&["apply", "-f", manifest_content])
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute kubectl apply");

    if !kubectl_apply_output.status.success() {
        panic!(
            "failed to apply kubernetes manifest: {}",
            String::from_utf8_lossy(&kubectl_apply_output.stderr)
        );
    }

    for id in &instance_ids {
        let replica_num = id.trim_start_matches("replica").parse::<i32>().unwrap();
        let service_name = format!("crdt-network-service");
        let deployment_name = format!("crdt-network-{}", replica_num);

        println!(
            "Waiting for Deployment '{}' to be ready...",
            deployment_name
        );
        let kubectl_wait_output = Command::new("kubectl")
            .args(&["wait", "--for=condition=available", "--timeout=5m"])
            .arg(format!("deployment/{}", deployment_name))
            .stderr(Stdio::piped())
            .output()
            .expect("failed to execute kubectl wait");
        if !kubectl_wait_output.status.success() {
            panic!(
                "failed to wait for deployment: {}",
                String::from_utf8_lossy(&kubectl_wait_output.stderr)
            );
        }
        println!("Deployment '{}' is ready.", deployment_name);

        println!("Waiting for Service '{}' to be ready...", service_name);
        let kubectl_wait_output = Command::new("kubectl")
            .args(&["wait", "--for=condition=ready", "--timeout=5m"])
            .arg(format!("service/{}", service_name))
            .stderr(Stdio::piped())
            .output()
            .expect("failed to execute kubectl wait");
        if !kubectl_wait_output.status.success() {
            panic!(
                "failed to wait for service: {}",
                String::from_utf8_lossy(&kubectl_wait_output.stderr)
            );
        }
        println!("Service '{}' is ready.", service_name);

        println!("Discovering service URL for Service '{}'...", service_name);
        let kubectl_get_service_output = Command::new("kubectl")
            .arg("get")
            .arg("service")
            .arg(service_name)
            .arg("-o")
            .arg("jsonpath='{.spec.clusterIP}:{.spec.ports[0].port}'")
            .output()
            .expect("Failed to execute kubectl get service");
        if !kubectl_get_service_output.status.success() {
            panic!(
                "failed to get service URL: {}",
                String::from_utf8_lossy(&kubectl_get_service_output.stderr)
            );
        }
    }

    service_base_urls
}

pub async fn teardown_remote_test_environment(config: &DeploymentConfig) {
    let manifest_content = get_crdt_service_yaml();

    let kubectl_delete_output = Command::new("kubectl")
        .args(&["delete", "-f", manifest_content])
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute kubectl delete");

    if !kubectl_delete_output.status.success() {
        panic!(
            "failed to delete kubernetes manifest: {}",
            String::from_utf8_lossy(&kubectl_delete_output.stderr)
        );
    }
}

pub async fn send_command_to_instance(
    instance_id: &str,
    crdt_type_str: &str,
    sync_type_str: &str,
    sync_mode_str: &str,
    command: CrdtInnerCommand<String>,
    service_base_url: &str,
) -> Result<StatusCode, String> {
    let client = Client::new();
    let url = format!(
        "{}/internal/receive/{}/{}/{}",
        service_base_url, crdt_type_str, sync_type_str, sync_mode_str
    );

    let response_result = client.post(&url).json(&command).send().await;

    match response_result {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response.status())
            } else {
                Err(format!(
                    "HTTP request failed with status: {}, Response Body: {:?}",
                    response.status(),
                    response.text().await
                ))
            }
        }
        Err(e) => Err(format!("Error sending HTTP request: {}", e)),
    }
}

pub async fn send_command_to_instance_with_loss(
    instance_id: &str,
    crdt_type_str: &str,
    sync_type_str: &str,
    sync_mode_str: &str,
    command: CrdtInnerCommand<String>,
    service_base_url: &str,
    message_loss_probability: f64,
) -> Result<StatusCode, String> {
    {
        let mut rng = rng();
        if rng.random_bool(message_loss_probability) {
            eprintln!("Simulating MESSAGE LOSS for command: {:?}", command);
            return Ok(StatusCode::OK);
        }
    };

    let client = Client::new();
    let url = format!(
        "{}/internal/receive/{}/{}/{}",
        service_base_url, crdt_type_str, sync_type_str, sync_mode_str
    );

    let response_result = client.post(&url).json(&command).send().await;

    match response_result {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response.status())
            } else {
                Err(format!(
                    "HTTP request failed with status: {}, Response Body: {:?}",
                    response.status(),
                    response.text().await
                ))
            }
        }
        Err(e) => Err(format!("Error sending HTTP request: {}", e)),
    }
}

pub async fn get_state_from_instance(
    instance_id: &str,
    crdt_type: &CrdtType<String>,
    service_base_url: &str,
) -> Result<CrdtType<String>, String> {
    let client = Client::new();
    let url = format!("{}/internal/state", service_base_url);

    let response_result = client.get(&url).send().await;

    match response_result {
        Ok(response) => {
            if response.status().is_success() {
                let state = response.json::<CrdtType<String>>().await;
                match state {
                    Ok(state) => Ok(state),
                    Err(e) => Err(format!("Error parsing response body: {}", e)),
                }
            } else {
                Err(format!(
                    "HTTP request failed with status: {}, Response Body: {:?}",
                    response.status(),
                    response.text().await
                ))
            }
        }
        Err(e) => Err(format!("Error sending HTTP request: {}", e)),
    }
}

pub async fn update_replicas(
    client: kube::Client,
    namespace: &str,
    deployment_name: &str,
    replicas: usize,
) {
    let api: Api<Deployment> = Api::namespaced(client, namespace);

    let patch = DeploymentReplicaPatch {
        replicas: Some(replicas),
    };

    let patch = Patch::Apply(&patch);

    let _ = api
        .patch(deployment_name, &PatchParams::default(), &patch)
        .await
        .unwrap();
}

pub async fn update_packet_loss(
    client: kube::Client,
    namespace: &str,
    deployment_name: &str,
    network_scenario: &NetworkScenario,
) {
    let api: Api<Deployment> = Api::namespaced(client, namespace);

    let patch = DeploymentPacketLossPatch {
        packet_loss: network_scenario.packet_loss,
    };

    let patch = Patch::Apply(&patch);

    let _ = api
        .patch(deployment_name, &PatchParams::default(), &patch)
        .await
        .unwrap();
}

pub async fn update_latency(
    client: kube::Client,
    namespace: &str,
    deployment_name: &str,
    network_scenario: &NetworkScenario,
) {
    let api: Api<Deployment> = Api::namespaced(client, namespace);

    let patch = DeploymentLatencyPatch {
        latency: network_scenario.latency,
    };

    let patch = Patch::Apply(&patch);

    let _ = api
        .patch(deployment_name, &PatchParams::default(), &patch)
        .await
        .unwrap();
}

pub async fn update_bandwidth(
    client: kube::Client,
    namespace: &str,
    deployment_name: &str,
    network_scenario: &NetworkScenario,
) {
    let api: Api<Deployment> = Api::namespaced(client, namespace);

    let patch = DeploymentBandwidthPatch {
        bandwidth: network_scenario.bandwidth,
    };

    let patch = Patch::Apply(&patch);

    let _ = api
        .patch(deployment_name, &PatchParams::default(), &patch)
        .await
        .unwrap();
}
