use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use crate::Value;

// StorageEngine handles all disk I/O operations
pub struct StorageEngine {
    file_path: String, // This just stores the path to our database file as a String
}

impl StorageEngine {
    // Create a new storage engine with the given file path
    pub fn new(file_path: &str) -> Self {
        // &str = borrowed string (doesn't own the data)
        // .to_string() = converts &str to String (owned data)
        // We need to_string() because we want to store the path permanently
        StorageEngine {
            file_path: file_path.to_string(),
        }
    }

    // Save the entire database to disk
    // Uses bincode for fast binary serialization
    // is not a special return type. It is simply a type alias. In the standard library (std::io): pub type Result<T> = std::result::Result<T, std::io::Error>; So: io::Result<()> is exactly equivalent to: Result<(), std::io::Error>
    pub fn save(&self, data: &HashMap<String, Value>) -> io::Result<()> {
        println!("💾 Saving database to '{}'...", self.file_path);

        // Serialize the HashMap to bytes
        let encoded = bincode::serialize(data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Let's break this down:
        //
        // bincode::serialize(data) - Converts our HashMap into bytes (Vec<u8>)
        //   Returns: Result<Vec<u8>, Box<dyn Error>>
        //
        // .map_err(...) - If there's an error, convert it to io::Error
        //   Why? Because our function returns io::Result, not bincode's Result
        //
        // |e| - This is a closure (anonymous function) with parameter e
        //
        // io::Error::new(...) - Creates a new IO error
        //   io::ErrorKind::Other - The type of error
        //   e.to_string() - Convert the bincode error to a String message
        //
        // ? - The question mark operator
        //   If Result is Ok(value), unwrap the value and continue
        //   If Result is Err(e), return early with the error

        // Write bytes to file
        let mut file = File::create(&self.file_path)?;
        // File::create() - Opens a file for writing
        //   - If file exists: truncates it (deletes old content)
        //   - If file doesn't exist: creates it
        //   Returns: io::Result<File>
        //
        // &self.file_path - Borrows the file path
        //
        // ? - If creating file fails, return the error immediately
        //
        // mut file - The file handle is mutable because we'll write to it
        file.write_all(&encoded)?;
        // write_all() - Writes all bytes to the file
        //   &encoded - Borrows the byte vector
        //   Returns: io::Result<()>
        //
        // ? - If writing fails, return error

        file.sync_all()?; // Ensure data is written to disk
        // sync_all() - Forces the OS to write data to disk immediately
        //   Normally, OS keeps data in memory buffer for speed
        //   This ensures data survives even if power goes out!
        //   This is called "flushing" or "syncing"

        println!("✓ Saved {} entries ({} bytes)", data.len(), encoded.len());
        Ok(())
    }

    // Load the entire database from disk
    // Returns: io::Result<HashMap<String, Value>>
    //   Ok(HashMap) if successful
    //   Err(io::Error) if file doesn't exist or can't be read
    pub fn load(&self) -> io::Result<HashMap<String, Value>> {
        // Check if file exists

        if !Path::new(&self.file_path).exists() {
            // Path::new() - Creates a Path object from string
            // .exists() - Returns true if file exists
            println!("ℹ No existing database file found, starting fresh");
            return Ok(HashMap::new());
        }

        println!("📂 Loading database from '{}'...", self.file_path);

        // Read all bytes from file
        let mut file = File::open(&self.file_path)?;
        // File::open() - Opens file in read-only mode
        //   Returns: io::Result<File>
        let mut buffer = Vec::new();

        // Vec::new() - Creates an empty vector to hold bytes
        // mut buffer - We'll add bytes to it, so it needs to be mutable

        file.read_to_end(&mut buffer)?;

        // read_to_end() - Reads entire file into the buffer
        //   &mut buffer - Needs mutable reference to add data
        //   Returns: io::Result<usize> (number of bytes read)
        //
        // ? - If reading fails, return error

        // Now 'buffer' contains all the bytes from the file

        // Deserialize bytes back into HashMap
        let data: HashMap<String, Value> = bincode::deserialize(&buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // bincode::deserialize(&buffer) - Converts bytes back to HashMap
        //   &buffer - Borrows the byte vector
        //   Returns: Result<HashMap<String, Value>, Box<dyn Error>>
        //
        // .map_err(...) - Convert bincode error to io::Error (same as in save)
        //
        // ? - If deserialization fails, return error
        //
        // : HashMap<String, Value> - Explicitly tells Rust what type to deserialize into

        println!("✓ Loaded {} entries ({} bytes)", data.len(), buffer.len());
        Ok(data)
    }

    // Append a single key-value pair to the file (write-ahead log style)
    // This is more efficient for single writes but we'll improve this later
    pub fn append(&self, key: &str, value: &Value) -> io::Result<()> {
        // For now, we'll implement a simple version
        // In Stage 5, we'll make this a proper write-ahead log

        // Create a single-entry map
        let mut entry = HashMap::new();
        entry.insert(key.to_string(), value.clone());

        // Serialize it
        let encoded = bincode::serialize(&entry)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Append to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        file.write_all(&encoded)?;
        Ok(())
    }

    // Check if storage file exists
    pub fn exists(&self) -> bool {
        Path::new(&self.file_path).exists()
    }

    // Delete the storage file
    pub fn delete_file(&self) -> io::Result<()> {
        if self.exists() {
            std::fs::remove_file(&self.file_path)?;
            println!("✓ Deleted storage file");
        }
        Ok(())
    }

    // Get file size in bytes
    pub fn file_size(&self) -> io::Result<u64> {
        let metadata = std::fs::metadata(&self.file_path)?;
        Ok(metadata.len())
    }
}
