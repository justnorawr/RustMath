// Use the library crate
extern crate rust_math_mcp;

use rust_math_mcp::config::Config;
use rust_math_mcp::error::McpResult;
use rust_math_mcp::protocol::{handle_method_with_config, send_response};
use rust_math_mcp::protocol::parser::parse_message;
use rust_math_mcp::tools::DefaultToolRegistry;
use std::io::{self, BufReader};
use std::sync::Arc;
use tracing::{error, info};

fn main() -> McpResult<()> {
    // Configure tracing to write to stderr to avoid polluting stdout (MCP protocol)
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false) // Disable ANSI codes for compatibility
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust_math_mcp=info".to_string())
        )
        .init();

    info!("Starting Rust Math MCP Server");

    // Create config once at startup
    let config = Arc::new(Config::new());
    info!(
        server_name = %config.server_name(),
        server_version = %config.server_version(),
        "Server configuration loaded"
    );

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    loop {
        match parse_message(&mut reader) {
            Ok(request) => {
                // Log request ID for debugging
                info!("Received request: method={}, id={:?}", request.method, request.id);
                
                let registry = DefaultToolRegistry;
                let response = handle_method_with_config(
                    &request.method,
                    request.params,
                    request.id.clone(),
                    &registry,
                    Arc::clone(&config),
                )?;
                
                // Log response ID for debugging
                info!("Sending response: id={:?}", response.id);
                send_response(response)?;
            }
            Err(e) => {
                error!("Error parsing message: {}", e);
                // Note: We can't get request ID from a failed parse, so id remains None
                // This is acceptable per JSON-RPC spec for parse errors
                let error_response = rust_math_mcp::protocol::JsonRpcResponse {
                    jsonrpc: rust_math_mcp::protocol::constants::JSON_RPC_VERSION.to_string(),
                    id: None,
                    result: None,
                    error: Some(rust_math_mcp::protocol::JsonRpcError::from(e)),
                };
                send_response(error_response)?;
                // Continue processing - don't exit on parse errors
            }
        }
    }
}
