use rust_math_mcp::protocol::JsonRpcRequest;
use serde_json::{json, Value};

#[test]
fn test_response_id_serialization() {
    // Test that response with id=0 serializes correctly (not as null)
    let response = rust_math_mcp::protocol::JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(Value::Number(0.into())),
        result: Some(json!({"test": "value"})),
        error: None,
    };
    
    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    
    // Verify id is a number, not null
    assert!(parsed["id"].is_number());
    assert_eq!(parsed["id"].as_i64(), Some(0));
    assert!(!parsed["id"].is_null());
}

#[test]
fn test_response_id_string_serialization() {
    // Test that response with string id serializes correctly
    let response = rust_math_mcp::protocol::JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(Value::String("test-id".to_string())),
        result: Some(json!({"test": "value"})),
        error: None,
    };
    
    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    
    // Verify id is a string, not null
    assert!(parsed["id"].is_string());
    assert_eq!(parsed["id"].as_str(), Some("test-id"));
    assert!(!parsed["id"].is_null());
}

#[test]
fn test_response_with_result_content() {
    // Test MCP tools/call response format
    let response = rust_math_mcp::protocol::JsonRpcResponse {
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
    
    assert_eq!(parsed["id"].as_i64(), Some(1));
    assert!(parsed["result"].is_object());
    assert!(parsed["result"]["content"].is_array());
    assert_eq!(parsed["result"]["content"][0]["type"], "text");
}

#[test]
fn test_initialize_request_parsing() {
    // Test parsing an initialize request like Claude Desktop sends
    let request_json = r#"{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"claude-ai","version":"0.1.0"}},"id":0}"#;
    
    let request: JsonRpcRequest = serde_json::from_str(request_json).unwrap();
    
    assert_eq!(request.method, "initialize");
    assert!(request.id.is_some());
    assert_eq!(request.id.as_ref().unwrap().as_i64(), Some(0));
}

#[test]
fn test_response_matches_request_id() {
    // Test that response ID matches request ID
    let request_json = r#"{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"claude-ai","version":"0.1.0"}},"id":0}"#;
    
    let request: JsonRpcRequest = serde_json::from_str(request_json).unwrap();
    let request_id = request.id.clone();
    
    // Simulate creating a response with the request ID
    let response = rust_math_mcp::protocol::JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request_id,
        result: Some(json!({"protocolVersion": "2025-06-18"})),
        error: None,
    };
    
    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    
    // Verify the ID is preserved and is a number (not null)
    assert!(parsed["id"].is_number());
    assert_eq!(parsed["id"].as_i64(), Some(0));
}

#[test]
fn test_error_response_format() {
    // Test error response format (using result instead of error field)
    let response = rust_math_mcp::protocol::JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(Value::Number(1.into())),
        result: Some(json!({
            "content": [
                {
                    "type": "text",
                    "text": "Error: Test error message"
                }
            ],
            "isError": true
        })),
        error: None,
    };
    
    let json_str = serde_json::to_string(&response).unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    
    // Verify error field is not present
    assert!(parsed.get("error").is_none());
    // Verify result is present
    assert!(parsed["result"].is_object());
    assert_eq!(parsed["result"]["isError"], true);
}

