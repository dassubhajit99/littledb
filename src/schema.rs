use serde::{Deserialize, Serialize};

use crate::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Text,
    Integer,
    Float,
    Boolean,
    Null,
}

impl DataType {
    // Check if a Value matches this data type
    pub fn matches(&self, value: &Value) -> bool {
        match (self, value) {
            (DataType::Text, Value::String(_)) => true,
            (DataType::Integer, Value::Integer(_)) => true,
            (DataType::Float, Value::Float(_)) => true,
            (DataType::Boolean, Value::Boolean(_)) => true,
            (DataType::Null, Value::Null) => true,
            (_, Value::Null) => true, // Allow null values for any type
            _ => false,
        }
    }

    // Get the type name as a string
    pub fn name(&self) -> &str {
        match self {
            DataType::Text => "TEXT",
            DataType::Integer => "INTEGER",
            DataType::Float => "FLOAT",
            DataType::Boolean => "BOOLEAN",
            DataType::Null => "NULL",
        }
    }

    // Parse a string into a DataType
    pub fn from_str(s: &str) -> Option<DataType> {
        match s.to_uppercase().as_str() {
            "TEXT" | "STRING" | "VARCHAR" => Some(DataType::Text),
            "INTEGER" | "INT" => Some(DataType::Integer),
            "FLOAT" | "REAL" | "DOUBLE" => Some(DataType::Float),
            "BOOLEAN" | "BOOL" => Some(DataType::Boolean),
            "NULL" => Some(DataType::Null),
            _ => None,
        }
    }
}

// Column definition in a table
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub primary_key: bool,
}

impl Column {
    pub fn new(name: String, data_type: DataType) -> Self {
        Column {
            name,
            data_type,
            nullable: true,
            primary_key: false,
        }
    }
    // Create a non-nullable column
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    // Mark as primary key
    pub fn primary(mut self) -> Self {
        self.primary_key = true;
        self
    }

    // Validate if a value is acceptable for this column
    pub fn validate(&self, value: &Value) -> Result<(), String> {
        if matches!(value, Value::Null) {
            if !self.nullable {
                return Err(format!("Column '{}' cannot be null", self.name));
            }
            return Ok(());
        }

        if !self.data_type.matches(value) {
            return Err(format!(
                "Column '{}' expects {} but got {}",
                self.name,
                self.data_type.name(),
                value.type_name()
            ));
        }

        Ok(())
    }
}

// Schema defines the structure of a table
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    pub table_name: String,
    pub columns: Vec<Column>,
}

impl Schema {
    pub fn new(table_name: String) -> Self {
        Schema {
            table_name,
            columns: Vec::new(),
        }
    }

    // Add a column to the schema
    pub fn add_column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    // Get a column by name
    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == name)
    }

    // Get the primary key column
    pub fn primary_key(&self) -> Option<&Column> {
        self.columns.iter().find(|c| c.primary_key)
    }

    // Validate a row (Object) against this schema
    pub fn validate_row(&self, row: &Value) -> Result<(), String> {
        let obj = match row {
            Value::Object(o) => o,
            _ => return Err("Row must be an object".to_string()),
        };

        // Check each column in the schema
        for column in &self.columns {
            match obj.get(&column.name) {
                Some(value) => column.validate(value)?,
                None => {
                    // Column is missing
                    if !column.nullable {
                        return Err(format!("Missing required column '{}'", column.name));
                    }
                }
            }
        }
        Ok(())
    }

    // Get column names as a list
    pub fn column_names(&self) -> Vec<String> {
        self.columns.iter().map(|c| c.name.clone()).collect()
    }

    // Display schema as a formatted string
    pub fn display(&self) -> String {
        let mut result = format!("Table: {}\n", self.table_name);
        result.push_str("Columns:\n");

        for column in &self.columns {
            let mut flags = Vec::new();

            if column.primary_key {
                flags.push("PRIMARY KEY");
            }

            if !column.nullable {
                flags.push("NOT NULL")
            }

            let flag_str = if flags.is_empty() {
                String::new()
            } else {
                format!(" ({})", flags.join(", "))
            };
            result.push_str(&format!(
                " - {} {}{}\n",
                column.name,
                column.data_type.name(),
                flag_str
            ));
        }
        result
    }
}
