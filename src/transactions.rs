use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub signature: String,
}

impl Transaction {
    pub fn hash(&self) -> String {
        let serialized = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn verify_signature(&self) -> bool {
        // Placeholder: Add real cryptographic signature verification
        !self.signature.is_empty()
    }
}
