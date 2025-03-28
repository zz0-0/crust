use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::hash::Hash;

use crate::message::NetworkMessage;
use crate::PORT;

pub struct NetworkSender {
    client: Client,
    replica_pod_name: String,
    replica_service_name: String,
    replica_pod_names: Vec<String>,
    #[cfg(any(
        feature = "byzantine",
        feature = "confidentiality",
        feature = "integrity",
        feature = "access_control"
    ))]
    security: Option<Box<dyn SecurityHook<K> + Send + Sync>>,
}

impl NetworkSender {
    pub fn new(
        replica_pod_name: String,
        replica_service_name: String,
        replica_pod_names: Vec<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            replica_pod_name,
            replica_service_name,
            replica_pod_names,
        }
    }

    pub async fn send_message<K>(&self, url: String, message: &NetworkMessage<K>) -> Response
    where
        NetworkMessage<K>: Serialize,
        K: Eq + Hash,
    {
        #[cfg(feature = "integrity")]
        let message = self.security.sign_data(message);

        #[cfg(feature = "confidentiality")]
        let message = self.security.encrypt_data(message);

        let res = self.client.get(url).json(message).send();
        let request_response = match res.await {
            Ok(res) => res,
            Err(_) => return (StatusCode::BAD_REQUEST, Body::empty()).into_response(),
        };
        let mut response_builder = Response::builder().status(request_response.status());
        *response_builder.headers_mut().unwrap() = request_response.headers().clone();
        let body = Body::from(request_response.bytes().await.unwrap());
        response_builder.body(body).unwrap()
    }

    pub fn broadcast_message<K>(&self, message: &NetworkMessage<K>)
    where
        NetworkMessage<K>: Serialize,
        K: Eq + Hash,
    {
        for pod_name in &self.replica_pod_names {
            let url = format!(
                "http://{pod_name}.{service_name}.default.svc.cluster.local:{PORT}/receive",
                pod_name = pod_name,
                service_name = self.replica_service_name,
            );
            if pod_name == &self.replica_pod_name {
                continue;
            }
            let _ = self.send_message(url, message);
        }
    }
}
