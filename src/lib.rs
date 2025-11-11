// Library crate for Rust Math MCP
// Main functionality is in the binary crate in src/main.rs

pub mod service;

// Re-export service for use in tests
pub use service::MathService;
