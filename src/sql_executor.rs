use std::collections::HashMap;

use crate::{
    Condition, Value,
    schema::{Column, DataType, Schema},
    sql_parser::{SqlCommand, WhereClause},
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

    // Execute a SQL command
    pub fn execute(&mut self, sql: &str) -> Result<QueryResult, String> {
        // Parse the SQL
        let command = crate::sql_parser::SqlParser::parse(sql)?;

        // Execute based on command type
        match command {
            SqlCommand::CreateTable {
                table_name,
                columns,
            } => self.execute_create_table(table_name, columns),
            SqlCommand::Insert {
                table_name,
                columns,
                values,
            } => self.execute_insert(table_name, columns, values),
            SqlCommand::Select {
                table_name,
                columns,
                where_clause,
            } => self.execute_select(table_name, columns, where_clause),
            SqlCommand::Update {
                table_name,
                set_values,
                where_clause,
            } => self.execute_update(table_name, set_values, where_clause),
            SqlCommand::Delete {
                table_name,
                where_clause,
            } => self.execute_delete(table_name, where_clause),
            SqlCommand::DropTable { table_name } => self.execute_drop_table(table_name),
        }
    }

    // CREATE TABLE
    fn execute_create_table(
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
    fn execute_insert(
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

        table.insert(None, row_obj)?;

        Ok(QueryResult::Success {
            message: "Row inserted".to_string(),
            rows_affected: 1,
        })
    }

    fn execute_select(
        &mut self,
        table_name: String,
        columns: Vec<String>,
        where_clause: Option<WhereClause>,
    ) -> Result<QueryResult, String> {
        let table = self
            .tables
            .get(&table_name)
            .ok_or(format!("Table '{}' does not exist", table_name))?;

        // Get matching rows
        let rows = if let Some(where_clause) = where_clause {
            let condition = Self::where_to_condition(where_clause)?;
            table.select_where(condition)
        } else {
            table.select_all()
        };

        // Filter columns if specific columns requested
        let filtered_rows = if columns.is_empty() {
            // SELECT * - return all columns
            rows
        } else {
            rows.into_iter()
                .map(|(key, value)| {
                    if let Value::Object(obj) = value {
                        let filtered_obj: HashMap<String, Value> = obj
                            .into_iter()
                            .filter(|(k, _)| columns.contains(k))
                            .collect();
                        (key, Value::Object(filtered_obj))
                    } else {
                        (key, value)
                    }
                })
                .collect()
        };
        let row_count = filtered_rows.len();
        Ok(QueryResult::Select {
            rows: filtered_rows,
            row_count: row_count,
        })
    }

    // UPDATE
    fn execute_update(
        &mut self,
        table_name: String,
        set_values: Vec<(String, Value)>,
        where_clause: Option<WhereClause>,
    ) -> Result<QueryResult, String> {
        let table = self
            .tables
            .get_mut(&table_name)
            .ok_or(format!("Table '{}' does not exist", table_name))?;

        // Find rows to update
        // Find rows to update
        let keys_to_update: Vec<String> = if let Some(where_clause) = where_clause {
            let condition = Self::where_to_condition(where_clause)?;
            table
                .select_where(condition)
                .into_iter()
                .map(|(k, _)| k)
                .collect()
        } else {
            table.select_all().into_iter().map(|(k, _)| k).collect()
        };

        let mut updated = 0;

        // Update each row
        for key in keys_to_update {
            if let Some(Value::Object(mut obj)) = table.get(&key).cloned() {
                // Apply updates
                for (col, val) in &set_values {
                    obj.insert(col.clone(), val.clone());
                }

                table.update(&key, Value::Object(obj))?;
                updated += 1;
            }
        }

        Ok(QueryResult::Success {
            message: format!("Updated {} row(s)", updated),
            rows_affected: updated,
        })
    }

    // DELETE
    fn execute_delete(
        &mut self,
        table_name: String,
        where_clause: Option<WhereClause>,
    ) -> Result<QueryResult, String> {
        let table = self
            .tables
            .get_mut(&table_name)
            .ok_or(format!("Table '{}' does not exist", table_name))?;

        // Find rows to delete
        let keys_to_delete: Vec<String> = if let Some(where_clause) = where_clause {
            let condition = Self::where_to_condition(where_clause)?;
            table
                .select_where(condition)
                .into_iter()
                .map(|(k, _)| k)
                .collect()
        } else {
            table.select_all().into_iter().map(|(k, _)| k).collect()
        };

        let count = keys_to_delete.len();

        // Delete each row
        for key in keys_to_delete {
            table.delete(&key)?;
        }

        Ok(QueryResult::Success {
            message: format!("Deleted {} row(s)", count),
            rows_affected: count,
        })
    }

    // DROP TABLE
    fn execute_drop_table(&mut self, table_name: String) -> Result<QueryResult, String> {
        if self.tables.remove(&table_name).is_none() {
            return Err(format!("Table '{}' does not exist", table_name));
        }

        Ok(QueryResult::Success {
            message: format!("Table '{}' dropped", table_name),
            rows_affected: 0,
        })
    }

    // Convert WHERE clause to Condition
    fn where_to_condition(where_clause: WhereClause) -> Result<Condition, String> {
        match where_clause.operator.as_str() {
            "=" => Ok(Condition::Equals(where_clause.column, where_clause.value)),
            ">" => {
                if let Value::Integer(i) = where_clause.value {
                    Ok(Condition::GreaterThan(where_clause.column, i))
                } else {
                    Err("Operator '>' requires integer value".to_string())
                }
            }
            "<" => {
                if let Value::Integer(i) = where_clause.value {
                    Ok(Condition::LessThan(where_clause.column, i))
                } else {
                    Err("Operator '<' requires integer value".to_string())
                }
            }
            ">=" => {
                if let Value::Integer(i) = where_clause.value {
                    Ok(Condition::GreaterOrEqual(where_clause.column, i))
                } else {
                    Err("Operator '>=' requires integer value".to_string())
                }
            }
            "<=" => {
                if let Value::Integer(i) = where_clause.value {
                    Ok(Condition::LessOrEqual(where_clause.column, i))
                } else {
                    Err("Operator '<=' requires integer value".to_string())
                }
            }
            _ => Err(format!("Unsupported operator: {}", where_clause.operator)),
        }
    }

    // Get a table
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    // List all table names
    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
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

                            let mut parts = Vec::new();

                            if let Some(id) = obj.get("id") {
                                parts.push(format!("id: {}", id.to_string()));
                            }

                            let mut keys: Vec<_> =
                                obj.keys().filter(|k| k.as_str() != "id").collect();

                            keys.sort();

                            for key in keys {
                                if let Some(value) = obj.get(key) {
                                    parts.push(format!("{}: {}", key, value.to_string()));
                                }
                            }

                            println!("{}", parts.join(", "));
                        }
                    }
                }
            }
        }
    }
}
