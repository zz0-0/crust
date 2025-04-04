use base64::{engine::general_purpose, Engine as _};
use crust_core::r#type::CrdtType;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::hash::Hash;

pub struct IntegritySecurity {
    signature_enabled: bool,
    keypair: Keypair,
    peer_public_keys: std::collections::HashMap<String, PublicKey>,
}

impl IntegritySecurity {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);

        IntegritySecurity {
            signature_enabled: true,
            keypair,
            peer_public_keys: std::collections::HashMap::new(),
        }
    }

    pub fn with_signature(enabled: bool) -> Self {
        let mut security = Self::new();
        security.signature_enabled = enabled;
        security
    }

    pub fn with_keypair(secret_key_bytes: &[u8], public_key_bytes: &[u8]) -> Result<Self, String> {
        if secret_key_bytes.len() != 32 || public_key_bytes.len() != 32 {
            return Err("Invalid key length".to_string());
        }

        let secret_key = match SecretKey::from_bytes(secret_key_bytes) {
            Ok(sk) => sk,
            Err(_) => return Err("Invalid secret key".to_string()),
        };

        let public_key = match PublicKey::from_bytes(public_key_bytes) {
            Ok(pk) => pk,
            Err(_) => return Err("Invalid public key".to_string()),
        };

        let keypair = Keypair {
            secret: secret_key,
            public: public_key,
        };

        Ok(IntegritySecurity {
            signature_enabled: true,
            keypair,
            peer_public_keys: std::collections::HashMap::new(),
        })
    }

    pub fn add_peer_public_key(
        &mut self,
        peer_id: &str,
        public_key_bytes: &[u8],
    ) -> Result<(), String> {
        match PublicKey::from_bytes(public_key_bytes) {
            Ok(pk) => {
                self.peer_public_keys.insert(peer_id.to_string(), pk);
                Ok(())
            }
            Err(_) => Err("Invalid public key".to_string()),
        }
    }

    pub fn get_public_key_bytes(&self) -> [u8; 32] {
        self.keypair.public.to_bytes()
    }

    pub fn sign_data<K>(&self, data: &CrdtType<K>) -> CrdtType<K>
    where
        K: Eq + Hash + Clone,
    {
        if !self.signature_enabled {
            return data.clone();
        }

        let state_json = data.get_state();

        let canonical_json = self.canonicalize_json(&state_json);

        let hash = self.hash_data(canonical_json.as_bytes());

        let signature = self.keypair.sign(&hash);
        let signature_bytes = signature.to_bytes();
        let signature_base64 = general_purpose::STANDARD.encode(&signature_bytes);

        let mut signed_state = state_json.clone();

        if let Some(obj) = signed_state.as_object_mut() {
            let meta = json!({
                "signed": true,
                "algorithm": "Ed25519",
                "pubkey": general_purpose::STANDARD.encode(self.keypair.public.as_bytes()),
                "signature": signature_base64,
                "timestamp": chrono::Utc::now().timestamp()
            });

            obj.insert("__signature".to_string(), meta);
        }

        data.with_state(signed_state)
    }

    pub fn verify_data<K>(&self, data: &CrdtType<K>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        if !self.signature_enabled {
            return true;
        }

        let state_json = data.get_state();

        let signature_meta = match state_json.get("__signature") {
            Some(meta) => meta,
            None => return false,
        };

        let (signature_base64, pubkey_base64) = match (
            signature_meta.get("signature").and_then(|s| s.as_str()),
            signature_meta.get("pubkey").and_then(|p| p.as_str()),
        ) {
            (Some(sig), Some(key)) => (sig, key),
            _ => return false,
        };

        let signature_bytes = match general_purpose::STANDARD.decode(signature_base64) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let pubkey_bytes = match general_purpose::STANDARD.decode(pubkey_base64) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let signature = match Signature::from_bytes(&signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        let public_key = match PublicKey::from_bytes(&pubkey_bytes) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        let mut unsigned_state = state_json.clone();
        if let Some(obj) = unsigned_state.as_object_mut() {
            obj.remove("__signature");
        }

        let canonical_json = self.canonicalize_json(&unsigned_state);

        let hash = self.hash_data(canonical_json.as_bytes());

        match public_key.verify(&hash, &signature) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn verify_peer_signature<K>(&self, data: &CrdtType<K>, peer_id: &str) -> bool
    where
        K: Eq + Hash + Clone,
    {
        if !self.signature_enabled {
            return true;
        }

        let peer_key = match self.peer_public_keys.get(peer_id) {
            Some(key) => key,
            None => return false,
        };

        let state_json = data.get_state();

        let signature_meta = match state_json.get("__signature") {
            Some(meta) => meta,
            None => return false,
        };

        let signature_base64 = match signature_meta.get("signature").and_then(|s| s.as_str()) {
            Some(sig) => sig,
            None => return false,
        };

        let signature_bytes = match general_purpose::STANDARD.decode(signature_base64) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let signature = match Signature::from_bytes(&signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        let mut unsigned_state = state_json.clone();
        if let Some(obj) = unsigned_state.as_object_mut() {
            obj.remove("__signature");
        }

        let canonical_json = self.canonicalize_json(&unsigned_state);

        let hash = self.hash_data(canonical_json.as_bytes());

        match peer_key.verify(&hash, &signature) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn canonicalize_json(&self, value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.canonicalize_json(v)).collect();
                format!("[{}]", items.join(","))
            }
            Value::Object(obj) => {
                let mut keys: Vec<&String> = obj.keys().collect();
                keys.sort();

                let pairs: Vec<String> = keys
                    .iter()
                    .map(|k| {
                        let v = &obj[*k];
                        format!("\"{}\":{}", k, self.canonicalize_json(v))
                    })
                    .collect();

                format!("{{{}}}", pairs.join(","))
            }
        }
    }

    fn hash_data(&self, data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
