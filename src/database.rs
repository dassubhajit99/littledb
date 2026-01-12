use crate::{Condition, StorageEngine, Value};
use std::{collections::HashMap, io};

// Our Database struct - this is like a class in other languages
// It holds all our data
pub struct Database {
    // HashMap is Rust's hash table - stores key-value pairs
    // String = key type, Value (Enum) = value type
    store: HashMap<String, Value>,
    storage: StorageEngine,
    auto_save: bool, // Automatically save after each write operation
}

impl Database {
    // Constructor - creates a new empty database
    // 'Self' refers to Database
    pub fn new(file_path: &str) -> Self {
        Database {
            store: HashMap::new(),
            storage: StorageEngine::new(file_path),
            auto_save: true,
        }
    }

    // Load database from disk (if file exists)
    pub fn load(&mut self) -> io::Result<()> {
        self.store = self.storage.load()?;
        Ok(())
    }

    // Save database to disk
    pub fn save(&self) -> io::Result<()> {
        self.storage.save(&self.store)
    }

    // Enable or disable auto-save (useful for batch operations)
    pub fn set_auto_save(&mut self, enabled: bool) {
        self.auto_save = enabled;
        println!("Auto-save {}", if enabled { "enabled" } else { "disabled" });
    }

    // Insert a key-value pair
    // &mut self = mutable reference to self (we need to modify the database)
    pub fn insert(&mut self, key: String, value: Value) -> io::Result<()> {
        self.store.insert(key, value);

        if self.auto_save {
            self.save()?;
        }
        println!("✓ Data inserted successfully");
        Ok(())
    }

    // NEW: Batch insert - insert multiple key-value pairs at once
    pub fn batch_insert(&mut self, entries: Vec<(String, Value)>) -> io::Result<usize> {
        //usize is guaranteed to be large enough to represent any memory address on the machine it's compiled for. On a 32-bit system, usize will be 32 bits wide (like u32), and on a 64-bit system, it will be 64 bits wide (like u64). usize is the standard type used for indexing into collections (like Vec or HashMap) and for representing sizes or lengths of data structures in Rust's standard library. This ensures compatibility and correctness across different architectures.
        let count = entries.len();
        for (key, value) in entries {
            self.store.insert(key, value);
        }
        if self.auto_save {
            self.save()?;
        }
        println!("✓ Batch inserted {} entries", count);
        Ok(count)
    }

    // // Convenience methods for inserting specific types
    // pub fn insert_string(&mut self, key: String, value: String) {
    //     self.insert(key, Value::String(value));
    // }

    // pub fn insert_integer(&mut self, key: String, value: i64) {
    //     self.insert(key, Value::Integer(value));
    // }

    // pub fn insert_float(&mut self, key: String, value: f64) {
    //     self.insert(key, Value::Float(value));
    // }

    // pub fn insert_boolean(&mut self, key: String, value: bool) {
    //     self.insert(key, Value::Boolean(value));
    // }

    // Retrieve a value by key
    // &self = immutable reference (we're just reading, not modifying)
    // Returns Option<String> - either Some(value) or None
    pub fn get(&self, key: &str) -> Option<Value> {
        // .get() returns Option<&String>, we clone to return owned String
        // .get() returns Option<&Value>, we clone to return owned Value
        self.store.get(key).cloned()
    }

    pub fn batch_get(&self, keys: Vec<&str>) -> HashMap<String, Value> {
        let mut res = HashMap::new();
        for key in keys {
            if let Some(value) = self.get(key) {
                res.insert(key.to_string(), value);
            }
        }
        res
    }

    // NEW: Batch get - retrieve multiple keys at once

    pub fn update(&mut self, key: String, value: Value) -> Result<(), String> {
        if self.store.contains_key(&key) {
            self.store.insert(key, value);
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key))
        }
    }

    // Delete a key-value pair
    pub fn delete(&mut self, key: &str) -> Result<(), String> {
        if self.store.remove(key).is_some() {
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key))
        }
    }

    pub fn batch_delete(&mut self, keys: Vec<&str>) -> usize {
        let mut deleted = 0;
        for key in keys {
            if self.store.remove(key).is_some() {
                deleted += 1;
            }
        }

        println!("✓ Batch deleted {} entries", deleted);
        deleted
    }

    // List all keys (useful for debugging)
    pub fn list_keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    // Get total number of entries
    pub fn count(&self) -> usize {
        self.store.len()
    }

    // clear the database
    pub fn clear(&mut self) {
        self.store.clear();

        println!("Database is cleared");
    }

    pub fn exists(&self, key: &str) -> bool {
        return self.store.contains_key(key);
    }

    // New: Get all entries of a specific type
    pub fn get_all_integers(&self) -> Vec<(String, i64)> {
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

    pub fn query(&self, condition: Condition) -> Vec<(String, Value)> {
        self.store
            .iter() //This returns an iterator over references: So each item is a tuple: (&key, &value)
            .filter(|(_key, value)| condition.matches(value)) // filter() always receives a reference to each iterator item. , Because iterator items are passed by reference to the closure. Actual iterator item: (&String, &Value)   // 1 layer of reference What the closure in filter receives: &(&String, &Value)  // extra reference → 2 layers
            .map(|(k, v)| (k.clone(), v.clone())) //At this point, keys and values are references: But we want to return owned values in a Vec.So .map() takes references and clones them:
            .collect() // looks at the return type of the function. Then Rust automatically collects all (String, Value) items into a Vec.
    }

    // NEW: Query with multiple conditions (AND logic)
    pub fn query_multiple(&self, conditions: Vec<Condition>) -> Vec<(String, Value)> {
        self.store
            .iter()
            .filter(|(_key, value)| conditions.iter().all(|cond| cond.matches(value)))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    // NEW: Get all keys matching a prefix pattern
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.store
            .keys() // .keys() returns: Iterator<Item = &String> So each item is a reference to a key.
            .filter(|k| k.starts_with(prefix))
            .cloned() //At this stage, each item is still &String. .cloned() converts: &String → String (owned) . It is shorthand for: .map(|k| k.clone())
            .collect() // Rust knows the return type is Vec<String>, so it builds a vector of the cloned keys.
    }

    // Get database statistics
    pub fn stats(&self) -> DatabaseStats {
        DatabaseStats {
            total_entries: self.count(),
            file_size: self.storage.file_size().unwrap_or(0),
            auto_save_enabled: self.auto_save,
        }
    }
}

pub struct DatabaseStats {
    pub total_entries: usize,
    pub file_size: u64,
    pub auto_save_enabled: bool,
}

impl DatabaseStats {
    pub fn print(&self) {
        println!("=== Database Statistics ===");
        println!("Total entries: {}", self.total_entries);
        println!(
            "File size: {} bytes ({:.2} KB)",
            self.file_size,
            self.file_size as f64 / 1024.0
        );
        println!(
            "Auto-save: {}",
            if self.auto_save_enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
    }
}
