// Use statements
use rand::Rng;
use sha2::{Sha256, Digest};

// Define the Keypair structure
pub struct Keypair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

// Implementation for Keypair
impl Keypair {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();

        // Generate a 32-byte private key
        let mut private_key_bytes = [0u8; 32];
        rng.fill(&mut private_key_bytes);

        // Generate a 32-byte public key
        let mut public_key_bytes = [0u8; 32];
        rng.fill(&mut public_key_bytes);

        Self {
            private_key: private_key_bytes.to_vec(),
            public_key: public_key_bytes.to_vec(),
        }
    }
}

// Define the Signer trait
pub trait Signer {
    fn sign(&self, msg: &[u8]) -> Vec<u8>;
    fn verify(&self, msg: &[u8], sig: &[u8]) -> bool;
}

// Implement of Signer trait for Keypair
impl Signer for Keypair {

    fn sign(&self, msg: &[u8]) ->  Vec<u8> {
        // Create a new SHA-256 hasher
        let mut hasher = Sha256::new();

        /// Concatenate the msg and private_key by updating the hasher sequentially
        /// SHA-256(msg + private_key)
        hasher.update(msg);
        hasher.update(&self.private_key);
 
        // Return the finalized hash as Vec<u8>
        hasher.finalize().to_vec()
    }


    /// Verifies the signature by recomputing the hash and checks for equality
    fn verify(&self, msg: &[u8], sig: &[u8]) -> bool {
        // Re-compute the hash using the same logic as sign
        let computed_sig = self.sign(msg);

        // Equality check
        computed_sig == sig
    }
}