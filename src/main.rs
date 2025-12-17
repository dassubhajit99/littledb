use std::collections::HashMap;

use littledb::{Database, Value};

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
}
