use axum::{extract::Path, Json};
use crust_core::text_operation::Message;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, ListParams, ObjectList},
    Client as KubeClient,
};
use reqwest::{Client, Response};

pub struct TestController {
    client: Client,
    pod_list: ObjectList<Pod>,
}

impl TestController {
    pub async fn new() -> Self {
        TestController {
            client: Client::new(),
            pod_list: {
                let kube_client = KubeClient::try_default().await.unwrap();
                let pods: Api<Pod> = Api::namespaced(kube_client, "default");
                let lp = ListParams::default().labels("app=crdt-service");
                let pod_list = pods.list(&lp).await.unwrap();
                pod_list
            },
        }
    }

    pub async fn send_operation_across_all_replicas(
        &self,
        Path((crdt_type, operation)): Path<(String, String)>,
        Json(message): Json<Message>,
    ) {
        for pod in self.pod_list.clone().items {
            self.send_to_single_pod(
                &pod.metadata.name.unwrap(),
                &crdt_type,
                &operation,
                &message,
            )
            .await;
        }
    }

    async fn send_to_single_pod(
        &self,
        pod_name: &str,
        crdt_type: &str,
        operation: &str,
        message: &Message,
    ) -> Response {
        let host = format!("{}.crdt-service.default.svc.cluster.local", pod_name);
        let url = format!("http://{}/crdt/{}/operation/{}", host, crdt_type, operation);
        self.client.get(&url).json(message).send().await.unwrap()
    }

    async fn get_pod_state(&self, pod_name: &str) -> String {
        let host = format!("{}.crdt-service.default.svc.cluster.local", pod_name);
        let url = format!("http://{}/crdt/state", host);
        self.client
            .get(&url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub async fn reset_all_pods(&self) {
        let kube_client = KubeClient::try_default().await.unwrap();
        let pods: Api<Pod> = Api::namespaced(kube_client, "default");
        for pod in self.pod_list.clone().items {
            if let Some(name) = pod.metadata.name {
                pods.delete(&name, &DeleteParams::default()).await.unwrap();
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    // cmrdt

    pub async fn test_cmrdt_semilattice(
        &self,
        Path((crdt_type, operation)): Path<(String, String)>,
    ) -> bool {
        self.test_cmrdt_associative(crdt_type.clone(), operation.clone())
            .await
            && self.test_cmrdt_commutative(crdt_type.clone(), operation.clone())
            && self.test_cmrdt_idempotent(crdt_type.clone(), operation.clone())
    }

    pub async fn test_cmrdt_associative(&self, crdt_type: String, operation: String) -> bool {
        // Setup test messages
        let a = Message {
            position: 0,
            text: "a".to_string(),
        };
        let b = Message {
            position: 1,
            text: "b".to_string(),
        };
        let c = Message {
            position: 2,
            text: "c".to_string(),
        };

        // Get two test pods
        let pods: Vec<String> = self
            .pod_list
            .items
            .iter()
            .take(2)
            .map(|pod| pod.metadata.name.clone().unwrap())
            .collect();

        // Reset both pods
        self.reset_all_pods().await;

        // Pod 1: Compute (a • b) • c
        self.send_to_single_pod(&pods[0], &crdt_type, &operation, &a)
            .await;
        self.send_to_single_pod(&pods[0], &crdt_type, &operation, &b)
            .await;
        self.send_to_single_pod(&pods[0], &crdt_type, &operation, &c)
            .await;

        // Pod 2: Compute a • (b • c)
        self.send_to_single_pod(&pods[1], &crdt_type, &operation, &b)
            .await;
        self.send_to_single_pod(&pods[1], &crdt_type, &operation, &c)
            .await;
        self.send_to_single_pod(&pods[1], &crdt_type, &operation, &a)
            .await;

        // Compare final states
        let state1 = self.get_pod_state(&pods[0]).await;
        let state2 = self.get_pod_state(&pods[1]).await;

        state1 == state2
    }

    pub fn test_cmrdt_commutative(&self, crdt_type: String, operation: String) -> bool {
        false
    }

    pub fn test_cmrdt_idempotent(&self, crdt_type: String, operation: String) -> bool {
        false
    }

    // cvrdt

    pub fn test_cvrdt_semilattice(&self) -> bool {
        self.test_cvrdt_associative()
            && self.test_cvrdt_commutative()
            && self.test_cvrdt_idempotent()
    }

    pub fn test_cvrdt_associative(&self) -> bool {
        false
    }

    pub fn test_cvrdt_commutative(&self) -> bool {
        false
    }

    pub fn test_cvrdt_idempotent(&self) -> bool {
        false
    }

    // delta

    pub fn test_delta_semilattice(&self) -> bool {
        self.test_delta_associative()
            && self.test_delta_commutative()
            && self.test_delta_idempotent()
    }

    pub fn test_delta_associative(&self) -> bool {
        false
    }

    pub fn test_delta_commutative(&self) -> bool {
        false
    }

    pub fn test_delta_idempotent(&self) -> bool {
        false
    }
}
