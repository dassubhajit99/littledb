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

    // SELECT ALL
    println!("\n--- SELECT * FROM users ---");
    match db.execute("SELECT * FROM users") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n--- SELECT * FROM products ---");
    match db.execute("SELECT * FROM products") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // SELECT SPECIFIC COLUMNS
    println!("\n--- SELECT name, age FROM users ---");
    match db.execute("SELECT name, age FROM users") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // SELECT WITH WHERE
    println!("\n--- SELECT * FROM users WHERE age > 28 ---");
    match db.execute("SELECT * FROM users WHERE age > 28") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n--- SELECT * FROM users WHERE age < 30 ---");
    match db.execute("SELECT * FROM users WHERE age < 30") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // UPDATE
    println!("\n--- UPDATE users SET age = 31 WHERE name = 'Alice' ---");
    match db.execute("UPDATE users SET age = 31 WHERE name = 'Alice'") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Verify update
    println!("\n--- Verifying Update: SELECT * FROM users WHERE name = 'Alice' ---");
    match db.execute("SELECT * FROM users WHERE name = 'Alice'") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // SELECT FROM products
    println!("\n--- SELECT * FROM products ---");
    match db.execute("SELECT * FROM products") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // UPDATE products
    println!("\n--- UPDATE products SET price = 899.99 WHERE name = 'Laptop' ---");
    match db.execute("UPDATE products SET price = 899.99 WHERE name = 'Laptop'") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // DELETE
    println!("\n--- DELETE FROM users WHERE age < 27 ---");
    match db.execute("DELETE FROM users WHERE age < 27") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Verify delete
    println!("\n--- After Delete: SELECT * FROM users ---");
    match db.execute("SELECT * FROM users") {
        Ok(result) => result.display(),
        Err(e) => println!("✗ Error: {}", e),
    }

    // List all tables
    println!("\n--- All Tables in Database ---");
    let tables = db.list_tables();
    println!("Tables: {:?}", tables);
}
