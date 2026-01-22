use regex::Regex;

use crate::{Value, schema::DataType};

#[derive(Debug, Clone)]
pub enum SqlCommand {
    CreateTable {
        table_name: String,
        columns: Vec<(String, DataType, bool, bool)>, // (name, type, is_primary_key, is_nullable)
    },
    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<Value>,
    },
    Select {
        table_name: String,
        columns: Vec<String>, // Empty vec means SELECT *
        where_clause: Option<WhereClause>,
    },
    Update {
        table_name: String,
        set_values: Vec<(String, Value)>, // column -> new value
        where_clause: Option<WhereClause>,
    },
    Delete {
        table_name: String,
        where_clause: Option<WhereClause>,
    },

    DropTable {
        table_name: String,
    },
}

// WHERE clause representation
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub column: String,
    pub operator: String, // "=", ">", "<", ">=", "<=", "!="
    pub value: Value,
}

pub struct SqlParser;

impl SqlParser {
    // Main parsing function
    pub fn parse(sql: &str) -> Result<SqlCommand, String> {
        let sql = sql.trim(); //"   CREATE TABLE users   ".trim() ---> "CREATE TABLE users"
        let command_type = sql
            .split_whitespace()
            .next()
            .ok_or("Empty SQL command")?
            .to_uppercase();

        /*
                Rust pipeline

                sql.split_whitespace()

                "CREATE TABLE users (id INTEGER)" ---> ["CREATE", "TABLE", "users", "(id", "INTEGER)"]

                .next() //Gets the first token:

                Some("CREATE")

                If SQL is empty: "" -> split_whitespace() -> next() -> None


                .ok_or("Empty SQL command")?

                Converts Option<T> → Result<T, E>
                If None, return early with error
                ? propagates the error

                Equivalent expanded version:
                let first = match sql.split_whitespace().next() {
            Some(v) => v,
            None => return Err("Empty SQL command".to_string()),
        };

                 */

        match command_type.as_str() {
            "CREATE" => Self::parse_create_table(sql),
            "INSERT" => Self::parse_insert(sql),
            "SELECT" => Self::parse_select(sql),
            "UPDATE" => Self::parse_update(sql),
            "DELETE" => Self::parse_delete(sql),
            "DROP" => Self::parse_drop(sql),
            _ => Err(format!("Unknown command: {}", command_type)),
        }
    }

    // Parse: CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER)
    pub fn parse_create_table(sql: &str) -> Result<SqlCommand, String> {
        let re = Regex::new(r"CREATE\s+TABLE\s+(\w+)\s*\((.+)\)").map_err(|e| e.to_string())?;

        let caps = re.captures(sql).ok_or("Invalid CREATE TABLE syntax")?;

        let table_name = caps
            .get(1)
            .ok_or("Missing table name")?
            .as_str()
            .to_string();

        let columns_str = caps.get(2).ok_or("Missing column definitions")?.as_str();

        // Parse column definitions
        let mut columns = Vec::new();
        for col_def in columns_str.split(',') {
            let col_def = col_def.trim();
            let parts: Vec<&str> = col_def.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(format!("Invalid column definition: {}", col_def));
            }

            let col_name = parts[0].to_string();
            let col_type =
                DataType::from_str(parts[1]).ok_or(format!("Unknown data type: {}", parts[1]))?;

            // Check for PRIMARY KEY
            let is_primary = col_def.to_uppercase().contains("PRIMARY KEY");
            let is_nullable = !col_def.to_uppercase().contains("NOT NULL");

            columns.push((col_name, col_type, is_primary, is_nullable));
        }

