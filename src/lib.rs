// Use statements
use rand::Rng;
use sha2::{Sha256, Digest};

// Define the Keypair structure
pub struct Keypair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

// Define the Signer trait
pub trait Signer {
    fn sign(&self, msg: &[u8]) -> Vec<u8>;
    fn verify(&self, msg: &[u8], sig: &[u8]) -> bool;
}