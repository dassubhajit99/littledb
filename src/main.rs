use std::collections::HashMap;
// use std::io::{self, Write};

// Enum to represent different value types our database can store
// This is like a union of different types
#[derive(Clone, Debug, PartialEq)] // These let us copy and print Values . The PartialEq trait allows you to compare instances of a type to check for equality and enables use of the == and != operators.
enum Value {
    // Enums can hold different types of data. Each variant can store its own data type!
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}

impl Value {
    // Convert Value to a displayable string
    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(), // type & because "s" reference to the string stored inside the enum , same for below as well
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();

                format!("{{{}}}", items.join(", "))
            }
            Value::Null => "null".to_string(),
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

    fn get_field(&self, field: &str) -> Option<&Value> {
        match self {
            Value::Object(obj) => obj.get(field),
            _ => None,
        }
    }
}

// NEW: Query filter conditions
#[derive(Debug)]
enum Condition {
    Equals(String, Value),    // field equals value
    GreaterThan(String, i64), // field > value (for integers)
    LessThan(String, i64),    // field < value
    Contains(String, String), // string field contains substring
}

impl Condition {
    // Check if a value matches this condition
    fn matches(&self, value: &Value) -> bool {
        match self {
            Condition::Equals(field, expected) => {
                if let Some(actual) = value.get_field(field) {
                    actual == expected
                } else {
                    false
                }
            }
            Condition::GreaterThan(field, threshold) => {
                if let Some(actual) = value.get_field(field) {
                    if let Some(num) = actual.as_integer() {
                        return num > *threshold;
                    }
                }
                false
            }
            Condition::LessThan(field, threshold) => {
                if let Some(actual) = value.get_field(field) {
                    if let Some(num) = actual.as_integer() {
                        return num < *threshold;
                    }
                }
                false
            }
            Condition::Contains(field, substring) => {
                if let Some(actual) = value.get_field(field) {
                    if let Some(s) = actual.as_string() {
                        return s.contains(substring);
                    }
                }
                false
            }
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

    // NEW: Batch insert - insert multiple key-value pairs at once
    fn batch_insert(&mut self, entries: Vec<(String, Value)>) -> usize {
        //usize is guaranteed to be large enough to represent any memory address on the machine it's compiled for. On a 32-bit system, usize will be 32 bits wide (like u32), and on a 64-bit system, it will be 64 bits wide (like u64). usize is the standard type used for indexing into collections (like Vec or HashMap) and for representing sizes or lengths of data structures in Rust's standard library. This ensures compatibility and correctness across different architectures.
        let count = entries.len();
        for (key, value) in entries {
            self.store.insert(key, value);
        }
        println!("✓ Batch inserted {} entries", count);
        count
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

    fn batch_get(&self, keys: Vec<&str>) -> HashMap<String, Value> {
        let mut res = HashMap::new();
        for key in keys {
            if let Some(value) = self.get(key) {
                res.insert(key.to_string(), value);
            }
        }
        res
    }

    // NEW: Batch get - retrieve multiple keys at once

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

    fn batch_delete(&mut self, keys: Vec<&str>) -> usize {
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
    fn list_keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    // Get total number of entries
    fn count(&self) -> usize {
        self.store.len()
    }

    // clear the database
    fn clear(&mut self) {
        self.store.clear();

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

    fn query(&self, condition: Condition) -> Vec<(String, Value)> {
        self.store
            .iter() //This returns an iterator over references: So each item is a tuple: (&key, &value)
            .filter(|(_key, value)| condition.matches(value)) // filter() always receives a reference to each iterator item. , Because iterator items are passed by reference to the closure. Actual iterator item: (&String, &Value)   // 1 layer of reference What the closure in filter receives: &(&String, &Value)  // extra reference → 2 layers
            .map(|(k, v)| (k.clone(), v.clone())) //At this point, keys and values are references: But we want to return owned values in a Vec.So .map() takes references and clones them:
            .collect() // looks at the return type of the function. Then Rust automatically collects all (String, Value) items into a Vec.
    }

    // NEW: Query with multiple conditions (AND logic)
    fn query_multiple(&self, conditions: Vec<Condition>) -> Vec<(String, Value)> {
        self.store
            .iter()
            .filter(|(_key, value)| conditions.iter().all(|cond| cond.matches(value)))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    // NEW: Get all keys matching a prefix pattern
    fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.store
            .keys() // .keys() returns: Iterator<Item = &String> So each item is a reference to a key.
            .filter(|k| k.starts_with(prefix))
            .cloned() //At this stage, each item is still &String. .cloned() converts: &String → String (owned) . It is shorthand for: .map(|k| k.clone())
            .collect() // Rust knows the return type is Vec<String>, so it builds a vector of the cloned keys.
    }
}

fn main() {
    println!("=== RustDB Stage 1 Part 2: Advanced Operations ===\n");

    // Create a new database instance
    // 'mut' makes it mutable (changeable)
    let mut db = Database::new();

    // --- PART 1: Structured Data (JSON-like objects) ---
    println!("--- Creating Structured User Records ---");

    let mut user1 = HashMap::new();
    user1.insert("name".to_string(), Value::String("Alice".to_string()));
    user1.insert("age".to_string(), Value::Integer(30));
    user1.insert(
        "email".to_string(),
        Value::String("alice@example.com".to_string()),
    );
    user1.insert("active".to_string(), Value::Boolean(true));
    user1.insert(
        "tags".to_string(),
        Value::Array(vec![
            Value::String("admin".to_string()),
            Value::String("developer".to_string()),
        ]),
    );

    let mut user2 = HashMap::new();
    user2.insert("name".to_string(), Value::String("Bob".to_string()));
    user2.insert("age".to_string(), Value::Integer(25));
    user2.insert(
        "email".to_string(),
        Value::String("bob@example.com".to_string()),
    );
    user2.insert("active".to_string(), Value::Boolean(false));
    user2.insert(
        "tags".to_string(),
        Value::Array(vec![Value::String("user".to_string())]),
    );

    let mut user3 = HashMap::new();
    user3.insert("name".to_string(), Value::String("Charlie".to_string()));
    user3.insert("age".to_string(), Value::Integer(35));
    user3.insert(
        "email".to_string(),
        Value::String("charlie@example.com".to_string()),
    );
    user3.insert("active".to_string(), Value::Boolean(true));
    user3.insert(
        "tags".to_string(),
        Value::Array(vec![
            Value::String("developer".to_string()),
            Value::String("manager".to_string()),
        ]),
    );

    db.insert("user:1".to_string(), Value::Object(user1));
    db.insert("user:2".to_string(), Value::Object(user2));
    db.insert("user:3".to_string(), Value::Object(user3));

    println!("✓ Inserted 3 structured user records");

    println!("\n--- Accessing Nested Fields ---");

    if let Some(Value::Object(user)) = db.get("user:1") {
        if let Some(name) = user.get("name") {
            println!("User 1 name: {}", name.to_string());
        }
        if let Some(age) = user.get("age") {
            println!("User 1 age: {}", age.to_string());
        }
        if let Some(tags) = user.get("tags") {
            println!("User 1 tags: {}", tags.to_string());
        }
    }

    // --- PART 2: Batch Operations ---
    println!("\n--- Batch Insert Operation ---");
    let batch_data = vec![
        ("product:1".to_string(), Value::String("Laptop".to_string())),
        ("product:2".to_string(), Value::String("Mouse".to_string())),
        (
            "product:3".to_string(),
            Value::String("Keyboard".to_string()),
        ),
    ];

    db.batch_insert(batch_data);
    println!("Total entries now: {}", db.count());

    // Batch get
    println!("\n--- Batch Get Operation ---");
    let results = db.batch_get(vec!["product:1", "product:2", "product:999"]);

    for (key, value) in results {
        println!("  {} = {}", key, value.to_string());
    }

    // --- PART 3: Query Filtering ---
    println!("\n--- Query: Find all users with age > 28 ---");
    let results = db.query(Condition::GreaterThan("age".to_string(), 28));
    println!("Found {} users with example.com email:", results.len());
    for (key, _) in results {
        println!("  {}", key);
    }

    // Multiple conditions (AND)
    println!("\n--- Query: Active users with age > 28 ---");
    let results = db.query_multiple(vec![
        Condition::Equals("active".to_string(), Value::Boolean(true)),
        Condition::GreaterThan("age".to_string(), 28),
    ]);
    println!("Found {} users:", results.len());
    for (key, value) in results {
        if let Value::Object(obj) = value {
            let name = obj
                .get("name")
                .map(|v| v.to_string()) // How .map(...) works on Option . its behavior is defined as: If self is Some(x) → apply f(x) → return Some(...) , If self is None → do nothing → return None. .map(|v| v.to_string()) runs only when get("name") returns Some(...).
                .unwrap_or("Unknown".to_string()); // If .get("name") returned Some(...) → use the "name" value. If it returned None → default to "Unknown".
            println!("  {} - {}", key, name);
        }
    }

    // --- PART 4: Prefix Matching ---
    println!("\n--- Find all user keys ---");
    let user_keys = db.keys_with_prefix("user:");
    println!("Found {} user keys:", user_keys.len());
    for key in user_keys {
        println!("  {}", key);
    }

    println!("\n--- Find all product keys ---");
    let product_keys = db.keys_with_prefix("product:");
    println!("Found {} product keys:", product_keys.len());
    for key in product_keys {
        println!("  {}", key);
    }

    // --- PART 5: Batch Delete ---
    println!("\n--- Batch Delete Products ---");
    db.batch_delete(vec!["product:1", "product:2", "product:3"]);
    println!("Total entries now: {}", db.count());
}
