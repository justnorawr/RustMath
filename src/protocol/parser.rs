use crate::error::{McpError, McpResult};
use crate::protocol::JsonRpcRequest;
use std::io::BufRead;
use tracing::debug;

/// Maximum allowed Content-Length to prevent memory exhaustion attacks.
/// Set to 10MB - enough for large tool calls but prevents DoS.
const MAX_CONTENT_LENGTH: usize = 10_000_000;

/// Parse result containing both the request and the format used
#[derive(Debug)]
pub struct ParseResult {
    pub request: JsonRpcRequest,
    pub uses_content_length: bool,
}

/// Parse MCP protocol message from a buffered reader.
///
/// Supports two formats:
/// 1. MCP stdio format with Content-Length header:
///    - Line 1: "Content-Length: <number>"
///    - Line 2: Empty line
///    - Line 3+: JSON message of <number> bytes
/// 2. Raw JSON format (Claude Desktop):
///    - Direct JSON object (may span multiple lines, may or may not have trailing newline)
///
/// # Arguments
///
/// * `reader` - Buffered reader (typically stdin)
///
/// # Returns
///
/// A parsed result containing the request and whether Content-Length format was used
///
/// # Errors
///
/// Returns an error if:
/// - Message format is unrecognized
/// - Content-Length header is invalid (for format 1)
/// - JSON message cannot be parsed
/// - JSON-RPC version is invalid
pub fn parse_message<R: BufRead>(reader: &mut R) -> McpResult<ParseResult> {
    // Try to peek at the first bytes to determine the format
    // This avoids blocking on read_line if there's no newline
    let buffer = reader.fill_buf()
        .map_err(|e| McpError::internal_error(format!("Failed to read input: {}", e)))?;
    
    // Handle EOF gracefully (clean shutdown)
    if buffer.is_empty() {
        return Err(McpError::new(-32001, "EOF: clean shutdown"));
    }
    
    // Check if it starts with '{' (raw JSON) or "Content-Length:" (MCP stdio format)
    let starts_with_json = buffer.first().map(|&b| b == b'{').unwrap_or(false);
    let starts_with_header = buffer.starts_with(b"Content-Length:");
    
    if starts_with_header {
        // MCP stdio format with Content-Length header
        let mut first_line = String::new();
        reader.read_line(&mut first_line)
            .map_err(|e| McpError::internal_error(format!("Failed to read header: {}", e)))?;
        
        let trimmed = first_line.trim();
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
        Ok(ParseResult {
            request,
            uses_content_length: true,
        })
    } else if starts_with_json {
        // It's raw JSON (Claude Desktop format) - read the entire JSON object
        // Read all available data and try to parse it
        // We'll read in chunks until we have a complete JSON object
        let mut json_buffer = Vec::new();
        
        // Read all available data first
        loop {
            let buffer = reader.fill_buf()
                .map_err(|e| McpError::internal_error(format!("Failed to read JSON: {}", e)))?;
            
            if buffer.is_empty() {
                break;
            }
            
            let consumed = buffer.len();
            json_buffer.extend_from_slice(buffer);
            reader.consume(consumed);
            
            // Try to parse what we have so far
            match serde_json::from_slice::<JsonRpcRequest>(&json_buffer) {
                Ok(request) => {
                    request.validate()?;
                    return Ok(ParseResult {
                        request,
                        uses_content_length: false,
                    });
                }
                Err(e) if e.is_eof() || e.is_data() => {
                    // Need more data - continue reading
                    continue;
                }
                Err(e) => {
                    // Try parsing as string in case there's trailing whitespace
                    match String::from_utf8(json_buffer.clone())
                        .ok()
                        .and_then(|s| serde_json::from_str::<JsonRpcRequest>(s.trim()).ok())
                    {
                        Some(request) => {
                            request.validate()?;
                            return Ok(ParseResult {
                                request,
                                uses_content_length: false,
                            });
                        }
                        None => {
                            return Err(McpError::parse_error(format!("JSON parse error: {}", e)));
                        }
                    }
                }
            }
        }
        
        // If we get here, we didn't get a complete JSON object in the buffer
        // Try parsing what we have as a string (might have trailing newline/whitespace)
        let json_str = String::from_utf8(json_buffer)
            .map_err(|e| McpError::parse_error(format!("Invalid UTF-8 in message: {}", e)))?;
        let request: JsonRpcRequest = serde_json::from_str(json_str.trim())?;
        request.validate()?;
        Ok(ParseResult {
            request,
            uses_content_length: false,
        })
    } else {
        // Unknown format - try to read a line to see what we got
        let mut first_line = String::new();
        reader.read_line(&mut first_line)
            .map_err(|e| McpError::internal_error(format!("Failed to read input: {}", e)))?;
        let trimmed = first_line.trim();
        debug!("Unknown input format, first line: {:?}", trimmed);
        return Err(McpError::invalid_request(format!(
            "Unknown message format, expected Content-Length header or JSON, got: {}",
            if trimmed.len() > 50 { format!("{}...", &trimmed[..50]) } else { trimmed.to_string() }
        )));
    }
}
