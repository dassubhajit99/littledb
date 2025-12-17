// Main library file that declares all modules

// Declare our modules (each corresponds to a .rs file)

pub mod condition;
pub mod database;
pub mod storage;
pub mod value;

// Re-export commonly used types for convenience
// This allows users to write: use littledb::Database instead of use littledb::database::Database
pub use condition::Condition;
pub use database::Database;
pub use storage::StorageEngine;
pub use value::Value;
