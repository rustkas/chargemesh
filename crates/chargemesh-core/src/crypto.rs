//! Cryptographic utilities

use serde::{Deserialize, Serialize};

/// SHA-256 hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sha256Hash(pub [u8; 32]);

impl Sha256Hash {
    /// Compute SHA-256 hash of data
    pub fn compute(data: &[u8]) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        Self(hash)
    }

    /// Compute SHA-256 hash of a string
    pub fn compute_str(s: &str) -> Self {
        Self::compute(s.as_bytes())
    }

    /// Get hash as hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string
    pub fn from_hex(s: &str) -> CoreResult<Self> {
        let bytes = hex::decode(s)
            .map_err(|e| CoreError::Crypto(e.to_string()))?;
        if bytes.len() != 32 {
            return Err(CoreError::Crypto("Invalid hash length".to_string()));
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes);
        Ok(Self(hash))
    }
}

impl std::fmt::Display for Sha256Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Random token generator
pub fn generate_token() -> String {
    use rand::Rng;
    let bytes: [u8; 32] = rand::thread_rng().gen();
    hex::encode(bytes)
}