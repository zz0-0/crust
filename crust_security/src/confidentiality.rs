use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use crust_core::r#type::CrdtType;
use serde_json::{json, Value};
use std::hash::Hash;

pub struct ConfidentialitySecurity {
    
    encryption_enabled: bool,
    key: Key<Aes256Gcm>,
    algorithm: EncryptionAlgorithm,
}

#[derive(Clone, Copy)]
pub enum EncryptionAlgorithm {
    AesGcm256,
    
}

impl ConfidentialitySecurity {
    pub fn new() -> Self {
        ConfidentialitySecurity {
            encryption_enabled: true,
            key: Aes256Gcm::generate_key(OsRng),
            algorithm: EncryptionAlgorithm::AesGcm256,
        }
    }

    pub fn with_encryption(enabled: bool) -> Self {
        let mut security = Self::new();
        security.encryption_enabled = enabled;
        security
    }

    pub fn with_key(mut self, key_data: &[u8]) -> Self {
        if key_data.len() == 32 {
            
            let mut key_bytes = [0u8; 32];
            key_bytes.copy_from_slice(key_data);
            self.key = Key::<Aes256Gcm>::from(key_bytes);
        }
        self
    }

    pub fn get_key_bytes(&self) -> [u8; 32] {
        *self.key.as_ref()
    }

