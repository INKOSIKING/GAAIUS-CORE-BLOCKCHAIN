use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let public_key_bytes = keypair.public.to_bytes().to_vec();
        let private_key_bytes = keypair.secret.to_bytes().to_vec();

        let mut hasher = Sha256::new();
        hasher.update(&public_key_bytes);
        let address = hex::encode(hasher.finalize());

        Wallet {
            address,
            public_key: public_key_bytes,
            private_key: private_key_bytes,
        }
    }

    pub fn sign_message(&self, message: &[u8]) -> Signature {
        let secret = SecretKey::from_bytes(&self.private_key).unwrap();
        let public = PublicKey::from_bytes(&self.public_key).unwrap();
        let keypair = Keypair { secret, public };

        keypair.sign(message)
    }

    pub fn verify_signature(&self, message: &[u8], signature: &Signature) -> bool {
        let public_key = PublicKey::from_bytes(&self.public_key).unwrap();
        public_key.verify(message, signature).is_ok()
    }
}
