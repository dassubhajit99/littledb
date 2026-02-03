# LittleDB 🗄️

A lightweight SQL database implementation in Rust for learning purposes.

## Features

- **SQL Support**: CREATE TABLE, INSERT, SELECT, UPDATE, DELETE, DROP TABLE
- **Data Types**: TEXT, INTEGER, FLOAT, BOOLEAN, NULL
- **Schema Validation**: Primary keys, NOT NULL constraints, type checking
- **Persistent Storage**: Binary serialization with bincode
- **WHERE Clauses**: Filtering with `=`, `>`, `<`, `>=`, `<=` operators

## Quick Start

```rust
use littledb::SqlDatabase;

fn main() ->  {
    let mut db = SqlDatabase::new();

    // Create table
    db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER)")?;

    // Insert data
    db.execute("INSERT INTO users (id, name, age) VALUES (1, 'Alice', 30)")?;

    // Query data
    db.execute("SELECT * FROM users WHERE age > 25")?;

    // Update
    db.execute("UPDATE users SET age = 31 WHERE name = 'Alice'")?;

    // Delete
    db.execute("DELETE FROM users WHERE id = 1")?;

    Ok(())
}
```

## SQL Examples

```sql
-- Create a table
CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL, price FLOAT)

-- Insert rows
INSERT INTO products (id, name, price) VALUES (1, 'Laptop', 999.99)

-- Query with WHERE
SELECT * FROM products WHERE price > 500

-- Update rows
UPDATE products SET price = 899.99 WHERE id = 1

-- Delete rows
DELETE FROM products WHERE price < 100

-- Drop table
DROP TABLE products
```

## Architecture

- **`Value`**: Enum representing all data types
- **`Schema`**: Table structure with column definitions
- **`Table`**: In-memory table with CRUD operations
- **`StorageEngine`**: Binary persistence layer
- **`SqlParser`**: SQL command parser
- **`SqlExecutor`**: Query execution engine
