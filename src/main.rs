use std::collections::HashMap;
// use std::io::{self, Write};

// Enum to represent different value types our database can store
// This is like a union of different types
#[derive(Clone, Debug)] // These let us copy and print Values
enum Value {
    // Enums can hold different types of data. Each variant can store its own data type!
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    // Convert Value to a displayable string
    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(), // type & because "s" reference to the string stored inside the enum , same for below as well
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
        }
    }

    fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i), // i is a reference to the integer stored inside the enum, because self is borrowed (&self). the * is the dereference operator.take the value inside the i instead of the reference to it.
            _ => None,
        }
    }

    // Helper to get string if the value is a string
    fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}
// Our Database struct - this is like a class in other languages
// It holds all our data
struct Database {
    // HashMap is Rust's hash table - stores key-value pairs
    // String = key type, Value (Enum) = value type
    store: HashMap<String, Value>,
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
    fn insert(&mut self, key: String, value: Value) {
        self.store.insert(key, value);
        println!("✓ Data inserted successfully");
    }

    // Convenience methods for inserting specific types
    fn insert_string(&mut self, key: String, value: String) {
        self.insert(key, Value::String(value));
    }

    fn insert_integer(&mut self, key: String, value: i64) {
        self.insert(key, Value::Integer(value));
    }

    fn insert_float(&mut self, key: String, value: f64) {
        self.insert(key, Value::Float(value));
    }

    fn insert_boolean(&mut self, key: String, value: bool) {
        self.insert(key, Value::Boolean(value));
    }

    // Retrieve a value by key
    // &self = immutable reference (we're just reading, not modifying)
    // Returns Option<String> - either Some(value) or None
    fn get(&self, key: &str) -> Option<Value> {
        // .get() returns Option<&String>, we clone to return owned String
        // .get() returns Option<&Value>, we clone to return owned Value
        self.store.get(key).cloned()
    }

    fn update(&mut self, key: String, value: Value) -> Result<(), String> {
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

    fn exists(&self, key: &str) -> bool {
        return self.store.contains_key(key);
    }

    // New: Get all entries of a specific type
    fn get_all_integers(&self) -> Vec<(String, i64)> {
        self.store
            .iter() // Start iterating over all key-value pairs in the HashMap `store`
            .filter_map(|(k, v)| {
                // What filter_map does: If you return Some(value) → it keeps value. If you return None → it discards it.
                // converts some items into (String, i64) and drops others,  filter_map will transform each (k, v) pair , - if the value is an Integer, return Some((key.clone(), value)) , - if not, return None (and filter_map will discard it)
                if let Value::Integer(i) = v {
                    // Pattern match: check if v is Value::Integer
                    Some((k.clone(), *i)) // Extract the i64 and clone the key, wrap in Some
                } else {
                    None
                }
            })
            .collect() // takes an iterator of items and builds a collection out of them. It converts the iterator into whatever collection type you ask for.

        /*
        Iterator<Item = Some((String, i64)) or None>
         →
        filter_map removes None
         →
        Iterator<Item = (String, i64)>
         →
        collect() → Vec<(String, i64)>

        */
    }
}

fn main() {
    println!("=== RustDB Stage 1: In-Memory Key-Value Store ===\n");

    // Create a new database instance
    // 'mut' makes it mutable (changeable)
    let mut db = Database::new();

    println!("--- Inserting Different Data Types ---");
    db.insert_string("user:1:name".to_string(), "Alice".to_string());
    db.insert_integer("user:1:age".to_string(), 30);
    db.insert_float("user:1:score".to_string(), 95.5);
    db.insert_boolean("user:1:active".to_string(), true);

    db.insert_string("user:2:name".to_string(), "Bob".to_string());
    db.insert_integer("user:2:age".to_string(), 25);
    db.insert_float("user:2:score".to_string(), 88.3);
    db.insert_boolean("user:2:active".to_string(), false);

    println!("\nTotal entries: {}", db.count());

    // Retrieve and display different types
    println!("\n--- Reading Different Data Types ---");

    if let Some(value) = db.get("user:1:name") {
        println!("Name: {} (type: String)", value.to_string());
    }

    if let Some(value) = db.get("user:1:age") {
        println!("Age: {} (type: Integer)", value.to_string());
        // We can also extract the actual integer
        if let Some(age) = value.as_integer() {
            println!("  → Actual age value: {}", age);
        }
    }

    if let Some(value) = db.get("user:1:score") {
        println!("Score: {} (type: Float)", value.to_string());
    }

    if let Some(value) = db.get("user:1:active") {
        println!("Active: {} (type: Boolean)", value.to_string());
    }

    // Exercise 2: Test exists method
    println!("\n--- Testing exists() method ---");
    println!("Does 'user:1:name' exist? {}", db.exists("user:1:name"));
    println!("Does 'user:999' exist? {}", db.exists("user:999"));

    // Update with different type
    println!("\n--- Updating Values ---");
    match db.update("user:1:age".to_string(), Value::Integer(31)) {
        Ok(_) => {
            println!("✓ Updated user:1:age");
            if let Some(value) = db.get("user:1:age") {
                println!("  New value: {}", value.to_string());
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Get all integers
    println!("\n--- All Integer Values ---");
    for (key, value) in db.get_all_integers() {
        println!("{} = {}", key, value);
    }

    // List all keys with their types
    println!("\n--- All Entries (Key = Value) ---");
    for key in db.list_keys() {
        if let Some(value) = db.get(&key) {
            println!("{} = {} {:?}", key, value.to_string(), value);
        }
    }

    // Exercise 1: Test clear method
    println!("\n--- Testing clear() method ---");
    println!("Entries before clear: {}", db.count());
    db.clear();
    println!("Entries after clear: {}", db.count());

    // Add data again to show it works
    println!("\n--- Reinserting Data ---");
    db.insert_string("test".to_string(), "Hello World".to_string());
    db.insert_integer("number".to_string(), 42);
    println!("Entries after reinsertion: {}", db.count());

    for key in db.list_keys() {
        if let Some(value) = db.get(&key) {
            println!("{} = {}", key, value.to_string());
        }
    }
}
