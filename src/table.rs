// table.rs - Represents a database table with rows and schema

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Condition, Value, schema::Schema};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub schema: Schema,
    pub rows: HashMap<String, Value>,
    next_id: i64,
}

impl Table {
    // Create a new table with a schema
    pub fn new(schema: Schema) -> Self {
        Table {
            schema,
            rows: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn insert(&mut self, key: Option<String>, mut row: Value) -> Result<String, String> {
        // Validate the row against schema
        self.schema.validate_row(&row)?;

        // Generate key if not provided
        let key = match key {
            Some(k) => k,
            None => {
                // Auto-generate key based on primary key or auto-increment
                let pk = format!("{}:{}", self.schema.table_name, self.next_id);
                self.next_id += 1;
                pk
            }
        };
        self.next_id += 1;

        // Check whether the schema defines a primary key.
        // `primary_key()` returns `Option<...>`, so this safely handles the case
        // where no primary key is configured.
        if let Some(pk_col) = self.schema.primary_key() {
            // Ensure the row is a JSON object before attempting mutation.
            // `Value` is an enum, and only `Value::Object` can hold key-value pairs.
            //
            // `ref mut obj` borrows the inner map mutably without moving it out of `row`,
            // allowing us to modify the object in place while keeping `row` valid.
            if let Value::Object(ref mut obj) = row {
                // Avoid overwriting an existing primary key value.
                // Only auto-generate the primary key if it is not already present.
                if !obj.contains_key(&key) {
                    // Insert an auto-generated primary key value.
                    //
                    // `pk_col.name.clone()`:
                    // - `insert` requires ownership of the key
                    // - we clone because `pk_col` is still in use and cannot be moved from
                    //
                    // `(self.next_id - 1) as i64`:
                    // - assigns the previously generated ID to this row
                    // - explicit cast is required to match JSON integer (`i64`)
                    // example , if primary key field is exists but it value is not set the add default
                    obj.insert(
                        pk_col.name.clone(),
                        Value::Integer((self.next_id - 1) as i64),
                    );
                }
            }
        }
        self.rows.insert(key.clone(), row);
        Ok(key)
    }

    // Get a row by key
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.rows.get(key)
    }

    // Update a row
    pub fn update(&mut self, key: &str, new_row: Value) -> Result<(), String> {
        if !self.rows.contains_key(key) {
            return Err(format!("Row with key '{}' not found", key));
        }
        self.schema.validate_row(&new_row)?;

        self.rows.insert(key.to_string(), new_row);

        Ok(())
    }

    // Delete a row
    pub fn delete(&mut self, key: &str) -> Result<(), String> {
        // if !self.rows.contains_key(key) {
        //     return Err(format!("Row with key '{}' not found", key));
        // }
        // self.rows.remove(key);

        if self.rows.remove(key).is_none() {
            return Err(format!("Row with key '{}' not found", key));
        }

        Ok(())
    }

    // Select all rows
    pub fn select_all(&self) -> Vec<(String, Value)> {
        self.rows
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    // Select rows matching a condition
    pub fn select_where(&self, condition: Condition) -> Vec<(String, Value)> {
        self.rows
            .iter()
            .filter(|(_k, v)| condition.matches(v))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    // Select rows with multiple conditions (AND)
    pub fn select_where_multiple(&self, conditions: Vec<Condition>) -> Vec<(String, Value)> {
        self.rows
            .iter()
            .filter(|(_k, v)| conditions.iter().all(|c| c.matches(v)))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    // Count rows
    pub fn count(&self) -> usize {
        self.rows.len()
    }

    // Get table statistics
    pub fn stats(&self) -> TableStats {
        TableStats {
            name: self.schema.table_name.clone(),
            row_count: self.count(),
            column_count: self.schema.columns.len(),
        }
    }
}

pub struct TableStats {
    pub name: String,
    pub row_count: usize,
    pub column_count: usize,
}

impl TableStats {
    pub fn display(&self) {
        println!("Table: {}", self.name);
        println!("  Rows: {}", self.row_count);
        println!("  Columns: {}", self.column_count);
    }
}
