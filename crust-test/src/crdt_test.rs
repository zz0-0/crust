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
        let kube_client = KubeClient::try_default().await.unwrap();
        let pods: Api<Pod> = Api::namespaced(kube_client, "default");
        let lp = ListParams::default().labels("app=crust-http");

        // Debug pod list before filtering
        let pod_list = pods.list(&lp).await.unwrap();
        println!("Total pods found: {}", pod_list.items.len());

        // Debug each pod's status
        for pod in &pod_list.items {
            println!("Pod: {:?}", pod.metadata.name);
            println!("Status: {:?}", pod.status.as_ref().map(|s| &s.phase));
        }

        // Only include Running pods
        let running_pods: Vec<_> = pod_list
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
            .collect();

        println!("Running pods: {}", running_pods.len());

        if running_pods.len() < 2 {
            panic!(
                "Not enough running pods. Found {}, needed 2",
                running_pods.len()
            );
        }

        TestController {
            client: Client::new(),
            pod_list,
        }
    }

    pub async fn send_operation_across_all_replicas(
        &self,
        Path((crdt_type, operation)): Path<(String, String)>,
        Json(message): Json<Message>,
    ) {
        for pod in self.pod_list.clone().items {
            self.send_operation_to_single_pod(
                &pod.metadata.name.unwrap(),
                &crdt_type,
                &operation,
                &message,
            )
            .await;
        }
    }

    async fn send_operation_to_single_pod(
        &self,
        pod_name: &str,
        crdt_type: &str,
        operation: &str,
        message: &Message,
    ) -> Response {
        let pod_ip = self
            .pod_list
            .items
            .iter()
            .find(|pod| {
                pod.metadata
                    .name
                    .as_ref()
                    .map_or(false, |name| name == pod_name)
            })
            .and_then(|pod| pod.status.as_ref())
            .and_then(|status| status.pod_ip.as_ref())
            .expect("Pod IP not found");
        let url = format!(
            "http://{}:3000/crust/{}/operation/{}",
            pod_ip, crdt_type, operation
        );
        self.client.get(&url).json(message).send().await.unwrap()
    }

    fn get_available_pods(&self, count: usize) -> Vec<String> {
        let pods = self
            .pod_list
            .items
            .iter()
            .filter_map(|pod| pod.metadata.name.clone())
            .take(count)
            .collect::<Vec<_>>();

        if pods.len() < count {
            panic!(
                "Not enough pods available. Found {} pods, needed {}",
                pods.len(),
                count
            );
        }

        pods
    }

    async fn get_pod_state(&self, pod_name: &str) -> String {
        let pod_ip = self
            .pod_list
            .items
            .iter()
            .find(|pod| {
                pod.metadata
                    .name
                    .as_ref()
                    .map_or(false, |name| name == pod_name)
            })
            .and_then(|pod| pod.status.as_ref())
            .and_then(|status| status.pod_ip.as_ref())
            .expect("Pod IP not found");
        let url = format!("http://{}:3000/crust/info", pod_ip);
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
    ) -> Json<bool> {
        axum::Json(
            self.test_cmrdt_associative(crdt_type.clone(), operation.clone())
                .await
                && self
                    .test_cmrdt_commutative(crdt_type.clone(), operation.clone())
                    .await
                && self
                    .test_cmrdt_idempotent(crdt_type.clone(), operation.clone())
                    .await,
        )
    }

    pub async fn test_cmrdt_associative(&self, crdt_type: String, operation: String) -> bool {
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

        let pods = self.get_available_pods(2);

        self.reset_all_pods().await;

        self.send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &a)
            .await;
        self.send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &b)
            .await;
        self.send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &c)
            .await;

        self.send_operation_to_single_pod(&pods[1], &crdt_type, &operation, &b)
            .await;
        self.send_operation_to_single_pod(&pods[1], &crdt_type, &operation, &c)
            .await;
        self.send_operation_to_single_pod(&pods[1], &crdt_type, &operation, &a)
            .await;

        let result1 = self.get_pod_state(&pods[0]).await;
        let result2 = self.get_pod_state(&pods[1]).await;

        result1 == result2
    }

    pub async fn test_cmrdt_commutative(&self, crdt_type: String, operation: String) -> bool {
        let a = Message {
            position: 0,
            text: "a".to_string(),
        };
        let b = Message {
            position: 1,
            text: "b".to_string(),
        };

        let pods = self.get_available_pods(2);

        self.reset_all_pods().await;

        self.send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &a)
            .await;
        self.send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &b)
            .await;

        self.send_operation_to_single_pod(&pods[1], &crdt_type, &operation, &b)
            .await;
        self.send_operation_to_single_pod(&pods[1], &crdt_type, &operation, &a)
            .await;

        let result1 = self.get_pod_state(&pods[0]).await;
        let result2 = self.get_pod_state(&pods[1]).await;

        result1 == result2
    }

    pub async fn test_cmrdt_idempotent(&self, crdt_type: String, operation: String) -> bool {
        let a = Message {
            position: 0,
            text: "a".to_string(),
        };

        let pods = self.get_available_pods(1);

        self.reset_all_pods().await;

        self.send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &a)
            .await;

        let result1 = self.get_pod_state(&pods[0]).await;

        self.send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &a)
            .await;

        let result2 = self.get_pod_state(&pods[0]).await;

        result1 == result2
    }

    // cvrdt

    pub async fn test_cvrdt_semilattice(
        &self,
        Path((crdt_type, operation)): Path<(String, String)>,
    ) -> Json<bool> {
        axum::Json(
            self.test_cvrdt_associative().await
                && self.test_cvrdt_commutative().await
                && self.test_cvrdt_idempotent().await,
        )
    }

    pub async fn test_cvrdt_associative(&self) -> bool {
        false
    }

    pub async fn test_cvrdt_commutative(&self) -> bool {
        false
    }

    pub async fn test_cvrdt_idempotent(&self) -> bool {
        false
    }

    // delta

    pub async fn test_delta_semilattice(
        &self,
        Path((crdt_type, operation)): Path<(String, String)>,
    ) -> Json<bool> {
        axum::Json(
            self.test_delta_associative().await
                && self.test_delta_commutative().await
                && self.test_delta_idempotent().await,
        )
    }

    pub async fn test_delta_associative(&self) -> bool {
        false
    }

    pub async fn test_delta_commutative(&self) -> bool {
        false
    }

    pub async fn test_delta_idempotent(&self) -> bool {
        false
    }
}
