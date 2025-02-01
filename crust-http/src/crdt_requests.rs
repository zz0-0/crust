use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use crust_core::text_operation::TextOperation;
use crust_core::{crdt_data_type::DataType, text_operation::Message};
use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api};
use reqwest::StatusCode;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    crdt: Arc<RwLock<DataType<String>>>,
    pod_list: Vec<Pod>,
}

impl AppState {
    pub fn new() -> Self {
        let pod_list = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Self::setup_k8s_environment());
        let crdt = DataType::new("none".to_string());
        let crdt = Arc::new(RwLock::new(crdt));
        Self { crdt, pod_list }
    }

    pub fn get_or_create_crdt(&mut self, crdt_type: String) -> Arc<RwLock<DataType<String>>> {
        if self.get_crdt().read().unwrap().name() == crdt_type {
            return self.get_crdt();
        }
        self.set_crdt(crdt_type)
    }

    pub fn set_crdt(&mut self, crdt_type: String) -> Arc<RwLock<DataType<String>>> {
        let crdt = DataType::new(crdt_type);
        self.crdt = Arc::new(RwLock::new(crdt));
        self.get_crdt()
    }

    pub fn get_crdt(&self) -> Arc<RwLock<DataType<String>>> {
        self.crdt.clone()
    }

    pub fn set_pod_list(&mut self, pod_list: Vec<Pod>) {
        self.pod_list = pod_list;
    }

    pub fn get_pod_list(&self) -> Vec<Pod> {
        self.pod_list.clone()
    }

    pub async fn setup_k8s_environment() -> Vec<Pod> {
        let client = kube::Client::try_default().await.unwrap();
        let pods: Api<Pod> = Api::namespaced(client, "default");
        let lp = ListParams::default().labels("app=crust-http");
        let pod_list = pods.list(&lp).await.unwrap();
        println!("Total pods found: {}", pod_list.items.len());
        for pod in &pod_list.items {
            println!("Pod: {:?}", pod.metadata.name);
            println!("Status: {:?}", pod.status.as_ref().map(|s| &s.phase));
        }
        let running_pods: Vec<Pod> = pod_list
            .items
            .iter()
            .filter(|pod| {
                if let Some(status) = &pod.status {
                    if let Some(phase) = &status.phase {
                        return phase == "Running";
                    }
                }
                false
            })
            .cloned()
            .collect();
        println!("Running pods: {}", running_pods.len());

        if running_pods.len() < 1 {
            panic!(
                "Not enough running pods. Found {}, needed 1",
                running_pods.len()
            );
        }

        running_pods
    }

    pub async fn send_operation(
        &mut self,
        pod_name: &str,
        crdt_type: &str,
        operation: &str,
        message: &Message,
    ) -> impl IntoResponse {
        let url = format!(
            "http://{}:3000/crdt/{}/operation/{}",
            pod_name, crdt_type, operation
        );
        let client = reqwest::Client::new();
        let res = client.post(&url).json(message).send().await.unwrap();
        (StatusCode::OK, res.text().await.unwrap())
    }

    pub async fn broadcast_operation(
        &mut self,
        crdt_type: String,
        operation: String,
        message: &Message,
    ) -> impl IntoResponse {
        let current_pod = std::env::var("POD_NAME").unwrap();
        for pod in self.get_pod_list() {
            if let Some(pod_name) = pod.metadata.name.as_ref() {
                if pod_name == &current_pod {
                    continue;
                }
            }
            let pod_name = pod.metadata.name.as_ref().unwrap();
            let res = self
                .send_operation(pod_name, &crdt_type, &operation, &message)
                .await;
            println!("Broadcasted operation to {}, with crdt type {}, with operation - {}, with message - {:?}, return with result of {:?}", pod_name, crdt_type, operation, message, res.into_response());
        }
        (StatusCode::OK, "Broadcasted operation")
    }

    pub async fn send_state(
        &mut self,
        pod_name: &str,
        crdt_type: &str,
        state: &str,
    ) -> impl IntoResponse {
        let url = format!(
            "http://{}:3000/crdt/{}/state/{}",
            pod_name, crdt_type, state
        );
        let client = reqwest::Client::new();
        let res = client.get(&url).send().await.unwrap();
        (StatusCode::OK, res.text().await.unwrap())
    }

    pub async fn broadcast_state(&mut self, crdt_type: String, state: String) -> impl IntoResponse {
        let current_pod = std::env::var("POD_NAME").unwrap();
        for pod in self.get_pod_list() {
            if let Some(pod_name) = pod.metadata.name.as_ref() {
                if pod_name == &current_pod {
                    continue;
                }
            }
            let pod_name = pod.metadata.name.as_ref().unwrap();
            let res = self.send_state(pod_name, &crdt_type, &state).await;
            println!("Broadcasted state to {}, with crdt type {}, with state - {}, return with result of {:?}", pod_name, crdt_type, state, res.into_response());
        }
        (StatusCode::OK, "Broadcasted state")
    }

    pub async fn send_delta(
        &mut self,
        pod_name: &str,
        crdt_type: &str,
        delta: &str,
    ) -> impl IntoResponse {
        let url = format!(
            "http://{}:3000/crdt/{}/delta/{}",
            pod_name, crdt_type, delta
        );
        let client = reqwest::Client::new();
        let res = client.get(&url).send().await.unwrap();
        (StatusCode::OK, res.text().await.unwrap())
    }

    pub async fn broadcast_delta(&mut self, crdt_type: String, delta: String) -> impl IntoResponse {
        let current_pod = std::env::var("POD_NAME").unwrap();
        for pod in self.get_pod_list() {
            if let Some(pod_name) = pod.metadata.name.as_ref() {
                if pod_name == &current_pod {
                    continue;
                }
            }
            let pod_name = pod.metadata.name.as_ref().unwrap();
            let res = self.send_delta(pod_name, &crdt_type, &delta).await;
            println!("Broadcasted delta to {}, with crdt type {}, with delta - {}, return with result of {:?}", pod_name, crdt_type, delta, res.into_response());
        }
        (StatusCode::OK, "Broadcasted delta")
    }
}

