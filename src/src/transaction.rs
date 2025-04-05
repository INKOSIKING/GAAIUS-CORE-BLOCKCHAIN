use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
    pub nonce: u64,
}

impl Transaction {
    pub fn hash(&self) -> String {
        let encoded = bincode::serialize(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(encoded);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn is_valid(&self) -> bool {
        // TODO: Implement cryptographic signature verification
        // Placeholder: assume all transactions are valid
        true
    }
}
