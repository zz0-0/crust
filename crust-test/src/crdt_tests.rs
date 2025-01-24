use std::time::Duration;

use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, ListParams, ObjectList},
    Client as KubeClient,
};

enum CRDTMode {
    CvRDT,
    CmRDT,
    Delta,
}

struct CRDTTest {
    replicas: Vec<Pod>,
    crdt_type: String,
    crdt_mode: CRDTMode,
    // metrics: TestMetrics,
}

// struct TestMetrics {
//     convergence_time: Duration,
//     size: usize,
//     throughput: f64,
//     memory_usage: f64,
//     cpu_usage: f64,
// }

// struct TestResults {
//     avg_latency: Duration,
//     max_throughput: f64,
//     max_memory_usage: f64,
//     max_cpu_usage: f64,
//     network_bandwidth: f64,
// }

impl CRDTTest {
    async fn setup_k8s_environment(&self) -> Vec<Pod> {
        let kube_client = KubeClient::try_default().await.unwrap();
        let pods: Api<Pod> = Api::namespaced(kube_client, "default");
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

        if running_pods.len() < 2 {
            panic!(
                "Not enough running pods. Found {}, needed 2",
                running_pods.len()
            );
        }

        running_pods
    }

    async fn test_cvrdt(&self) {
        self.test_cvrdt_associativity().await;
        self.test_cvrdt_commutativity().await;
        self.test_cvrdt_idempotence().await;
        self.test_cvrdt_convergence().await;
    }

    async fn test_cvrdt_associativity(&self) {}

    async fn test_cvrdt_commutativity(&self) {}

    async fn test_cvrdt_idempotence(&self) {}

    async fn test_cvrdt_convergence(&self) {}

    async fn test_cmrdt(&self) {
        self.test_cmrdt_commutativity().await;
        self.test_cmrdt_idempotence().await;
        self.test_cmrdt_sequential_consistency().await;
        self.test_cmrdt_convergence().await;
    }

    async fn test_cmrdt_commutativity(&self) {}

    async fn test_cmrdt_idempotence(&self) {}

    async fn test_cmrdt_sequential_consistency(&self) {}

    async fn test_cmrdt_convergence(&self) {}

    async fn test_delta(&self) {
        self.test_delta_associativity().await;
        self.test_delta_commutativity().await;
        self.test_delta_idempotence().await;
        self.test_delta_convergence().await;
    }

    async fn test_delta_associativity(&self) {}

    async fn test_delta_commutativity(&self) {}

    async fn test_delta_idempotence(&self) {}

    async fn test_delta_convergence(&self) {}
}
