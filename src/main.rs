use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing::info;

mod service;
use service::MathService;

#[tokio::main]
async fn main() -> Result<()> {
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
                .unwrap_or_else(|_| "rust_math_mcp=warn".to_string()), // Default to warn
        )
        .init();

    info!("Starting Rust Math MCP Server");

    // Create and serve the MathService via stdio transport
    let service = MathService::new()
        .serve(stdio())
        .await
        .inspect_err(|e| tracing::error!("Server error: {:?}", e))?;

    // Wait for the server to complete
    service.waiting().await?;

    Ok(())
}
