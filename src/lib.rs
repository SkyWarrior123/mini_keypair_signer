// Use statements
use rand::Rng;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};


// Define the Keypair structure
pub struct Keypair {
    #[serde(with = "hex")] 
    pub private_key: Vec<u8>,
    #[serde(with = "hex")] 
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

        // Concatenate the msg and private_key by updating the hasher sequentially: SHA-256(msg + private_key)
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

// Unit tests 
#[cfg(test)]
mod tests {
    // Import everything from the parent module 
    use super::*;

    #[test]
    fn test_sign_verify_success() {
        // Generate a keypair
        let keypair = Keypair::generate(); 
        
        // Define a message
        let msg = b"Sahbaaz is developing a mini key signer";

        // Sign the message
        let signature = keypair.sign(msg);

        // Verify the signature
        let is_valid = keypair.verify(msg, &signature);

        // Assert that the verification was successful
        assert!(is_valid, "Signature should be valid");
    }

    #[test]
    fn test_sign_verify_fail_msg_changed() {
        // Generate a keypair
        let keypair = Keypair::generate();

        // Original message
        let msg_original = b"Sahbaaz is developing a toy key signer";
        // Tampered message
        let msg_tampered = b"Sahbaaz is not developing a toy key signer";

        // Sign the original message
        let signature  = keypair.sign(msg_original);

        // Verify the tampered message
        let is_valid = keypair.verify(msg_tampered, &signature);

        // Assert that the signature is not valid 
        assert!(!is_valid, "Signature is invalid when the message changes");
    }

}