        Ok(SqlCommand::CreateTable {
            table_name,
            columns,
        })
    }

    // Parse: INSERT INTO users (name, age) VALUES ('Alice', 30)
    fn parse_insert(sql: &str) -> Result<SqlCommand, String> {
        let re = Regex::new(r"INSERT\s+INTO\s+(\w+)\s*\(([^)]+)\)\s*VALUES\s*\(([^)]+)\)")
            .map_err(|e| e.to_string())?;

        let caps = re.captures(sql).ok_or("Invalid INSERT syntax")?;

        let table_name = caps
            .get(1)
            .ok_or("Missing table name")?
            .as_str()
            .to_string();

        let columns_str = caps.get(2).ok_or("Missing columns")?.as_str();

        let values_str = caps.get(3).ok_or("Missing values")?.as_str();

        // Parse columns
        let columns: Vec<String> = columns_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Parse values
        let values: Vec<Value> = values_str
            .split(',')
            .map(|s| Self::parse_value(s.trim()))
            .collect::<Result<Vec<_>, _>>()?;

        if columns.len() != values.len() {
            return Err("Column count doesn't match value count".to_string());
        }

        Ok(SqlCommand::Insert {
            table_name,
            columns,
            values,
        })
    }

    // Parse a value: 'Alice', 30, 3.14, true, null
    fn parse_value(s: &str) -> Result<Value, String> {
        let s = s.trim();

        // String (quoted)
        if (s.starts_with('\'') && s.ends_with('\'')) || (s.starts_with('"') && s.ends_with('"')) {
            let content = &s[1..s.len() - 1];
            return Ok(Value::String(content.to_string()));
        }

        // Boolean
        if s.to_lowercase() == "true" {
            return Ok(Value::Boolean(true));
        }

        if s.to_lowercase() == "false" {
            return Ok(Value::Boolean(false));
        }

        // Null
        if s.to_lowercase() == "null" {
            return Ok(Value::Null);
        }

        // integer
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Value::Integer(i));
        }

        // Float
        if let Ok(f) = s.parse::<f64>() {
            return Ok(Value::Float(f));
        }

        Err(format!("Cannot parse value: {}", s))
    }

    // Parse: SELECT * FROM users WHERE age > 25
    // Parse: SELECT name, age FROM users
    fn parse_select(sql: &str) -> Result<SqlCommand, String> {
        let re = Regex::new(r"SELECT\s+(.+?)\s+FROM\s+(\w+)(?:\s+WHERE\s+(.+))?")
            .map_err(|e| e.to_string())?;

        let caps = re.captures(sql).ok_or("Invalid SELECT syntax")?;

        let columns_str = caps.get(1).ok_or("Missing Columns")?.as_str().trim();

        let table_name = caps
            .get(2)
            .ok_or("Missing Table Name")?
            .as_str()
            .to_string();

        let where_str = caps.get(3);

        // Parse columns
        let columns = if columns_str == "*" {
            Vec::new() // Empty means all columns
        } else {
            columns_str
                .split(",")
                .map(|s| s.trim().to_string())
                .collect()
        };

        // Parse WHERE clause if present
        let where_clause = if let Some(where_condition) = where_str {
            Some(Self::parse_where(where_condition.as_str())?)
        } else {
            None
        };

        Ok(SqlCommand::Select {
            table_name,
            columns,
            where_clause,
        })
    }

    // Parse WHERE clause: age > 25
    fn parse_where(where_str: &str) -> Result<WhereClause, String> {
        let re = Regex::new(r"(\w+)\s*(=|>|<|>=|<=|!=)\s*(.+)").map_err(|e| e.to_string())?;

        let caps = re
            .captures(where_str)
            .ok_or(format!("Invalid WHERE clause: {}", where_str))?;

        let column = caps
            .get(1)
            .ok_or("Missing column in WHERE")?
            .as_str()
            .to_string();

        let operator = caps
            .get(2)
            .ok_or("Missing operator in WHERE")?
            .as_str()
            .to_string();

        let value_str = caps.get(3).ok_or("Missing value in WHERE")?.as_str();

        let value = Self::parse_value(value_str)?;

        Ok(WhereClause {
            column,
            operator,
            value,
        })
    }

    // Parse: UPDATE users SET name = 'Bob' WHERE id = 1
    fn parse_update(sql: &str) -> Result<SqlCommand, String> {
        let re = Regex::new(r"UPDATE\s+(\w+)\s+SET\s+(.+?)(?:\s+WHERE\s+(.+))?")
            .map_err(|e| e.to_string())?;

        let caps = re.captures(sql).ok_or("Invalid UPDATE syntax")?;

        let table_name = caps
            .get(1)
            .ok_or("Missing table name")?
            .as_str()
            .to_string();

        let set_str = caps.get(2).ok_or("Missing SET clause")?.as_str();

        // Parse SET values
        let mut set_values = Vec::new();

        for assignment in set_str.split(',') {
            let parts: Vec<&str> = assignment.split('=').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid assignment: {}", assignment));
            }

            let column = parts[0].trim().to_string();
            let value = Self::parse_value(parts[1].trim())?;

            set_values.push((column, value));
        }

        // Parse WHERE clause if present
        let where_clause = if let Some(where_str) = caps.get(3) {
            Some(Self::parse_where(where_str.as_str())?)
        } else {
            None
        };

        Ok(SqlCommand::Update {
            table_name,
            set_values,
            where_clause,
        })
    }

    // Parse: DELETE FROM users WHERE id = 1
    fn parse_delete(sql: &str) -> Result<SqlCommand, String> {
        let re =
            Regex::new(r"DELETE\s+FROM\s+(\w+)(?:\s+WHERE\s+(.+))?").map_err(|e| e.to_string())?;

        let caps = re.captures(sql).ok_or("Invalid DELETE syntax")?;

        let table_name = caps
            .get(1)
            .ok_or("Missing table name")?
            .as_str()
            .to_string();

        let where_clause = if let Some(where_str) = caps.get(2) {
            Some(Self::parse_where(where_str.as_str())?)
        } else {
            None
        };

        Ok(SqlCommand::Delete {
            table_name,
            where_clause,
        })
    }

    // Parse: DROP TABLE users
    fn parse_drop(sql: &str) -> Result<SqlCommand, String> {
        let re = Regex::new(r"DROP\s+TABLE\s+(\w+)").map_err(|e| e.to_string())?;

        let caps = re.captures(sql).ok_or("Invalid DROP TABLE syntax")?;

        let table_name = caps
            .get(1)
            .ok_or("Missing table name")?
            .as_str()
            .to_string();

        Ok(SqlCommand::DropTable { table_name })
    }
}
