// Use the library crate
extern crate rust_math_mcp;

use rust_math_mcp::config::Config;
use rust_math_mcp::error::McpResult;
use rust_math_mcp::protocol::{handle_method_with_config, send_response};
use rust_math_mcp::protocol::parser::parse_message;
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
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust_math_mcp=warn".to_string()) // Default to warn to reduce noise
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
            Ok(request) => {
                // Log to stderr only (tracing is configured to use stderr)
                debug!("Received request: method={}, id={:?}", request.method, request.id);
                
                let registry = DefaultToolRegistry;
                let response = handle_method_with_config(
                    &request.method,
                    request.params,
                    request.id.clone(),
                    &registry,
                    Arc::clone(&config),
                )?;
                
                // Log to stderr only
                debug!("Sending response: id={:?}", response.id);
                send_response(response)?;
            }
            Err(e) => {
                error!("Error parsing message: {}", e);
                // For parse errors, JSON-RPC 2.0 spec says we can send a response with null ID
                // However, if the parse completely fails, we might not be able to send a proper response
                // Try to send an error response, but if it fails, just log and continue
                // Claude Desktop might not accept responses with null ID, so we'll try anyway
                match send_response(rust_math_mcp::protocol::JsonRpcResponse {
                    jsonrpc: rust_math_mcp::protocol::constants::JSON_RPC_VERSION.to_string(),
                    id: None, // Parse errors can have null ID per JSON-RPC 2.0
                    result: Some(serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": format!("Parse error: {}", e.message)
                            }
                        ],
                        "isError": true
                    })),
                    error: None, // Don't use error field - Claude Desktop doesn't recognize it
                }) {
                    Ok(_) => {
                        // Response sent successfully
                    }
                    Err(send_err) => {
                        error!("Failed to send error response: {}", send_err);
                        // Don't exit - continue processing
                    }
                }
            }
        }
    }
}
