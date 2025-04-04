use chrono::{DateTime, Utc};
use crust_core::operation::CrdtOperation;
use crust_core::r#type::CrdtType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    name: String,
    permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub action: String,
    pub resource: String,
    pub success: bool,
    pub details: Option<String>,
    pub client_ip: Option<String>,
}

pub struct AccessControl {
    users: HashMap<String, UserInfo>,
    roles: HashMap<String, Role>,

    resource_permissions: HashMap<String, HashMap<String, Vec<Permission>>>,

    audit_logs: Arc<Mutex<VecDeque<AuditLogEntry>>>,
    max_audit_log_size: usize,

    sessions: HashMap<String, SessionInfo>,
}

#[derive(Debug, Clone)]
struct UserInfo {
    user_id: String,
    role_id: String,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct SessionInfo {
    session_id: String,
    user_id: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    is_active: bool,
}

impl AccessControl {
    pub fn new() -> Self {
        let mut access_control = AccessControl {
            users: HashMap::new(),
            roles: HashMap::new(),
            resource_permissions: HashMap::new(),
            audit_logs: Arc::new(Mutex::new(VecDeque::new())),
            max_audit_log_size: 10000,
            sessions: HashMap::new(),
        };

        access_control.initialize_default_roles();

        access_control.add_user("admin", "admin_role", None);
        access_control.add_user("user", "user_role", None);
        access_control.add_user("guest", "guest_role", None);

        access_control
    }

    fn initialize_default_roles(&mut self) {
        let admin_role = Role {
            name: "admin_role".to_string(),
            permissions: vec![Permission::Read, Permission::Write, Permission::Admin],
        };

        let user_role = Role {
            name: "user_role".to_string(),
            permissions: vec![Permission::Read, Permission::Write],
        };

        let guest_role = Role {
            name: "guest_role".to_string(),
            permissions: vec![Permission::Read],
        };

        self.roles.insert("admin_role".to_string(), admin_role);
        self.roles.insert("user_role".to_string(), user_role);
        self.roles.insert("guest_role".to_string(), guest_role);
    }

    pub fn add_user(
        &mut self,
        user_id: &str,
        role_id: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> bool {
        if !self.roles.contains_key(role_id) {
            return false;
        }

        let user_info = UserInfo {
            user_id: user_id.to_string(),
            role_id: role_id.to_string(),
            metadata: metadata.unwrap_or_default(),
        };

        self.users.insert(user_id.to_string(), user_info);
        true
    }

    pub fn remove_user(&mut self, user_id: &str) -> bool {
        self.users.remove(user_id).is_some()
    }

    pub fn add_role(&mut self, role_name: &str, permissions: Vec<Permission>) -> bool {
        let role = Role {
            name: role_name.to_string(),
            permissions,
        };

        self.roles.insert(role_name.to_string(), role);
        true
    }

    pub fn set_resource_permissions(
        &mut self,
        resource_id: &str,
        role_id: &str,
        permissions: Vec<Permission>,
    ) -> bool {
        if !self.roles.contains_key(role_id) {
            return false;
        }

        self.resource_permissions
            .entry(resource_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(role_id.to_string(), permissions);

        true
    }

    pub fn create_session(&mut self, user_id: &str, duration_seconds: u64) -> Option<String> {
        if !self.users.contains_key(user_id) {
            return None;
        }

        let session_id = format!("sess_{}", uuid::Uuid::new_v4());
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(duration_seconds as i64);

        let session = SessionInfo {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at,
            is_active: true,
        };

        self.sessions.insert(session_id.clone(), session);

        Some(session_id)
    }

    pub fn validate_session(&self, session_id: &str) -> Option<String> {
        if let Some(session) = self.sessions.get(session_id) {
            if session.is_active && session.expires_at > Utc::now() {
                return Some(session.user_id.clone());
            }
        }
        None
    }

    pub fn invalidate_session(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.is_active = false;
            return true;
        }
        false
    }

    pub fn check_access(
        &self,
        user_id: &str,
        resource_id: &str,
        required_permission: &Permission,
    ) -> bool {
        let role_id = match self.users.get(user_id) {
            Some(user) => &user.role_id,
            None => return false,
        };

        if let Some(role) = self.roles.get(role_id) {
            if role.permissions.contains(required_permission) {
                if role.permissions.contains(&Permission::Admin) {
                    return true;
                }

                if let Some(resource_perms) = self.resource_permissions.get(resource_id) {
                    if let Some(role_perms) = resource_perms.get(role_id) {
                        return role_perms.contains(required_permission);
                    }
                }

                return true;
            }
        }

        false
    }

    pub fn log_access(
        &self,
        user_id: &str,
        action: &str,
        resource: &str,
        success: bool,
        details: Option<String>,
        client_ip: Option<String>,
    ) {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            user_id: user_id.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
            success,
            details,
            client_ip,
        };

        if let Ok(mut logs) = self.audit_logs.lock() {
            logs.push_back(entry);

            while logs.len() > self.max_audit_log_size {
                logs.pop_front();
            }
        }
    }

    pub fn get_audit_logs(&self, limit: usize) -> Vec<AuditLogEntry> {
        if let Ok(logs) = self.audit_logs.lock() {
            logs.iter().rev().take(limit).cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn check_access_security<K>(&self, data: &CrdtType<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        let user_id = "user";
        let resource_id = format!("crdt:{}", data.name());
        let required_permission = Permission::Read;

        let access_granted = self.check_access(user_id, &resource_id, &required_permission);

        self.log_access(
            user_id,
            "read",
            &resource_id,
            access_granted,
            Some(format!("Access to CRDT type {}", data.name())),
            None,
        );

        access_granted
    }

    pub fn check_operation_access<K>(&self, operation: &CrdtOperation<K>, user_id: &str) -> bool
    where
        K: Eq + Hash + Clone,
    {
        let (action, required_permission) = match operation {
            CrdtOperation::Counter(_) => ("increment", Permission::Write),
            CrdtOperation::Register(_) => ("update", Permission::Write),
            CrdtOperation::Set(set_op) => match &set_op.action {
                crust_core::operation::SetAction::Add(_) => ("add", Permission::Write),
                crust_core::operation::SetAction::Remove(_) => ("remove", Permission::Write),
                _ => ("unknown", Permission::Write),
            },
            _ => ("unknown", Permission::Write),
        };

        let resource_id = format!("crdt:{}", operation.crdt_type());

        let access_granted = self.check_access(user_id, &resource_id, &required_permission);

        self.log_access(
            user_id,
            action,
            &resource_id,
            access_granted,
            Some(format!("Operation on CRDT type {}", operation.crdt_type())),
            None,
        );

        access_granted
    }

    pub fn audit_log<K>(&self, data: &CrdtType<K>)
    where
        K: Eq + Hash + Clone,
    {
        let user_id = "system";
        let action = "access";
        let resource_id = format!("crdt:{}", data.name());

        self.log_access(
            user_id,
            action,
            &resource_id,
            true,
            Some(format!("Automatic audit log for CRDT {}", data.name())),
            None,
        );
    }
}
