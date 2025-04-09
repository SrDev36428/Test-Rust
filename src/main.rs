use flate2::read::GzDecoder;
use serde_json::Value;
use std::env;                                                                         // Importing env module to get system path
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Get the system path for the .tar.gz file
    let file_path = env::current_dir()?.join("reports/SBAdGroup/280339202921401.gz"); // Update with your file name
    match read_gz(file_path.to_str().unwrap()) {
        Ok(json) => {
            println!("Decoded JSON: {:?}", json);
        }
        Err(e) => {
            println!("Error reading tar.gz file: {}", e);
        }
    }
    Ok(())
}

fn read_gz(file_path: &str) -> io::Result<Value> {
    let file = File::open(file_path)?;
    let mut decoder = GzDecoder::new(file);
    let mut contents = Vec::new();                                                   // Read as bytes
    decoder.read_to_end(&mut contents)?;                                             // Read the entire .gz file into bytes

    // Attempt to decode JSON from bytes
    match serde_json::from_slice(&contents) {
        Ok(json) => return Ok(json),
        Err(e) => {
            // Log the error and the raw contents for debugging
            println!("Failed to decode JSON: {}", e);
            println!("Raw contents (first 100 bytes): {:?}", &contents[..100]);     // Show first 100 bytes
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid JSON data",
            ));
        }
    }
}
