use std::collections::HashMap;

use crate::{
    Value,
    schema::{Column, DataType, Schema},
    table::Table,
};

// Database that holds multiple tables
pub struct SqlDatabase {
    tables: HashMap<String, Table>,
}

impl SqlDatabase {
    pub fn new() -> Self {
        SqlDatabase {
            tables: HashMap::new(),
        }
    }

    // CREATE TABLE
    pub fn execute_create_table(
        &mut self,
        table_name: String,
        columns: Vec<(String, DataType, bool, bool)>,
    ) -> Result<QueryResult, String> {
        if self.tables.contains_key(&table_name) {
            return Err(format!("Table '{}' already exists", table_name));
        }

        let mut schema = Schema::new(table_name.clone());

        for (col_name, col_type, is_primary, is_nullable) in columns {
            let mut column = Column::new(col_name, col_type);

            if is_primary {
                column = column.primary();
            }

            if is_nullable {
                column = column.not_null()
            }

            schema.add_column(column);
        }

        let table = Table::new(schema);
        self.tables.insert(table_name.clone(), table);

        Ok(QueryResult::Success {
            message: format!("Table '{}' created", table_name),
            rows_affected: 0,
        })
    }

    // INSERT
    pub fn execute_insert(
        &mut self,
        table_name: String,
        columns: Vec<String>,
        values: Vec<Value>,
    ) -> Result<QueryResult, String> {
        let table = self
            .tables
            .get_mut(&table_name)
            .ok_or(format!("Table '{}' does not exist", table_name))?;

        // Build row object

        let mut row_obj = HashMap::new();

        for (col, val) in columns.iter().zip(values.iter()) {
            row_obj.insert(col.clone(), val.clone());
        }

        let row = Value::Object(row_obj);
        table.insert(None, row)?;

        Ok(QueryResult::Success {
            message: "Row inserted".to_string(),
            rows_affected: 1,
        })
    }
}

// Query result types
#[derive(Debug)]
pub enum QueryResult {
    Success {
        message: String,
        rows_affected: usize,
    },
    Select {
        rows: Vec<(String, Value)>,
        row_count: usize,
    },
}

impl QueryResult {
    pub fn display(&self) {
        match self {
            QueryResult::Success {
                message,
                rows_affected,
            } => {
                println!("✓ {}", message);

                if *rows_affected > 0 {
                    println!("  Rows affected: {}", rows_affected);
                }
            }

            QueryResult::Select { rows, row_count } => {
                println!("Query returned {} row(s):\n", row_count);

                if rows.is_empty() {
                    println!("(No rows)");
                } else {
                    for (key, value) in rows {
                        if let Value::Object(obj) = value {
                            print!("[{}] ", key);
                            let items: Vec<String> = obj
                                .iter()
                                .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                                .collect();
                            println!("{}", items.join(", "));
                        }
                    }
                }
            }
        }
    }
}