#[debug_handler(state = AppState)]
pub async fn get_operation(
    State(mut app_state): State<AppState>,
    Path((crdt_type, operation)): Path<(String, String)>,
    Json(message): Json<Message>,
) {
    {
        let crdt = app_state.get_or_create_crdt(crdt_type.clone());
        let mut crdt = crdt.write().unwrap();
        let text_operation = match operation.as_str() {
            "insert" => TextOperation::Insert {
                position: message.position,
                value: message.text.clone(),
            },
            "delete" => TextOperation::Delete {
                position: message.position,
                value: message.text.clone(),
            },
            _ => TextOperation::Insert {
                position: message.position,
                value: message.text.clone(),
            },
        };
        let ops = crdt.convert_operation(text_operation);
        for op in ops.iter() {
            crdt.apply_operation(op.clone());
        }
    }
    app_state
        .broadcast_operation(crdt_type, operation, &message)
        .await;
}

#[debug_handler(state = AppState)]
pub async fn get_state(
    State(mut app_state): State<AppState>,
    Path((crdt_type, state)): Path<(String, String)>,
) {
    {
        let crdt = app_state.get_or_create_crdt(crdt_type.clone());
        let mut crdt = crdt.write().unwrap();
        let mut crdt_1 = DataType::new(crdt.name());
        crdt_1.to_crdt(state.clone());
        crdt.merge(&crdt_1);
    }
    app_state.broadcast_state(crdt_type, state).await;
}

#[debug_handler(state = AppState)]
pub async fn get_delta(
    State(mut app_state): State<AppState>,
    Path((crdt_type, delta)): Path<(String, String)>,
) {
    {
        let crdt = app_state.get_or_create_crdt(crdt_type.clone());
        let mut crdt = crdt.write().unwrap();
        let mut crdt_1 = DataType::new(crdt.name());
        let crdt_delta = crdt_1.to_delta(delta.clone());
        crdt.apply_delta(&crdt_delta);
    }
    app_state.broadcast_delta(crdt_type, delta).await;
}
