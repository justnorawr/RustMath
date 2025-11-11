pub mod constants;
pub mod parser;

use crate::config::Config;
use crate::error::{McpError, McpResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, Write};
use std::sync::Arc;
use tracing::{debug, error, instrument, span, Level};

pub use constants::*;

/// JSON-RPC 2.0 request structure.
///
/// Represents an incoming request from an MCP client.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

impl JsonRpcRequest {
    /// Validate the JSON-RPC version
    pub fn validate(&self) -> McpResult<()> {
        if self.jsonrpc != constants::JSON_RPC_VERSION {
            return Err(McpError::invalid_request(format!(
                "Invalid JSON-RPC version: expected '{}', got '{}'",
                constants::JSON_RPC_VERSION,
                self.jsonrpc
            )));
        }
        Ok(())
    }
}

/// JSON-RPC response structure
/// 
/// Note: For request responses, `id` must match the request ID.
/// For notifications (requests without ID), `id` should be None.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    /// Request ID - must be present for request responses (not null)
    /// Can be None only for parse errors or notifications
    /// Note: Claude Desktop requires id to be present (not null) for request responses
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error structure
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl From<crate::error::McpError> for JsonRpcError {
    fn from(err: crate::error::McpError) -> Self {
        Self {
            code: err.code,
            message: err.message,
            data: err.data,
        }
    }
}

/// Server information
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// Initialize parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: Value,
    pub client_info: Value,
}

/// Initialize result
#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResult {
    pub protocol_version: String,
    pub capabilities: Value,
    pub server_info: ServerInfo,
}

/// Tool call parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: Value,
}

/// Send a JSON-RPC response to stdout.
///
/// Formats the response according to MCP protocol:
/// - Content-Length header
/// - Blank line
/// - JSON message
///
/// # Arguments
///
/// * `response` - The JSON-RPC response to send
#[instrument(skip(response))]
pub fn send_response(response: JsonRpcResponse) -> McpResult<()> {
    // Validate response format for Claude Desktop compatibility
    // Claude Desktop requires id to be non-null for request responses
    // (null is only acceptable for parse errors per JSON-RPC 2.0 spec)
    if response.id.is_none() && response.error.is_none() {
        // This is a request response without an ID - log warning but allow it
        // (might be a notification response, which shouldn't happen per spec)
        debug!("Warning: Response without ID (might be notification response)");
    }
    
    let json = serde_json::to_string(&response)?;
    let content_length = json.len();
    
    debug!("Sending response: {} bytes, id={:?}", content_length, response.id);
    debug!("Response JSON: {}", json);
    
    // MCP protocol format: Content-Length header, blank line, then JSON
    // Use write! instead of println! for more control and to avoid any extra formatting
    // Ensure no extra newlines or characters are added
    write!(io::stdout(), "Content-Length: {}\r\n\r\n{}", content_length, json)
        .map_err(|e| McpError::internal_error(format!("Failed to write response: {}", e)))?;
    io::stdout().flush()
        .map_err(|e| McpError::internal_error(format!("Failed to flush stdout: {}", e)))?;
    Ok(())
}

/// Handle the initialize method.
///
/// Responds to MCP client initialization with server capabilities and information.
///
/// # Arguments
///
/// * `params` - Initialize parameters from the client
/// * `config` - Server configuration
#[instrument(skip(config))]
pub fn handle_initialize(params: InitializeParams, config: &Config) -> McpResult<JsonRpcResponse> {
    debug!(
        protocol_version = %params.protocol_version,
        "Handling initialize request"
    );

    let result = InitializeResult {
        protocol_version: params.protocol_version,
        capabilities: serde_json::json!({
            "tools": {}
        }),
        server_info: ServerInfo {
            name: config.server_name().to_string(),
            version: config.server_version().to_string(),
        },
    };

    // Note: id will be set by caller (handle_method_with_config)
    Ok(JsonRpcResponse {
        jsonrpc: constants::JSON_RPC_VERSION.to_string(),
        id: None, // Will be set by caller from request
        result: Some(serde_json::to_value(result)?),
        error: None,
    })
}

