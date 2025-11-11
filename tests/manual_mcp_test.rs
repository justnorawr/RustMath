// Manual test to verify MCP response format matches Claude Desktop expectations
use rust_math_mcp::protocol::JsonRpcResponse;
use serde_json::{json, Value};

#[test]
fn test_claude_desktop_response_format() {
    // Simulate an initialize response that Claude Desktop expects
    let response = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(Value::Number(0.into())), // Must be non-null
        result: Some(json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "rust-math-mcp",
                "version": "0.1.0"
            }
        })),
        error: None, // Should not be present
    };

    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();

    // Claude Desktop requirements:
    // 1. id must be present and non-null (string or number)
    assert!(parsed["id"].is_number() || parsed["id"].is_string());
    assert!(!parsed["id"].is_null());

    // 2. result must be present
    assert!(parsed["result"].is_object());

    // 3. error should not be present (or should be null/undefined)
    assert!(parsed.get("error").is_none() || parsed["error"].is_null());

    println!("Response format: {}", json_str);
}

#[test]
fn test_tools_call_response_format() {
    // Simulate a tools/call response
    let response = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(Value::Number(1.into())),
        result: Some(json!({
            "content": [
                {
                    "type": "text",
                    "text": "{\"result\":42}"
                }
            ]
        })),
        error: None,
    };

    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();

    // Verify format
    assert!(parsed["id"].is_number());
    assert!(parsed["result"].is_object());
    assert!(parsed["result"]["content"].is_array());
    assert_eq!(parsed["result"]["content"][0]["type"], "text");
    assert!(parsed.get("error").is_none());

    println!("Tools call response format: {}", json_str);
}

#[test]
fn test_error_response_format() {
    // Simulate an error response (using result, not error field)
    let response = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(Value::Number(2.into())),
        result: Some(json!({
            "content": [
                {
                    "type": "text",
                    "text": "Error: Division by zero"
                }
            ],
            "isError": true
        })),
        error: None, // Must not be present
    };

    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();

    // Verify error format
    assert!(parsed["id"].is_number());
    assert!(parsed["result"].is_object());
    assert!(parsed.get("error").is_none()); // Critical: error field must not exist
    assert_eq!(parsed["result"]["isError"], true);

    println!("Error response format: {}", json_str);
}
