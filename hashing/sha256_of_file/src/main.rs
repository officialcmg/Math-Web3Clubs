use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, Read};

fn main() {
    match sha256_file_hash("/home/chris/math/hashing/sha256/src/main.rs") {
        Ok(file_hash) => println!("SHA-256 hash of file: {}", file_hash),
        Err(e) => eprintln!("Error hashing file: {}", e),
    }
}

fn sha256_file_hash(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    
    // Create a buffer for reading chunks of the file
    let mut buffer = [0; 1024];  // 1KB chunks

    // Read the file in chunks and update the hasher with each chunk
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
             break; 
        } // End of file
        hasher.update(&buffer[..n]);
    }

    // Finalize the hash computation and return as hex string
    Ok(format!("{:x}", hasher.finalize()))
}

