use std::collections::HashMap;

use littledb::{Condition, Database, Value};

fn main() {
    println!("=== RustDB Stage 2: Persistent Storage ===\n");
    // Create or load database from file
    let mut db = Database::new("mydata.db");

    // Try to load existing data
    println!("--- Loading Database ---");

    match db.load() {
        Ok(_) => println!(
            "✓ Database loaded successfully (found {} entries)",
            db.count()
        ),
        Err(e) => println!("ℹ Starting fresh database: {}", e),
    }

    // Show current stats
    println!();
    db.stats().print();

    // Add some data if database is empty
    if db.count() == 0 {
        println!("\n--- Populating Database ---");

        // Create user objects
        let mut user1 = HashMap::new();
        user1.insert("name".to_string(), Value::String("Alice".to_string()));
        user1.insert("age".to_string(), Value::Integer(30));
        user1.insert(
            "email".to_string(),
            Value::String("alice@example.com".to_string()),
        );
        user1.insert("active".to_string(), Value::Boolean(true));

        let mut user2 = HashMap::new();
        user2.insert("name".to_string(), Value::String("Bob".to_string()));
        user2.insert("age".to_string(), Value::Integer(25));
        user2.insert(
            "email".to_string(),
            Value::String("bob@example.com".to_string()),
        );
        user2.insert("active".to_string(), Value::Boolean(false));

        let mut user3 = HashMap::new();
        user3.insert("name".to_string(), Value::String("Charlie".to_string()));
        user3.insert("age".to_string(), Value::Integer(35));
        user3.insert(
            "email".to_string(),
            Value::String("charlie@example.com".to_string()),
        );
        user3.insert("active".to_string(), Value::Boolean(true));

        // Insert users
        db.insert("user:1".to_string(), Value::Object(user1))
            .unwrap();
        db.insert("user:2".to_string(), Value::Object(user2))
            .unwrap();
        db.insert("user:3".to_string(), Value::Object(user3))
            .unwrap();

        println!("✓ Inserted 3 users");
    } else {
        println!("\n--- Existing Data Found ---");
        println!("Working with {} existing entries", db.count());
    }

    // Display all users
    println!("\n--- All Users ---");

    let user_keys = db.keys_with_prefix("user:");
    for key in &user_keys {
        if let Some(Value::Object(user)) = db.get(key) {
            let name = user
                .get("name")
                .map(|v| v.to_string()) //.map(|v| v.to_string()) .Runs only if Some, Converts the value to String, Option<&Value> ---> → Option<String>. If "name" is missing: map is skipped Result is None
                .unwrap_or("?".to_string()); // If Some(String) → use it , If None → use "?"
            let age = user
                .get("age")
                .map(|v| v.to_string())
                .unwrap_or("?".to_string());
            let active = user
                .get("active")
                .map(|v| v.to_string())
                .unwrap_or("?".to_string());
            println!("{}: {} (age: {}, active: {})", key, name, age, active);
        }
    }

    // Query operations
    println!("\n--- Query: Users age > 28 ---");
    let results = db.query(Condition::GreaterThan("age".to_string(), 28));
    for (key, value) in results {
        if let Value::Object(obj) = value {
            let name = obj
                .get("name")
                .map(|v| v.to_string())
                .unwrap_or("?".to_string());
            println!("{}: {}", key, name);
        }
    }

    println!("\n--- Query: Active users ---");
    let active_count = db.query(Condition::Equals(
        "active".to_string(),
        Value::Boolean(true),
    ));
    println!("Found {} active users", active_count.len());

    println!("\n--- Query: Users between age 25-32 ---");
    let results = db.query(Condition::Between("age".to_string(), 25, 32));
    for (key, value) in results {
        if let Value::Object(obj) = value {
            let name = obj
                .get("name")
                .map(|v| v.to_string())
                .unwrap_or("?".to_string());
            let age = obj
                .get("age")
                .map(|v| v.to_string())
                .unwrap_or("?".to_string());
            println!("{}: {} (age: {})", key, name, age);
        }
    }

    // Demonstrate batch operations with auto-save disabled
    println!("\n--- Batch Operations (Auto-save OFF) ---");
    db.set_auto_save(false);

    let batch_data = vec![
        ("product:1".to_string(), Value::String("Laptop".to_string())),
        ("product:2".to_string(), Value::String("Mouse".to_string())),
        (
            "product:3".to_string(),
            Value::String("Keyboard".to_string()),
        ),
    ];

    db.batch_insert(batch_data).unwrap();

    // Manually save after batch
    println!("Manually saving after batch operations...");
    db.save().unwrap();

    // Re-enable auto-save
    db.set_auto_save(true);

    let batch_data_2 = vec![
        ("product:4".to_string(), Value::String("Laptop".to_string())),
        ("product:5".to_string(), Value::String("Mouse".to_string())),
        (
            "product:6".to_string(),
            Value::String("Keyboard".to_string()),
        ),
    ];

    db.batch_insert(batch_data_2).unwrap();

    // Show final stats
    println!("\n--- Final Statistics ---");
    db.stats().print();

    // List all keys
    println!("\n--- All Keys in Database ---");
    let all_keys = db.list_keys();
    println!("Total keys: {}", all_keys.len());
    for key in all_keys {
        println!("  {}", key);
    }

    println!("\n✓ Database saved to disk. Run the program again to see persistence!");
    println!("💡 Tip: The data will still be there after you restart!");
}
