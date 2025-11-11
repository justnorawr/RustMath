// Integration tests that simulate Claude Desktop's actual usage of the MCP server
// These tests spawn the binary as a subprocess and communicate via stdin/stdout
// exactly as Claude Desktop does.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use serde_json::{json, Value};

/// Helper struct to manage communication with the MCP server process
struct McpServerProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl McpServerProcess {
    /// Spawn the MCP server binary
    fn spawn() -> Self {
        // Build path to the binary
        let binary_path = std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Failed to get parent directory")
            .parent()
            .expect("Failed to get grandparent directory")
            .join("rust-math-mcp");

        let mut child = Command::new(&binary_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn MCP server process");

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = BufReader::new(child.stdout.take().expect("Failed to open stdout"));

        McpServerProcess {
            child,
            stdin,
            stdout,
        }
    }

    /// Send a JSON-RPC request to the server (raw JSON format like Claude Desktop)
    fn send_request(&mut self, method: &str, params: Value, id: i64) {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });

        let request_str = serde_json::to_string(&request).expect("Failed to serialize request");

        // Claude Desktop sends raw JSON followed by newline
        writeln!(self.stdin, "{}", request_str).expect("Failed to write request");
        self.stdin.flush().expect("Failed to flush stdin");
    }

    /// Read a JSON-RPC response from the server
    /// Expects raw JSON format (newline-delimited)
    fn read_response(&mut self) -> Value {
        let mut response_line = String::new();

        // Read until we get a complete JSON line
        self.stdout
            .read_line(&mut response_line)
            .expect("Failed to read response");

        let trimmed = response_line.trim();
        serde_json::from_str(trimmed).expect(&format!("Failed to parse response: {}", trimmed))
    }


    /// Terminate the server process
    fn terminate(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for McpServerProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

#[test]
fn test_initialize_handshake() {
    let mut server = McpServerProcess::spawn();

    // Send initialize request (exactly as Claude Desktop does)
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {
                "name": "claude-ai",
                "version": "0.1.0"
            }
        }),
        0,
    );

    // Read initialize response
    let response = server.read_response();

    // Verify response format
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 0);
    assert!(response["result"].is_object(), "result should be an object");
    assert!(response.get("error").is_none(), "error field should not be present");

    // Verify response content
    let result = &response["result"];
    assert_eq!(result["protocolVersion"], "2025-06-18");
    assert!(result["capabilities"].is_object());
    assert!(result["capabilities"]["tools"].is_object());
    assert_eq!(result["serverInfo"]["name"], "rust-math-mcp");
    assert_eq!(result["serverInfo"]["version"], "0.1.0");

    server.terminate();
}

#[test]
fn test_tools_list() {
    let mut server = McpServerProcess::spawn();

    // Initialize first
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Now request tools list
    server.send_request("tools/list", json!({}), 1);

    let response = server.read_response();

    // Verify response format
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"].is_object());
    assert!(response.get("error").is_none());

    // Verify tools are returned
    let tools = &response["result"]["tools"];
    assert!(tools.is_array(), "tools should be an array");
    let tools_array = tools.as_array().unwrap();
    assert!(!tools_array.is_empty(), "should have at least one tool");

    // Verify at least some expected tools exist
    let tool_names: Vec<&str> = tools_array
        .iter()
        .filter_map(|t| t.get("name").and_then(|n| n.as_str()))
        .collect();

    assert!(tool_names.contains(&"add"), "should have add tool");
    assert!(tool_names.contains(&"multiply"), "should have multiply tool");
    assert!(tool_names.contains(&"mean"), "should have mean tool");

    server.terminate();
}

#[test]
fn test_tools_call_add() {
    let mut server = McpServerProcess::spawn();

    // Initialize
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Call the add tool
    server.send_request(
        "tools/call",
        json!({
            "name": "add",
            "arguments": {
                "numbers": [10.0, 20.0, 30.0]
            }
        }),
        2,
    );

    let response = server.read_response();

    // Verify response format
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 2);
    assert!(response["result"].is_object());
    assert!(response.get("error").is_none());

    // Verify result content
    let content = &response["result"]["content"];
    assert!(content.is_array());
    assert_eq!(content[0]["type"], "text");

    // Parse the text content (it's JSON stringified)
    let text = content[0]["text"].as_str().unwrap();
    let result_data: Value = serde_json::from_str(text).unwrap();
    assert_eq!(result_data["result"], 60.0);

    server.terminate();
}