    pub fn encrypt_data<K>(&self, data: &CrdtType<K>) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        if !self.encryption_enabled {
            return data.clone();
        }

        
        let state_json = data.get_state();

        
        match data.name().as_str() {
            "gcounter" => self.encrypt_gcounter_data(data, state_json),
            "pncounter" => self.encrypt_pncounter_data(data, state_json),
            "lwwregister" => self.encrypt_lwwregister_data(data, state_json),
            "orset" => self.encrypt_orset_data(data, state_json),
            _ => data.clone(), 
        }
    }

    fn encrypt_gcounter_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let Some(counter_map) = state_json.get("state").and_then(|v| v.as_object()) {
            let mut encrypted_state = json!({
                "state": {},
                "__meta": {
                    "encrypted": true,
                    "algorithm": "AES-GCM-256",
                }
            });

            
            let encrypted_map = encrypted_state["state"].as_object_mut().unwrap();
            for (key, value) in counter_map {
                if let Some(count) = value.as_u64() {
                    
                    let count_bytes = count.to_be_bytes();

                    
                    if let Ok(encrypted_bytes) = self.encrypt_bytes(&count_bytes) {
                        
                        let base64_value = general_purpose::STANDARD.encode(&encrypted_bytes);
                        encrypted_map.insert(key.clone(), json!(base64_value));
                    }
                }
            }

            
            data.with_state(encrypted_state)
        } else {
            data.clone()
        }
    }

    fn encrypt_pncounter_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let (Some(p_map), Some(n_map)) = (
            state_json.get("p").and_then(|v| v.as_object()),
            state_json.get("n").and_then(|v| v.as_object()),
        ) {
            let mut encrypted_state = json!({
                "p": {},
                "n": {},
                "__meta": {
                    "encrypted": true,
                    "algorithm": "AES-GCM-256",
                }
            });

            
            let p_encrypted = encrypted_state["p"].as_object_mut().unwrap();
            for (key, value) in p_map {
                if let Some(count) = value.as_u64() {
                    let count_bytes = count.to_be_bytes();
                    if let Ok(encrypted_bytes) = self.encrypt_bytes(&count_bytes) {
                        let base64_value = general_purpose::STANDARD.encode(&encrypted_bytes);
                        p_encrypted.insert(key.clone(), json!(base64_value));
                    }
                }
            }

            
            let n_encrypted = encrypted_state["n"].as_object_mut().unwrap();
            for (key, value) in n_map {
                if let Some(count) = value.as_u64() {
                    let count_bytes = count.to_be_bytes();
                    if let Ok(encrypted_bytes) = self.encrypt_bytes(&count_bytes) {
                        let base64_value = general_purpose::STANDARD.encode(&encrypted_bytes);
                        n_encrypted.insert(key.clone(), json!(base64_value));
                    }
                }
            }

            
            data.with_state(encrypted_state)
        } else {
            data.clone()
        }
    }

    fn encrypt_lwwregister_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let (Some(value), Some(timestamp)) =
            (state_json.get("value"), state_json.get("timestamp"))
        {
            
            let value_str = value.to_string();

            
            if let Ok(encrypted_bytes) = self.encrypt_bytes(value_str.as_bytes()) {
                let encrypted_base64 = general_purpose::STANDARD.encode(&encrypted_bytes);

                let encrypted_state = json!({
                    "value": encrypted_base64,
                    "timestamp": timestamp,
                    "__meta": {
                        "encrypted": true,
                        "algorithm": "AES-GCM-256",
                        "value_type": value.is_string()? "string" :
                                     value.is_number()? "number" :
                                     value.is_boolean()? "boolean" :
                                     "object"
                    }
                });

                
                data.with_state(encrypted_state)
            } else {
                data.clone()
            }
        } else {
            data.clone()
        }
    }

    fn encrypt_orset_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let Some(elements) = state_json.get("elements").and_then(|v| v.as_object()) {
            let mut encrypted_state = json!({
                "elements": {},
                "__meta": {
                    "encrypted": true,
                    "algorithm": "AES-GCM-256",
                }
            });

            let encrypted_elements = encrypted_state["elements"].as_object_mut().unwrap();

            for (element_key, tags) in elements {
                
                let key_bytes = element_key.as_bytes();
                if let Ok(encrypted_key) = self.encrypt_bytes(key_bytes) {
                    let encrypted_key_base64 = general_purpose::STANDARD.encode(&encrypted_key);
                    
                    encrypted_elements.insert(encrypted_key_base64, tags.clone());
                }
            }

            
            data.with_state(encrypted_state)
        } else {
            data.clone()
        }
    }

    pub fn decrypt_data<K>(&self, data: &CrdtType<K>) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        if !self.encryption_enabled {
            return data.clone();
        }

        
        let state_json = data.get_state();

        
        if let Some(meta) = state_json.get("__meta") {
            if let Some(true) = meta.get("encrypted").and_then(|v| v.as_bool()) {
                
                match data.name().as_str() {
                    "gcounter" => self.decrypt_gcounter_data(data, state_json),
                    "pncounter" => self.decrypt_pncounter_data(data, state_json),
                    "lwwregister" => self.decrypt_lwwregister_data(data, state_json),
                    "orset" => self.decrypt_orset_data(data, state_json),
                    _ => data.clone(), 
                }
            } else {
                data.clone() 
            }
        } else {
            data.clone() 
        }
    }

    fn decrypt_gcounter_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let Some(encrypted_map) = state_json.get("state").and_then(|v| v.as_object()) {
            let mut decrypted_state = json!({
                "state": {}
            });

            let decrypted_map = decrypted_state["state"].as_object_mut().unwrap();

            for (key, encrypted_value) in encrypted_map {
                if let Some(encrypted_base64) = encrypted_value.as_str() {
                    
                    if let Ok(encrypted_bytes) = general_purpose::STANDARD.decode(encrypted_base64)
                    {
                        
                        if let Ok(decrypted_bytes) = self.decrypt_bytes(&encrypted_bytes) {
                            
                            if decrypted_bytes.len() == 8 {
                                let mut count_bytes = [0u8; 8];
                                count_bytes.copy_from_slice(&decrypted_bytes);
                                let count = u64::from_be_bytes(count_bytes);
                                decrypted_map.insert(key.clone(), json!(count));
                            }
                        }
                    }
                }
            }

            
            data.with_state(decrypted_state)
        } else {
            data.clone()
        }
    }

    fn decrypt_pncounter_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let (Some(p_encrypted), Some(n_encrypted)) = (
            state_json.get("p").and_then(|v| v.as_object()),
            state_json.get("n").and_then(|v| v.as_object()),
        ) {
            let mut decrypted_state = json!({
                "p": {},
                "n": {}
            });

            
            let p_decrypted = decrypted_state["p"].as_object_mut().unwrap();
            for (key, encrypted_value) in p_encrypted {
                if let Some(encrypted_base64) = encrypted_value.as_str() {
                    if let Ok(encrypted_bytes) = general_purpose::STANDARD.decode(encrypted_base64)
                    {
                        if let Ok(decrypted_bytes) = self.decrypt_bytes(&encrypted_bytes) {
                            if decrypted_bytes.len() == 8 {
                                let mut count_bytes = [0u8; 8];
                                count_bytes.copy_from_slice(&decrypted_bytes);
                                let count = u64::from_be_bytes(count_bytes);
                                p_decrypted.insert(key.clone(), json!(count));
                            }
                        }
                    }
                }
            }

            
            let n_decrypted = decrypted_state["n"].as_object_mut().unwrap();
            for (key, encrypted_value) in n_encrypted {
                if let Some(encrypted_base64) = encrypted_value.as_str() {
                    if let Ok(encrypted_bytes) = general_purpose::STANDARD.decode(encrypted_base64)
                    {
                        if let Ok(decrypted_bytes) = self.decrypt_bytes(&encrypted_bytes) {
                            if decrypted_bytes.len() == 8 {
                                let mut count_bytes = [0u8; 8];
                                count_bytes.copy_from_slice(&decrypted_bytes);
                                let count = u64::from_be_bytes(count_bytes);
                                n_decrypted.insert(key.clone(), json!(count));
                            }
                        }
                    }
                }
            }

            
            data.with_state(decrypted_state)
        } else {
            data.clone()
        }
    }

    fn decrypt_lwwregister_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let (Some(encrypted_value), Some(timestamp)) = (
            state_json.get("value").and_then(|v| v.as_str()),
            state_json.get("timestamp"),
        ) {
            
            let value_type = state_json
                .get("__meta")
                .and_then(|m| m.get("value_type"))
                .and_then(|t| t.as_str())
                .unwrap_or("string");

            
            if let Ok(encrypted_bytes) = general_purpose::STANDARD.decode(encrypted_value) {
                
                if let Ok(decrypted_bytes) = self.decrypt_bytes(&encrypted_bytes) {
                    
                    let decrypted_str = String::from_utf8_lossy(&decrypted_bytes).to_string();

                    let decrypted_value = match value_type {
                        "number" => {
                            serde_json::from_str::<Value>(&decrypted_str).unwrap_or(Value::Null)
                        }
                        "boolean" => match decrypted_str.as_str() {
                            "true" => Value::Bool(true),
                            "false" => Value::Bool(false),
                            _ => Value::String(decrypted_str),
                        },
                        "object" => {
                            serde_json::from_str::<Value>(&decrypted_str).unwrap_or(Value::Null)
                        }
                        _ => Value::String(decrypted_str), 
                    };

                    let decrypted_state = json!({
                        "value": decrypted_value,
                        "timestamp": timestamp
                    });

                    
                    data.with_state(decrypted_state)
                } else {
                    data.clone()
                }
            } else {
                data.clone()
            }
        } else {
            data.clone()
        }
    }

    fn decrypt_orset_data<K>(&self, data: &CrdtType<K>, state_json: Value) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        
        if let Some(encrypted_elements) = state_json.get("elements").and_then(|v| v.as_object()) {
            let mut decrypted_state = json!({
                "elements": {}
            });

            let decrypted_elements = decrypted_state["elements"].as_object_mut().unwrap();

            for (encrypted_key, tags) in encrypted_elements {
                
                if let Ok(encrypted_bytes) = general_purpose::STANDARD.decode(encrypted_key) {
                    
                    if let Ok(decrypted_bytes) = self.decrypt_bytes(&encrypted_bytes) {
                        
                        let decrypted_key = String::from_utf8_lossy(&decrypted_bytes).to_string();
                        decrypted_elements.insert(decrypted_key, tags.clone());
                    }
                }
            }

            
            data.with_state(decrypted_state)
        } else {
            data.clone()
        }
    }

    
    fn encrypt_bytes(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        match self.algorithm {
            EncryptionAlgorithm::AesGcm256 => {
                
                let cipher = Aes256Gcm::new(&self.key);

                
                let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

                
                match cipher.encrypt(&nonce, plaintext) {
                    Ok(mut ciphertext) => {
                        
                        let mut result = nonce.to_vec();
                        result.append(&mut ciphertext);
                        Ok(result)
                    }
                    Err(_) => Err("Encryption failed".to_string()),
                }
            }
        }
    }

    fn decrypt_bytes(&self, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        match self.algorithm {
            EncryptionAlgorithm::AesGcm256 => {
                
                if ciphertext.len() <= 12 {
                    return Err("Ciphertext too short".to_string());
                }

                
                let nonce_bytes = &ciphertext[0..12];
                let nonce = Nonce::from_slice(nonce_bytes);

                
                let actual_ciphertext = &ciphertext[12..];

                
                let cipher = Aes256Gcm::new(&self.key);

                
                match cipher.decrypt(nonce, actual_ciphertext) {
                    Ok(plaintext) => Ok(plaintext),
                    Err(_) => Err("Decryption failed".to_string()),
                }
            }
        }
    }
}
