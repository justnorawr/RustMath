use rust_math_mcp::tools::{DefaultToolRegistry, ToolRegistry};
use serde_json::json;

#[test]
fn test_tool_registry_list_tools() {
    let registry = DefaultToolRegistry;
    let tools = registry.get_all_tools();
    
    assert!(tools.is_array());
    let tools_array = tools.as_array().unwrap();
    assert!(!tools_array.is_empty());
    
    // Check that some expected tools exist
    let tool_names: Vec<&str> = tools_array
        .iter()
        .filter_map(|t| t.get("name").and_then(|n| n.as_str()))
        .collect();
    
    assert!(tool_names.contains(&"add"));
    assert!(tool_names.contains(&"mean"));
    assert!(tool_names.contains(&"quadratic_formula"));
}

#[test]
fn test_tool_execution_add() {
    let registry = DefaultToolRegistry;
    let args = json!({ "numbers": [1.0, 2.0, 3.0] });
    
    let result = registry.execute_tool("add", &args).unwrap();
    assert_eq!(result["result"], 6.0);
}

#[test]
fn test_tool_execution_unknown() {
    let registry = DefaultToolRegistry;
    let args = json!({});
    
    let result = registry.execute_tool("unknown_tool", &args);
    assert!(result.is_err());
}

#[test]
fn test_tool_execution_divide_by_zero() {
    let registry = DefaultToolRegistry;
    let args = json!({ "a": 10.0, "b": 0.0 });
    
    let result = registry.execute_tool("divide", &args);
    assert!(result.is_err());
}

