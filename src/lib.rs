pub mod config;
pub mod error;
pub mod protocol;
pub mod tools;
pub mod utils;

// Re-export commonly used types
pub use error::{McpError, McpResult};
pub use tools::{DefaultToolRegistry, ToolRegistry};

