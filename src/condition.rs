use crate::value::Value;
// NEW: Query filter conditions
#[derive(Debug, Clone)]
pub enum Condition {
    Equals(String, Value),    // field equals value
    GreaterThan(String, i64), // field > value (for integers)
    LessThan(String, i64),    // field < value
    Contains(String, String), // string field contains substring
    Between(String, i64, i64),
}

impl Condition {
    // Check if a value matches this condition
    pub fn matches(&self, value: &Value) -> bool {
        match self {
            Condition::Equals(field, expected) => {
                if let Some(actual) = value.get_field(field) {
                    actual == expected
                } else {
                    false
                }
            }
            Condition::GreaterThan(field, threshold) => {
                if let Some(actual) = value.get_field(field) {
                    if let Some(num) = actual.as_integer() {
                        return num > *threshold;
                    }
                }
                false
            }
            Condition::LessThan(field, threshold) => {
                if let Some(actual) = value.get_field(field) {
                    if let Some(num) = actual.as_integer() {
                        return num < *threshold;
                    }
                }
                false
            }
            Condition::Contains(field, substring) => {
                if let Some(actual) = value.get_field(field) {
                    if let Some(s) = actual.as_string() {
                        return s.contains(substring);
                    }
                }
                false
            }

            Condition::Between(field, min, max) => {
                if let Some(actual) = value.get_field(field) {
                    if let Some(num) = actual.as_integer() {
                        return num >= *min && num <= *max;
                    }
                }
                false
            }
        }
    }
}
