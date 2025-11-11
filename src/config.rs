use std::env;

/// Server configuration.
///
/// Provides configurable settings for the MCP server, with defaults
/// that can be overridden via environment variables.
///
/// # Example
///
/// ```rust
/// use rust_math_mcp::config::Config;
///
/// let config = Config::new();
/// assert_eq!(config.server_name(), "rust-math-mcp");
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Server name
    pub server_name: String,
    /// Server version
    pub server_version: String,
    /// Maximum array size for tool inputs
    pub max_array_size: usize,
    /// Maximum number of decimal places for rounding
    pub max_decimal_places: i32,
    /// Enable rate limiting
    pub enable_rate_limit: bool,
    /// Maximum requests per second (when rate limiting enabled)
    pub max_requests_per_second: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_name: env::var("MCP_SERVER_NAME")
                .unwrap_or_else(|_| crate::protocol::constants::server::DEFAULT_NAME.to_string()),
            server_version: env::var("MCP_SERVER_VERSION").unwrap_or_else(|_| {
                crate::protocol::constants::server::DEFAULT_VERSION.to_string()
            }),
            max_array_size: env::var("MCP_MAX_ARRAY_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10_000),
            max_decimal_places: env::var("MCP_MAX_DECIMAL_PLACES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15),
            enable_rate_limit: env::var("MCP_ENABLE_RATE_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true), // Enabled by default for security
            max_requests_per_second: env::var("MCP_MAX_REQUESTS_PER_SECOND")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
        }
    }
}

impl Config {
    /// Create a new configuration instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the server name
    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    /// Get the server version
    pub fn server_version(&self) -> &str {
        &self.server_version
    }
}
