use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
    time::Duration,
};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use crust_config::{
    instance::{setup_remote_test_environement, DeploymentConfig},
    k8s_discovery::get_replica_pod_names,
};
use crust_core::{
    command::CrdtInnerCommand,
    r#type::CrdtType,
    security,
    sync::{SyncConfig, SyncMode, SyncType},
};
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::json;

use crate::{
    get_current_pod_name, get_current_service_name, message::NetworkMessage, sender::NetworkSender,
};

#[derive(Clone)]
pub struct AppState {
    crdt_type: Arc<RwLock<Option<CrdtType<String>>>>,
}

impl AppState {
    pub fn new() -> Self {
        let crdt_type = Arc::new(RwLock::new(CrdtType::new("none".to_string())));
        Self { crdt_type }
    }

    pub fn get_crdt_type(&self) -> CrdtType<String> {
        self.crdt_type.read().unwrap().clone().unwrap()
    }

    pub fn set_crdt_type(&mut self, crdt_type: String) {
        let crdt_type = CrdtType::new(crdt_type);
        self.crdt_type = Arc::new(RwLock::new(crdt_type));
    }

    pub fn get_or_create_crdt_type(&mut self, crdt_type: String) -> CrdtType<String> {
        if self.crdt_type.read().unwrap().clone().unwrap().name() == crdt_type {
            return self.get_crdt_type();
        }
        self.set_crdt_type(crdt_type);
        self.get_crdt_type()
    }
}

#[debug_handler(state = AppState)]
pub async fn receive_message_from_other_instances(
    State(mut state): State<AppState>,
    Path(crdt_type): Path<String>,
    Json(message): Json<NetworkMessage<String>>,
) -> impl IntoResponse
where
    NetworkMessage<String>: Debug,
    CrdtType<String>: Clone,
{
    let mut crdt = state.get_or_create_crdt_type(crdt_type);

    #[cfg(any(
        feature = "byzantine",
        feature = "confidentiality",
        feature = "integrity",
        feature = "access_control"
    ))]
    let security = state.security.clone();

    #[cfg(feature = "integrity")]
    let message = security.sign_data(message);

    #[cfg(feature = "confidentiality")]
    let message = security.encrypt_data(message);

    match &message {
        NetworkMessage::Operation {
            payload,
            sender_pod_name,
        } => {
            if *sender_pod_name != get_current_pod_name() {
                crdt.apply(payload);
            }
        }
        NetworkMessage::Delta {
            payload,
            sender_pod_name,
        } => {
            if *sender_pod_name != get_current_pod_name() {
                crdt.merge_delta(payload);
            }
        }
        NetworkMessage::State {
            payload,
            sender_pod_name,
        } => {
            if *sender_pod_name != get_current_pod_name() {
                crdt.merge(payload);
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({"message":format!("{:?}", message)})),
    )
}

#[debug_handler(state = AppState)]
pub async fn receive_message_from_internal(
    State(mut state): State<AppState>,
    Path((crdt_type, sync_type, sync_mode)): Path<(String, String, String)>,
    Json(command): Json<CrdtInnerCommand<String>>,
) -> impl IntoResponse
where
    NetworkMessage<String>: Debug + Serialize,
    CrdtType<String>: Clone,
{
    let mut crdt = state.get_or_create_crdt_type(crdt_type.clone());

    

    let test_config = DeploymentConfig::new(
        3,
        crdt_type.as_str(),
        sync_type.as_str(),
        sync_mode.as_str(),
        None,
        None,
    );

    let _ = setup_remote_test_environement(&test_config).await;

    let sender = NetworkSender::new(
        get_current_pod_name(),
        get_current_service_name(),
        get_replica_pod_names().await.unwrap(),
    );

    

    let mut sync_config = SyncConfig {
        sync_type: SyncType::new(sync_type),
        sync_mode: SyncMode::new(sync_mode),
        #[cfg(feature = "batch")]
        batch_times: Some(3), 
        #[cfg(feature = "batch")]
        batching_interval: Some(Duration::from_secs(5)), 
        #[cfg(feature = "batch")]
        last_batch_check_timestamp: None, 
    };

    let message_option = handle_sync_message(
        &mut crdt,
        &command,
        &mut sync_config,
        get_current_pod_name(),
    )
    .await;

    if let Some(message) = message_option {
        sender.broadcast_message(&message);
        (
            StatusCode::OK,
            Json(json!({"message":format!("{:?}", message)})),
        )
    } else {
        (
            StatusCode::OK,
            Json(json!({"message":"No operation to sync"})), 
        )
    }
}

async fn handle_sync_message(
    crdt: &mut CrdtType<String>,
    command: &CrdtInnerCommand<String>,
    sync_config: &mut SyncConfig,
    pod_name: String,
) -> Option<NetworkMessage<String>>
where
    CrdtType<String>: Clone,
{
    let operation = crdt.apply_command(command);
    match sync_config.sync_mode {
        SyncMode::Immediate => match sync_config.sync_type {
            SyncType::Delta => Some(NetworkMessage::Delta {
                payload: crdt.generate_delta(),
                sender_pod_name: pod_name,
            }),
            SyncType::Operation => {
                if let Some(operation) = operation {
                    Some(NetworkMessage::Operation {
                        payload: operation,
                        sender_pod_name: pod_name,
                    })
                } else {
                    None
                }
            }
            SyncType::State => Some(NetworkMessage::State {
                payload: crdt.clone(),
                sender_pod_name: pod_name,
            }),
        },
        #[cfg(feature = "batch")]
        SyncMode::BatchTimeBased => match sync_config.sync_type {
            SyncType::Delta => {
                if let Some(delta) = crdt.generate_delta_time_based(sync_config) {
                    Some(NetworkMessage::Delta {
                        payload: delta,
                        sender_pod_name: pod_name,
                    })
                } else {
                    None
                }
            }
            SyncType::Operation => {
                if let Some(operation) = crdt.generate_operation_time_based(sync_config) {
                    Some(NetworkMessage::Operation {
                        payload: operation,
                        sender_pod_name: pod_name,
                    })
                } else {
                    None
                }
            }
            SyncType::State => Some(NetworkMessage::State {
                payload: crdt.clone(),
                sender_pod_name: pod_name,
            }),
        },
        #[cfg(feature = "batch")]
        SyncMode::BatchCountBased => match sync_config.sync_type {
            SyncType::Delta => {
                if let Some(delta) = crdt.generate_delta_count_based(sync_config) {
                    Some(NetworkMessage::Delta {
                        payload: delta,
                        sender_pod_name: pod_name,
                    })
                } else {
                    None
                }
            }
            SyncType::Operation => {
                if let Some(operation) = crdt.generate_operation_count_based(sync_config) {
                    Some(NetworkMessage::Operation {
                        payload: operation,
                        sender_pod_name: pod_name,
                    })
                } else {
                    None
                }
            }
            SyncType::State => Some(NetworkMessage::State {
                payload: crdt.clone(),
                sender_pod_name: pod_name,
            }),
        },
    }
}

#[debug_handler(state = AppState)]
pub async fn get_state(
    State(mut state): State<AppState>,
    Path(crdt_type): Path<String>,
) -> impl IntoResponse
where
    CrdtType<String>: Clone + Serialize,
{
    let crdt = state.get_or_create_crdt_type(crdt_type);
    (StatusCode::OK, Json(json!({"state": crdt.get_state()})))
}
