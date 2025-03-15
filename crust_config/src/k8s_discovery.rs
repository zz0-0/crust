use k8s_openapi::api::core::v1::Pod;
use kube::{api::ListParams, Api, Client, Error};

pub async fn get_replica_pod_names() -> Result<Vec<String>, Error> {
    let client = Client::try_default().await?;
    let endpoints_api: Api<Pod> = Api::namespaced(client.clone(), "default");
    let lp = ListParams::default().labels("app=crust-network");
    let pod_list = endpoints_api.list(&Default::default()).await?;
    let replica_pod_names = pod_list
        .iter()
        .map(|pod| {
            pod.metadata
                .name
                .clone()
                .unwrap_or_else(|| "none".to_string())
        })
        .collect();
    Ok(replica_pod_names)
}
