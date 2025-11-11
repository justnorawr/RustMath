use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// Custom error type for MCP server operations.
///
/// Implements JSON-RPC 2.0 error codes and provides structured error information.
/// All errors are serializable and can be sent directly to MCP clients.
///
/// # Example
///
/// ```rust
/// use rust_math_mcp::error::McpError;
///
/// let error = McpError::validation_error("Division by zero");
/// assert_eq!(error.code, -32001);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    /// JSON-RPC error code
    pub code: i32,
    /// Human-readable error message
    pub message: String,
    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl McpError {
    /// Create a new MCP error
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    /// Create an error with additional data
    pub fn with_data(code: i32, message: impl Into<String>, data: Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }

    /// Parse error (-32700)
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::new(-32700, message)
    }

    /// Invalid request error (-32600)
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::new(-32600, message)
    }

    /// Method not found error (-32601)
    pub fn method_not_found(method: &str) -> Self {
        Self::new(-32601, format!("Method not found: {}", method))
    }

    /// Invalid params error (-32602)
    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new(-32602, message)
    }

    /// Internal error (-32603)
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(-32603, message)
    }

    /// Tool execution error (custom code -32000)
    pub fn tool_error(message: impl Into<String>) -> Self {
        Self::new(-32000, message)
    }

    /// Validation error (custom code -32001)
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::new(-32001, message)
    }

    /// Resource limit error (custom code -32002)
    pub fn resource_limit(message: impl Into<String>) -> Self {
        Self::new(-32002, message)
    }
}

impl fmt::Display for McpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for McpError {}

/// Result type alias for MCP operations.
///
/// All MCP operations return this type, which provides structured error handling
/// compatible with JSON-RPC 2.0.
pub type McpResult<T> = Result<T, McpError>;

/// Convert from anyhow::Error to McpError
impl From<anyhow::Error> for McpError {
    fn from(err: anyhow::Error) -> Self {
        Self::internal_error(err.to_string())
    }
}

/// Convert from serde_json::Error to McpError
impl From<serde_json::Error> for McpError {
    fn from(err: serde_json::Error) -> Self {
        Self::parse_error(format!("JSON parse error: {}", err))
    }
}

/// Convert from std::io::Error to McpError
impl From<std::io::Error> for McpError {
    fn from(err: std::io::Error) -> Self {
        Self::internal_error(format!("IO error: {}", err))
    }
}
