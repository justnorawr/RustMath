// Use the library crate
extern crate rust_math_mcp;

use rust_math_mcp::config::Config;
use rust_math_mcp::error::McpResult;
use rust_math_mcp::protocol::parser::parse_message;
use rust_math_mcp::protocol::{handle_method_with_config, send_response};
use rust_math_mcp::tools::DefaultToolRegistry;
use std::io::{self, BufReader};
use std::sync::Arc;
use tracing::{debug, error};

fn main() -> McpResult<()> {
    // Configure tracing to write to stderr to avoid polluting stdout (MCP protocol)
    // MCP uses stdout for protocol communication, so ALL output must go to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr) // Critical: stderr only, never stdout
        .with_ansi(false) // Disable ANSI codes for compatibility
        .with_target(false) // Disable target prefix to reduce noise
        .with_thread_ids(false) // Disable thread IDs to reduce noise
        .with_thread_names(false) // Disable thread names to reduce noise
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_math_mcp=warn".to_string()), // Default to warn to reduce noise
        )
        .init();

    // Log startup to stderr only (tracing is configured to use stderr)
    debug!("Starting Rust Math MCP Server");

    // Create config once at startup
    let config = Arc::new(Config::new());
    debug!(
        server_name = %config.server_name(),
        server_version = %config.server_version(),
        "Server configuration loaded"
    );

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    loop {
        match parse_message(&mut reader) {
            Ok(parse_result) => {
                // Log to stderr only (tracing is configured to use stderr)
                debug!(
                    "Received request: method={}, id={:?}, format={}",
                    parse_result.request.method,
                    parse_result.request.id,
                    if parse_result.uses_content_length {
                        "Content-Length"
                    } else {
                        "raw JSON"
                    }
                );

                let registry = DefaultToolRegistry;
                let response = handle_method_with_config(
                    &parse_result.request.method,
                    parse_result.request.params,
                    parse_result.request.id.clone(),
                    &registry,
                    Arc::clone(&config),
                )?;

                // Use the same format as the request (match request format)
                send_response(response, parse_result.uses_content_length)?;
            }
            Err(e) => {
                // Handle EOF gracefully - this is a clean shutdown, not an error
                // Check error code and message to detect EOF
                let error_code = e.code;
                let error_msg = e.message.clone();
                if error_code == -32001 && error_msg.contains("EOF") {
                    debug!("Received EOF, shutting down gracefully");
                    return Ok(()); // Exit cleanly
                }

                // For parse errors, we cannot send a valid response because:
                // 1. JSON-RPC 2.0 spec allows null ID for parse errors
                // 2. Claude Desktop's validation REJECTS responses with null ID
                // 3. We don't have a valid request ID from a failed parse
                // Solution: Log the error and continue waiting for next valid message
                error!("Parse error (cannot respond to client): {}", e);
                // Continue loop to process next message
            }
        }
    }
}
