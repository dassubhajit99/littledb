// Main library file that declares all modules

// Declare our modules (each corresponds to a .rs file)

pub mod condition;
pub mod database;
pub mod schema;
pub mod sql_executor;
pub mod sql_parser;
pub mod storage;
pub mod table;
pub mod value;

// Re-export commonly used types for convenience
// This allows users to write: use littledb::Database instead of use littledb::database::Database
pub use condition::Condition;
pub use database::Database;
pub use schema::{Column, DataType, Schema};
pub use sql_executor::{QueryResult, SqlDatabase};
pub use storage::StorageEngine;
pub use table::Table;
pub use value::Value;
