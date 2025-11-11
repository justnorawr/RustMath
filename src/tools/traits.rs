use crate::error::McpResult;
use serde_json::Value;

/// Trait for tool registry operations.
///
/// This trait allows for different implementations of tool storage and execution,
/// enabling flexibility in how tools are managed and dispatched.
///
/// # Example
///
/// ```rust
/// use rust_math_mcp::tools::{DefaultToolRegistry, ToolRegistry};
/// use serde_json::json;
///
/// let registry = DefaultToolRegistry;
/// let tools = registry.get_all_tools();
/// let result = registry.execute_tool("add", &json!({ "numbers": [1, 2, 3] })).unwrap();
/// assert_eq!(result["result"], 6.0);
/// ```
pub trait ToolRegistry {
    /// Get all tool definitions as a JSON array.
    ///
    /// Returns a `Value` containing an array of tool definition objects,
    /// each with `name`, `description`, and `inputSchema` fields.
    fn get_all_tools(&self) -> Value;
    
    /// Execute a tool by name with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool to execute
    /// * `arguments` - JSON object containing the tool's input parameters
    ///
    /// # Returns
    ///
    /// A `McpResult` containing the tool's result as a JSON `Value`, or an error
    /// if the tool is not found or execution fails.
    fn execute_tool(&self, name: &str, arguments: &Value) -> McpResult<Value>;
}

