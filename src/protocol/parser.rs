use crate::error::{McpError, McpResult};
use crate::protocol::JsonRpcRequest;
use std::io::BufRead;
use tracing::debug;

/// Maximum allowed Content-Length to prevent memory exhaustion attacks.
/// Set to 10MB - enough for large tool calls but prevents DoS.
const MAX_CONTENT_LENGTH: usize = 10_000_000;

/// Parse MCP protocol message from a buffered reader.
///
/// MCP uses a simple protocol:
/// - Line 1: "Content-Length: <number>"
/// - Line 2: Empty line
/// - Line 3+: JSON message of <number> bytes
///
/// # Arguments
///
/// * `reader` - Buffered reader (typically stdin)
///
/// # Returns
///
/// A parsed and validated JSON-RPC request, or an error if parsing fails.
///
/// # Errors
///
/// Returns an error if:
/// - Content-Length header is missing or invalid
/// - JSON message cannot be parsed
/// - JSON-RPC version is invalid
pub fn parse_message<R: BufRead>(reader: &mut R) -> McpResult<JsonRpcRequest> {
    // Read Content-Length header
    let mut content_length_line = String::new();
    let bytes_read = reader.read_line(&mut content_length_line)
        .map_err(|e| McpError::internal_error(format!("Failed to read Content-Length: {}", e)))?;

    // Handle EOF gracefully (clean shutdown)
    if bytes_read == 0 {
        return Err(McpError::new(-32001, "EOF: clean shutdown"));
    }

    if content_length_line.trim().is_empty() {
        return Err(McpError::invalid_request("Empty input"));
    }

    // Parse Content-Length
    // Trim the line to handle any trailing whitespace/newlines
    let trimmed = content_length_line.trim();
    if !trimmed.starts_with("Content-Length:") {
        // Log what we actually received for debugging
        debug!("Expected Content-Length header, got: {:?}", trimmed);
        return Err(McpError::invalid_request(format!(
            "Expected Content-Length header, got: {}",
            if trimmed.len() > 50 { format!("{}...", &trimmed[..50]) } else { trimmed.to_string() }
        )));
    }

    // Use trimmed line for parsing to avoid issues with trailing whitespace
    let length: usize = trimmed
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| McpError::invalid_request("Invalid Content-Length header format"))?
        .parse()
        .map_err(|e| McpError::invalid_request(format!("Invalid Content-Length value: {}", e)))?;

    // Prevent memory exhaustion attacks
    if length > MAX_CONTENT_LENGTH {
        return Err(McpError::resource_limit(format!(
            "Content-Length {} exceeds maximum allowed size of {} bytes",
            length, MAX_CONTENT_LENGTH
        )));
    }

    // Read blank line after header
    let mut blank_line = String::new();
    reader.read_line(&mut blank_line)
        .map_err(|e| McpError::internal_error(format!("Failed to read blank line: {}", e)))?;

    // Read the actual JSON message
    let mut json_buffer = vec![0u8; length];
    reader.read_exact(&mut json_buffer)
        .map_err(|e| McpError::internal_error(format!("Failed to read JSON message: {}", e)))?;
    
    let json_str = String::from_utf8(json_buffer)
        .map_err(|e| McpError::parse_error(format!("Invalid UTF-8 in message: {}", e)))?;

    // Parse JSON-RPC request
    let request: JsonRpcRequest = serde_json::from_str(&json_str)?;
    
    // Validate the request
    request.validate()?;
    
    Ok(request)
}

