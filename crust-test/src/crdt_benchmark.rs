// use axum::{extract::Path, Json};
// use crust_core::text_operation::Message;
// use k8s_openapi::api::core::v1::Pod;
// use kube::{
//     api::{ListParams, ObjectList},
//     Api, Client as KubeClient,
// };
// use reqwest::{Client, Response};

// pub struct BenchmarkController {
//     client: Client,
//     pod_list: ObjectList<Pod>,
// }

// impl BenchmarkController {
//     pub async fn new() -> Self {
//         let kube_client = KubeClient::try_default().await.unwrap();
//         let pods: Api<Pod> = Api::namespaced(kube_client, "default");
//         let lp = ListParams::default().labels("app=crust-http");

//         let pod_list = pods.list(&lp).await.unwrap();
//         println!("Total pods found: {}", pod_list.items.len());

//         for pod in &pod_list.items {
//             println!("Pod: {:?}", pod.metadata.name);
//             println!("Status: {:?}", pod.status.as_ref().map(|s| &s.phase));
//         }

//         let running_pods: Vec<_> = pod_list
//             .items
//             .iter()
//             .filter(|pod| {
//                 if let Some(status) = &pod.status {
//                     if let Some(phase) = &status.phase {
//                         return phase == "Running";
//                     }
//                 }
//                 false
//             })
//             .collect();

//         println!("Running pods: {}", running_pods.len());

//         if running_pods.len() < 1 {
//             panic!(
//                 "Not enough running pods. Found {}, needed 1",
//                 running_pods.len()
//             );
//         }

//         Self {
//             client: Client::new(),
//             pod_list,
//         }
//     }

//     pub async fn benchmark_cmrdt(
//         &self,
//         Path((crdt_type, operation)): Path<(String, String)>,
//         Json(message): Json<Message>,
//     ) {
//         let a = Message {
//             position: 0,
//             text: "a".to_string(),
//         };
//         let pods = self.get_available_pods(1);
//         let response = self
//             .send_operation_to_single_pod(&pods[0], &crdt_type, &operation, &message)
//             .await;
//         // let time = response.json().into();
//     }

//     pub async fn benchmark_cvrdt(&self) {}

//     pub async fn benchmark_delta(&self) {}

//     async fn send_operation_to_single_pod(
//         &self,
//         pod_name: &str,
//         crdt_type: &str,
//         operation: &str,
//         message: &Message,
//     ) -> Response {
//         let pod_ip = self
//             .pod_list
//             .items
//             .iter()
//             .find(|pod| {
//                 pod.metadata
//                     .name
//                     .as_ref()
//                     .map_or(false, |name| name == pod_name)
//             })
//             .and_then(|pod| pod.status.as_ref())
//             .and_then(|status| status.pod_ip.as_ref())
//
//         let url = format!(
//             "http://{}:3000/crust/{}/operation/{}/time",
//             pod_ip, crdt_type, operation
//         );
//         self.client.get(&url).json(message).send().await.unwrap()
//     }

//     async fn send_state_to_single_pod(&self, pod_name: &str, state: &str) -> Response {
//         let pod_ip = self
//             .pod_list
//             .items
//             .iter()
//             .find(|pod| {
//                 pod.metadata
//                     .name
//                     .as_ref()
//                     .map_or(false, |name| name == pod_name)
//             })
//             .and_then(|pod| pod.status.as_ref())
//             .and_then(|status| status.pod_ip.as_ref())
//
//         let url = format!("http://{}:3000/crust/state/time", pod_ip);
//         self.client.get(&url).json(state).send().await.unwrap()
//     }

//     fn get_available_pods(&self, count: usize) -> Vec<String> {
//         let pods = self
//             .pod_list
//             .items
//             .iter()
//             .filter_map(|pod| pod.metadata.name.clone())
//             .take(count)
//             .collect::<Vec<_>>();

//         if pods.len() < count {
//             panic!(
//                 "Not enough pods available. Found {} pods, needed {}",
//                 pods.len(),
//                 count
//             );
//         }

//         pods
//     }
// }
