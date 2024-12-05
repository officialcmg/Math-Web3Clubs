use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use hex::encode;
use rand;

fn main() {
    // Generate random private key
    let private_key = SigningKey::random(&mut rand::thread_rng());
    let private_key_bytes = private_key.to_bytes();
    
    // Get the public key
    let verifying_key = private_key.verifying_key(); // generates the public key by doing scalar multiplication of the private key with the generator point of secp256k1 elliptic curve
    let public_key = verifying_key.to_encoded_point(false); // encodes the public key in uncompressed format
    let public_key_bytes = public_key.as_bytes();
    
    // Generate Ethereum address (last 20 bytes of keccak256 hash of public key)
    let mut hasher = Keccak256::new();
    // Skip the first byte (0x04) which indicates uncompressed public key
    hasher.update(&public_key_bytes[1..]);
    let hash: = hasher.finalize(); //  The hash is 32 bytes long
    
    // Take last 20 bytes as Ethereum address
    let eth_address = &hash[12..];
    
    // Print results
    println!("Private Key: {}", encode(private_key_bytes));
    println!("Public Key: {}", encode(public_key_bytes));
    println!("Ethereum Address: 0x{}", encode(eth_address));
}