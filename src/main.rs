use std::collections::HashMap;
// use std::io::{self, Write};

// Our Database struct - this is like a class in other languages
// It holds all our data
struct Database {
    // HashMap is Rust's hash table - stores key-value pairs
    // String = key type, String = value type
    store: HashMap<String, String>,
}

impl Database {
    // Constructor - creates a new empty database
    // 'Self' refers to Database
    fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    // Insert a key-value pair
    // &mut self = mutable reference to self (we need to modify the database)
    fn insert(&mut self, key: String, value: String) {
        self.store.insert(key, value);
        println!("✓ Data inserted successfully");
    }

    // Retrieve a value by key
    // &self = immutable reference (we're just reading, not modifying)
    // Returns Option<String> - either Some(value) or None
    fn get(&self, key: &str) -> Option<String> {
        // .get() returns Option<&String>, we clone to return owned String
        self.store.get(key).cloned()
    }

    fn update(&mut self, key: String, value: String) -> Result<(), String> {
        if self.store.contains_key(&key) {
            self.store.insert(key, value);
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key))
        }
    }

    // Delete a key-value pair
    fn delete(&mut self, key: &str) -> Result<(), String> {
        if self.store.remove(key).is_some() {
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key))
        }
    }

    // List all keys (useful for debugging)
    fn list_keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    // Get total number of entries
    fn count(&self) -> usize {
        self.store.len()
    }

    // clear the database
    fn clear(&mut self) {
        let current_keys = self.list_keys();
        for key in current_keys {
            let _ = self.delete(&key);
        }

        println!("Database is cleared");
    }

    fn is_exists(&self, key: &str) -> bool {
        return self.store.contains_key(key);
    }
}

fn main() {
    println!("=== RustDB Stage 1: In-Memory Key-Value Store ===\n");

    // Create a new database instance
    // 'mut' makes it mutable (changeable)
    let mut db = Database::new();

    // Insert some data
    println!("Inserting data...");
    db.insert("user:1".to_string(), "Alice".to_string());
    db.insert("user:2".to_string(), "Bob".to_string());
    db.insert("user:3".to_string(), "Charlie".to_string());

    println!("\nTotal entries: {}", db.count());

    // Retrieve data
    println!("\n--- Reading Data ---");
    match db.get("user:1") {
        Some(value) => println!("Found: user:1 = {}", value),
        None => println!("Key not found"),
    }

    match db.get("user:999") {
        Some(value) => println!("Found: user:999 = {}", value),
        None => println!("user:999 not found (expected)"),
    }

    // Update data
    println!("\n--- Updating Data ---");
    match db.update("user:2".to_string(), "Bob Smith".to_string()) {
        Ok(_) => {
            println!("✓ Updated user:2");
            if let Some(value) = db.get("user:2") {
                println!("  New value: {}", value);
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Try to update non-existent key
    match db.update("user:999".to_string(), "Nobody".to_string()) {
        Ok(_) => println!("Updated user:999"),
        Err(e) => println!("✗ Error: {} (expected)", e),
    }

    // Delete data
    println!("\n--- Deleting Data ---");
    match db.delete("user:3") {
        Ok(_) => println!("✓ Deleted user:3"),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\nTotal entries after delete: {}", db.count());

    // List all remaining keys
    println!("\n--- All Keys ---");
    for key in db.list_keys() {
        if let Some(value) = db.get(&key) {
            println!("{} = {}", key, value);
        }
    }

    // check if a key exists
    println!("is {} exists {}", "user:1", db.is_exists("user:1"));

    // clear the db
    db.clear();
    println!("is {} exists {}", "user:1", db.is_exists("user:1"));
    println!("\nTotal entries after clear: {}", db.count());
}
