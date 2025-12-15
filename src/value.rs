//  Defines the Value enum and its methods

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Enum to represent different value types our database can store
// This is like a union of different types
// Serialize and Deserialize let us save/load data to/from disk
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] // These let us copy and print Values . The PartialEq trait allows you to compare instances of a type to check for equality and enables use of the == and != operators. 
pub enum Value {
    // Enums can hold different types of data. Each variant can store its own data type!
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}

impl Value {
    // Convert Value to a displayable string
    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(), // type & because "s" reference to the string stored inside the enum , same for below as well
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();

                format!("{{{}}}", items.join(", "))
            }
            Value::Null => "null".to_string(),
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i), // i is a reference to the integer stored inside the enum, because self is borrowed (&self). the * is the dereference operator.take the value inside the i instead of the reference to it.
            _ => None,
        }
    }

    // Helper to get string if the value is a string
    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<&Value> {
        match self {
            Value::Object(obj) => obj.get(field),
            _ => None,
        }
    }

    // Get the type name as a string (useful for debugging)
    pub fn type_name(&self) -> &str {
        match self {
            Value::String(_) => "String",
            Value::Integer(_) => "Integer",
            Value::Float(_) => "Float",
            Value::Boolean(_) => "Boolean",
            Value::Array(_) => "Array",
            Value::Object(_) => "Object",
            Value::Null => "Null",
        }
    }
}