#[test]
fn test_tools_call_quadratic_formula() {
    let mut server = McpServerProcess::spawn();

    // Initialize
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Call quadratic_formula tool (ax^2 + bx + c = 0)
    // Example: x^2 - 5x + 6 = 0 (roots are 2 and 3)
    server.send_request(
        "tools/call",
        json!({
            "name": "quadratic_formula",
            "arguments": {
                "a": 1.0,
                "b": -5.0,
                "c": 6.0
            }
        }),
        3,
    );

    let response = server.read_response();

    // Verify response
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 3);
    assert!(response["result"].is_object());

    let content = &response["result"]["content"];
    let text = content[0]["text"].as_str().unwrap();
    let result_data: Value = serde_json::from_str(text).unwrap();

    // Check that we got two roots
    assert!(result_data["roots"].is_array());
    let roots = result_data["roots"].as_array().unwrap();
    assert_eq!(roots.len(), 2);

    server.terminate();
}

#[test]
fn test_tools_call_error_handling() {
    let mut server = McpServerProcess::spawn();

    // Initialize
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Call divide with zero (should error)
    server.send_request(
        "tools/call",
        json!({
            "name": "divide",
            "arguments": {
                "a": 10.0,
                "b": 0.0
            }
        }),
        4,
    );

    let response = server.read_response();

    // Verify response format
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 4);
    assert!(response["result"].is_object());
    assert!(response.get("error").is_none(), "should use result, not error field");

    // Verify error is in result content
    let result = &response["result"];
    assert_eq!(result["isError"], true);

    let content = &result["content"];
    assert!(content.is_array());
    let text = content[0]["text"].as_str().unwrap();
    assert!(text.contains("Error"), "error message should contain 'Error'");

    server.terminate();
}

#[test]
fn test_unknown_tool() {
    let mut server = McpServerProcess::spawn();

    // Initialize
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Call non-existent tool
    server.send_request(
        "tools/call",
        json!({
            "name": "nonexistent_tool",
            "arguments": {}
        }),
        5,
    );

    let response = server.read_response();

    // Should return error in result
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 5);
    assert!(response["result"].is_object());
    assert_eq!(response["result"]["isError"], true);

    server.terminate();
}

#[test]
fn test_unknown_method() {
    let mut server = McpServerProcess::spawn();

    // Initialize
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Send unknown method
    server.send_request("unknown/method", json!({}), 6);

    let response = server.read_response();

    // Should return error in result
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 6);
    assert!(response["result"].is_object());
    assert_eq!(response["result"]["isError"], true);

    let content = &response["result"]["content"];
    let text = content[0]["text"].as_str().unwrap();
    assert!(text.contains("Method not found"));

    server.terminate();
}

#[test]
fn test_multiple_sequential_calls() {
    let mut server = McpServerProcess::spawn();

    // Initialize
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Make multiple tool calls in sequence
    for i in 1..=5 {
        server.send_request(
            "tools/call",
            json!({
                "name": "add",
                "arguments": {
                    "numbers": [i as f64, i as f64]
                }
            }),
            i,
        );

        let response = server.read_response();
        assert_eq!(response["id"], i);
        assert!(response["result"].is_object());

        let content = &response["result"]["content"];
        let text = content[0]["text"].as_str().unwrap();
        let result_data: Value = serde_json::from_str(text).unwrap();
        assert_eq!(result_data["result"], (i * 2) as f64);
    }

    server.terminate();
}

#[test]
fn test_statistics_tools() {
    let mut server = McpServerProcess::spawn();

    // Initialize
    server.send_request(
        "initialize",
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }),
        0,
    );
    let _ = server.read_response();

    // Test mean
    server.send_request(
        "tools/call",
        json!({
            "name": "mean",
            "arguments": {
                "numbers": [10.0, 20.0, 30.0]
            }
        }),
        1,
    );

    let response = server.read_response();
    assert_eq!(response["id"], 1);
    let content = &response["result"]["content"];
    let text = content[0]["text"].as_str().unwrap();
    let result_data: Value = serde_json::from_str(text).unwrap();
    assert_eq!(result_data["result"], 20.0);

    // Test median
    server.send_request(
        "tools/call",
        json!({
            "name": "median",
            "arguments": {
                "numbers": [1.0, 2.0, 3.0, 4.0, 5.0]
            }
        }),
        2,
    );

    let response = server.read_response();
    assert_eq!(response["id"], 2);
    let content = &response["result"]["content"];
    let text = content[0]["text"].as_str().unwrap();
    let result_data: Value = serde_json::from_str(text).unwrap();
    assert_eq!(result_data["result"], 3.0);

    server.terminate();
}
