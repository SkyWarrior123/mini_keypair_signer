use std::error::Error;
use mini_keypair_signer::{Keypair, Signer};
use hex::encode as hex_encode;


fn main() -> Result<(), Box<dyn Error>> {
    // Sample message to sign
    let message = b"Hello this is Sahbaaz Ansari :)";

    // Generate a new keypair
    let kp = Keypair::generate();

    println!("Public key (hex): {}", hex_encode(&kp.public_key));
    
    // Json representation 
    // let json = kp.to_json()?;
    // println!("Generated Keypair (JSON):\n{}\n", json);
    
    // Sign the sample message
    let sig = kp.sign(message);
    println!("Message: {}", String::from_utf8_lossy(message));
    println!("Signature (hex): {}", hex_encode(&sig));

    // Verify signature using the same keypair
    let ok = kp.verify(message, &sig);
    println!("Verification result: {}", ok);

    Ok(())
}
