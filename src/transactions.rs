use ed25519_dalek::{PublicKey, Signature, Verifier};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub timestamp: u128,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
    pub data: Option<String>, // Optional: for smart contract interactions
}

impl Transaction {
    pub fn new(
        sender: String,
        recipient: String,
        amount: u64,
        fee: u64,
        nonce: u64,
        public_key: Vec<u8>,
        signature: Vec<u8>,
        data: Option<String>,
    ) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            fee,
            nonce,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            signature,
            public_key,
            data,
        }
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.sender.as_bytes());
        hasher.update(self.recipient.as_bytes());
        hasher.update(&self.amount.to_be_bytes());
        hasher.update(&self.fee.to_be_bytes());
        hasher.update(&self.nonce.to_be_bytes());
        hasher.update(&self.timestamp.to_be_bytes());

        if let Some(ref d) = self.data {
            hasher.update(d.as_bytes());
        }

        hasher.finalize().to_vec()
    }

    pub fn is_valid(&self) -> bool {
        let public_key = PublicKey::from_bytes(&self.public_key).unwrap();
        let signature = Signature::from_bytes(&self.signature).unwrap();
        let hash = self.calculate_hash();

        public_key.verify(&hash, &signature).is_ok()
    }
}