/// Handle a JSON-RPC method call with Arc<Config>.
///
/// Routes method calls to appropriate handlers:
/// - `initialize`: Server initialization
/// - `tools/list`: List all available tools
/// - `tools/call`: Execute a tool
///
/// # Arguments
///
/// * `method` - The method name
/// * `params` - Optional method parameters
/// * `id` - Request ID for response correlation
/// * `registry` - Tool registry for tool operations
/// * `config` - Shared configuration (Arc)
#[instrument(skip(registry, config))]
pub fn handle_method_with_config<T: crate::tools::ToolRegistry>(
    method: &str,
    params: Option<Value>,
    id: Option<Value>,
    registry: &T,
    config: Arc<Config>,
) -> McpResult<JsonRpcResponse> {
    let span = span!(Level::DEBUG, "handle_method", method = method);
    let _enter = span.enter();

    match method {
        constants::methods::INITIALIZE => {
            let init_params: InitializeParams = serde_json::from_value(
                params.ok_or_else(|| McpError::invalid_params("Missing params"))?,
            )?;
            let mut response = handle_initialize(init_params, &config)?;
            // Ensure ID is preserved from request - Claude Desktop requires non-null ID
            response.id = id.clone();
            debug!("Initialize response id: {:?}", response.id);
            Ok(response)
        }
        constants::methods::TOOLS_LIST => {
            debug!("Listing tools, id: {:?}", id);
            let result = serde_json::json!({
                "tools": registry.get_all_tools()
            });
            Ok(JsonRpcResponse {
                jsonrpc: constants::JSON_RPC_VERSION.to_string(),
                id: id.clone(),
                result: Some(result),
                error: None,
            })
        }
        constants::methods::TOOLS_CALL => {
            let call_params: ToolCallParams = serde_json::from_value(
                params.ok_or_else(|| McpError::invalid_params("Missing params"))?,
            )?;

            debug!(
                tool_name = %call_params.name,
                "Executing tool"
            );

            match registry.execute_tool(&call_params.name, &call_params.arguments) {
                Ok(result) => {
                    debug!("Tool execution success, id: {:?}", id);
                    Ok(JsonRpcResponse {
                        jsonrpc: constants::JSON_RPC_VERSION.to_string(),
                        id: id.clone(),
                        result: Some(serde_json::json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": serde_json::to_string(&result)?
                                }
                            ]
                        })),
                        error: None,
                    })
                },
                Err(e) => {
                    error!(
                        tool_name = %call_params.name,
                        error = %e,
                        "Tool execution error"
                    );
                    // MCP requires result to always be present, even for errors
                    // Return error information in the result content
                    debug!("Tool execution error, id: {:?}", id);
                    Ok(JsonRpcResponse {
                        jsonrpc: constants::JSON_RPC_VERSION.to_string(),
                        id: id.clone(),
                        result: Some(serde_json::json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": format!("Error: {}", e.message)
                                }
                            ],
                            "isError": true
                        })),
                        error: None,
                    })
                }
            }
        }
        _ => {
            error!(method = %method, "Method not found");
            debug!("Method not found, id: {:?}", id);
            // MCP requires result to always be present, even for errors
            Ok(JsonRpcResponse {
                jsonrpc: constants::JSON_RPC_VERSION.to_string(),
                id: id.clone(),
                result: Some(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Method not found: {}", method)
                        }
                    ],
                    "isError": true
                })),
                error: None,
            })
        }
    }
}

/// Handle a JSON-RPC method call (legacy, creates config on each call).
///
/// Routes method calls to appropriate handlers:
/// - `initialize`: Server initialization
/// - `tools/list`: List all available tools
/// - `tools/call`: Execute a tool
///
/// # Arguments
///
/// * `method` - The method name
/// * `params` - Optional method parameters
/// * `id` - Request ID for response correlation
/// * `registry` - Tool registry for tool operations
#[instrument(skip(registry))]
pub fn handle_method<T: crate::tools::ToolRegistry>(
    method: &str,
    params: Option<Value>,
    id: Option<Value>,
    registry: &T,
) -> McpResult<JsonRpcResponse> {
    let config = Arc::new(Config::new());
    handle_method_with_config(method, params, id, registry, config)
}

