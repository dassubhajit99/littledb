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
