use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use crate::Value;

// StorageEngine handles all disk I/O operations
pub struct StorageEngine {
    file_path: String,
}

impl StorageEngine {
    // Create a new storage engine with the given file path
    pub fn new(file_path: &str) -> Self {
        StorageEngine {
            file_path: file_path.to_string(),
        }
    }

    // Save the entire database to disk
    // Uses bincode for fast binary serialization
    pub fn save(&self, data: &HashMap<String, Value>) -> io::Result<()> {
        println!("💾 Saving database to '{}'...", self.file_path);

        // Serialize the HashMap to bytes
        let encoded = bincode::serialize(data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Write bytes to file
        let mut file = File::create(&self.file_path)?;
        file.write_all(&encoded)?;
        file.sync_all()?; // Ensure data is written to disk

        println!("✓ Saved {} entries ({} bytes)", data.len(), encoded.len());
        Ok(())
    }

    // Load the entire database from disk
    pub fn load(&self) -> io::Result<HashMap<String, Value>> {
        // Check if file exists

        if !Path::new(&self.file_path).exists() {
            println!("ℹ No existing database file found, starting fresh");
            return Ok(HashMap::new());
        }

        println!("📂 Loading database from '{}'...", self.file_path);

        // Read all bytes from file
        let mut file = File::open(&self.file_path)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        // Deserialize bytes back into HashMap
        let data: HashMap<String, Value> = bincode::deserialize(&buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        println!("✓ Loaded {} entries ({} bytes)", data.len(), buffer.len());
        Ok(data)
    }
}
