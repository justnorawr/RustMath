use crate::error::{McpError, McpResult};
use crate::protocol::JsonRpcRequest;
use std::io::BufRead;
use tracing::debug;

/// Maximum allowed Content-Length to prevent memory exhaustion attacks.
/// Set to 10MB - enough for large tool calls but prevents DoS.
const MAX_CONTENT_LENGTH: usize = 10_000_000;

/// Parse MCP protocol message from a buffered reader.
///
/// Supports two formats:
/// 1. MCP stdio format with Content-Length header:
///    - Line 1: "Content-Length: <number>"
///    - Line 2: Empty line
///    - Line 3+: JSON message of <number> bytes
/// 2. Raw JSON format (Claude Desktop):
///    - Direct JSON object (may span multiple lines)
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
/// - Message format is unrecognized
/// - Content-Length header is invalid (for format 1)
/// - JSON message cannot be parsed
/// - JSON-RPC version is invalid
pub fn parse_message<R: BufRead>(reader: &mut R) -> McpResult<JsonRpcRequest> {
    // Try to read the first line to determine the format
    let mut first_line = String::new();
    let bytes_read = reader.read_line(&mut first_line)
        .map_err(|e| McpError::internal_error(format!("Failed to read input: {}", e)))?;

    // Handle EOF gracefully (clean shutdown)
    if bytes_read == 0 {
        return Err(McpError::new(-32001, "EOF: clean shutdown"));
    }

    let trimmed = first_line.trim();
    
    // Check if it's a Content-Length header (MCP stdio format)
    if trimmed.starts_with("Content-Length:") {
        // Parse Content-Length header format
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
        request.validate()?;
        Ok(request)
    } else if trimmed.starts_with('{') {
        // It's raw JSON (Claude Desktop format) - read the entire JSON object
        // We need to read until we have a complete JSON object
        let mut json_buffer = first_line;
        
        // Try to parse what we have so far
        // If it's incomplete, we need to read more
        loop {
            match serde_json::from_str::<JsonRpcRequest>(&json_buffer.trim()) {
                Ok(request) => {
                    request.validate()?;
                    return Ok(request);
                }
                Err(e) if e.is_eof() || e.is_data() => {
                    // Need more data - read another line
                    let mut next_line = String::new();
                    let bytes = reader.read_line(&mut next_line)
                        .map_err(|e| McpError::internal_error(format!("Failed to read JSON: {}", e)))?;
                    if bytes == 0 {
                        return Err(McpError::parse_error("Incomplete JSON message"));
                    }
                    json_buffer.push_str(&next_line);
                }
                Err(e) => {
                    return Err(McpError::parse_error(format!("JSON parse error: {}", e)));
                }
            }
        }
    } else {
        // Unknown format
        debug!("Unknown input format, first line: {:?}", trimmed);
        return Err(McpError::invalid_request(format!(
            "Unknown message format, expected Content-Length header or JSON, got: {}",
            if trimmed.len() > 50 { format!("{}...", &trimmed[..50]) } else { trimmed.to_string() }
        )));
    }
}

