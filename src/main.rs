use littledb::SqlDatabase;

fn main() {
    println!("=== RustDB Stage 3: SQL Query Parser ===\n");

    let mut db = SqlDatabase::new();

    // CREATE TABLE
    println!("--- Creating Tables ---");
    let create_users =
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER, email TEXT)";

    match db.execute(create_users) {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    let create_products = "CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT, price FLOAT)";

    match db.execute(create_products) {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // INSERT DATA
    println!("\n--- Inserting Data ---");

    let inserts = vec![
        "INSERT INTO users (name, age, email) VALUES ('Alice', 30, 'alice@example.com')",
        "INSERT INTO users (name, age, email) VALUES ('Bob', 25, 'bob@example.com')",
        "INSERT INTO users (name, age, email) VALUES ('Charlie', 35, 'charlie@example.com')",
        "INSERT INTO users (name, age, email) VALUES ('Diana', 28, 'diana@example.com')",
        "INSERT INTO products (name, price) VALUES ('Laptop', 999.99)",
        "INSERT INTO products (name, price) VALUES ('Mouse', 29.99)",
        "INSERT INTO products (name, price) VALUES ('Keyboard', 79.99)",
    ];

    for sql in inserts {
        match db.execute(sql) {
            Ok(result) => result.display(),
            Err(e) => println!("✗ Error: {}", e),
        }
    }
}
