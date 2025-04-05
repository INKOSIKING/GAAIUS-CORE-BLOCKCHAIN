use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
    pub signature: String,
    pub timestamp: u128,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, nonce: u64, signature: String, timestamp: u128) -> Self {
        Self {
            sender,
            receiver,
            amount,
            nonce,
            signature,
            timestamp,
        }
    }

    pub fn hash(&self) -> String {
        let serialized = serde_json::to_string(self).expect("Failed to serialize transaction");
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

// Optional: Custom display format
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tx from {} to {} of {} GAAIUS [nonce={}, hash={}]",
            self.sender,
            self.receiver,
            self.amount,
            self.nonce,
            self.hash()
        )
    }
}
