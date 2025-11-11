use crate::error::{McpError, McpResult};
use crate::tools::registry::DefaultToolRegistry;
use crate::tools::ToolRegistry;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const TOOL_BATCH: &str = "batch_operations";

/// Represents a single operation in a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperation {
    /// Unique identifier for this operation (used to match results)
    pub id: String,
    /// Name of the tool to execute
    pub tool: String,
    /// Arguments for the tool
    pub arguments: Value,
}

/// Result of a single operation in a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// ID matching the original operation
    pub id: String,
    /// Success flag
    pub success: bool,
    /// Result value (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Arguments for batch operations
#[derive(Debug, Deserialize)]
struct BatchArgs {
    operations: Vec<BatchOperation>,
}

/// Get tool definitions for batch operations
pub fn get_tool_definitions() -> Vec<Value> {
    vec![json!({
        "name": TOOL_BATCH,
        "description": "Execute multiple math operations in a single call. Allows the LLM to batch multiple calculations and get all results back together. Each operation has a unique ID to match results. Operations are executed independently - if one fails, others still execute.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "operations": {
                    "type": "array",
                    "description": "Array of operations to execute",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "string",
                                "description": "Unique identifier for this operation (e.g., 'op1', 'step1', 'calc_a')"
                            },
                            "tool": {
                                "type": "string",
                                "description": "Name of the tool to execute (e.g., 'add', 'multiply', 'mean')"
                            },
                            "arguments": {
                                "type": "object",
                                "description": "Arguments to pass to the tool"
                            }
                        },
                        "required": ["id", "tool", "arguments"]
                    }
                }
            },
            "required": ["operations"]
        }
    })]
}

/// Execute batch operations tool
pub fn execute(_tool_name: &str, args: &Value) -> McpResult<Value> {
    let batch_args: BatchArgs = serde_json::from_value(args.clone())
        .map_err(|e| McpError::invalid_params(format!("Invalid batch arguments: {}", e)))?;

    if batch_args.operations.is_empty() {
        return Err(McpError::invalid_params("No operations provided"));
    }

    // Limit batch size to prevent abuse
    const MAX_BATCH_SIZE: usize = 50;
    if batch_args.operations.len() > MAX_BATCH_SIZE {
        return Err(McpError::invalid_params(format!(
            "Batch size exceeds maximum of {} operations",
            MAX_BATCH_SIZE
        )));
    }

    // Check for duplicate IDs
    let mut seen_ids = std::collections::HashSet::new();
    for op in &batch_args.operations {
        if !seen_ids.insert(&op.id) {
            return Err(McpError::invalid_params(format!(
                "Duplicate operation ID: {}",
                op.id
            )));
        }
    }

    let registry = DefaultToolRegistry;
    let mut results = Vec::new();

    // Execute each operation independently
    for operation in batch_args.operations {
        let result = match registry.execute_tool(&operation.tool, &operation.arguments) {
            Ok(value) => BatchOperationResult {
                id: operation.id.clone(),
                success: true,
                result: Some(value),
                error: None,
            },
            Err(e) => BatchOperationResult {
                id: operation.id.clone(),
                success: false,
                result: None,
                error: Some(e.message),
            },
        };
        results.push(result);
    }

    // Count successes and failures
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len() - successful;

    Ok(json!({
        "results": results,
        "summary": {
            "total": results.len(),
            "successful": successful,
            "failed": failed
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_operations_success() {
        let args = json!({
            "operations": [
                {
                    "id": "add1",
                    "tool": "add",
                    "arguments": {"numbers": [1.0, 2.0, 3.0]}
                },
                {
                    "id": "mult1",
                    "tool": "multiply",
                    "arguments": {"numbers": [2.0, 3.0, 4.0]}
                },
                {
                    "id": "mean1",
                    "tool": "mean",
                    "arguments": {"numbers": [10.0, 20.0, 30.0]}
                }
            ]
        });

        let result = execute(TOOL_BATCH, &args).unwrap();

        assert!(result["results"].is_array());
        assert_eq!(result["summary"]["total"], 3);
        assert_eq!(result["summary"]["successful"], 3);
        assert_eq!(result["summary"]["failed"], 0);

        let results = result["results"].as_array().unwrap();

        // Check add result
        let add_result = results.iter().find(|r| r["id"] == "add1").unwrap();
        assert_eq!(add_result["success"], true);
        assert_eq!(add_result["result"]["result"], 6.0);

        // Check multiply result
        let mult_result = results.iter().find(|r| r["id"] == "mult1").unwrap();
        assert_eq!(mult_result["success"], true);
        assert_eq!(mult_result["result"]["result"], 24.0);

        // Check mean result
        let mean_result = results.iter().find(|r| r["id"] == "mean1").unwrap();
        assert_eq!(mean_result["success"], true);
        assert_eq!(mean_result["result"]["result"], 20.0);
    }

    #[test]
    fn test_batch_operations_partial_failure() {
        let args = json!({
            "operations": [
                {
                    "id": "good1",
                    "tool": "add",
                    "arguments": {"numbers": [1.0, 2.0]}
                },
                {
                    "id": "bad1",
                    "tool": "divide",
                    "arguments": {"a": 10.0, "b": 0.0}  // Division by zero
                },
                {
                    "id": "good2",
                    "tool": "multiply",
                    "arguments": {"numbers": [5.0, 6.0]}
                }
            ]
        });

        let result = execute(TOOL_BATCH, &args).unwrap();

        assert_eq!(result["summary"]["total"], 3);
        assert_eq!(result["summary"]["successful"], 2);
        assert_eq!(result["summary"]["failed"], 1);

        let results = result["results"].as_array().unwrap();

        // Check successful operations
        let good1 = results.iter().find(|r| r["id"] == "good1").unwrap();
        assert_eq!(good1["success"], true);

        let good2 = results.iter().find(|r| r["id"] == "good2").unwrap();
        assert_eq!(good2["success"], true);

        // Check failed operation
        let bad1 = results.iter().find(|r| r["id"] == "bad1").unwrap();
        assert_eq!(bad1["success"], false);
        assert!(bad1["error"].is_string());
    }

    #[test]
    fn test_batch_operations_unknown_tool() {
        let args = json!({
            "operations": [
                {
                    "id": "unknown",
                    "tool": "nonexistent_tool",
                    "arguments": {}
                }
            ]
        });

        let result = execute(TOOL_BATCH, &args).unwrap();
        let results = result["results"].as_array().unwrap();
        let unknown = &results[0];

        assert_eq!(unknown["success"], false);
        assert!(unknown["error"].as_str().unwrap().contains("Unknown tool"));
    }

    #[test]
    fn test_batch_operations_empty() {
        let args = json!({
            "operations": []
        });

        let result = execute(TOOL_BATCH, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("No operations"));
    }

    #[test]
    fn test_batch_operations_duplicate_ids() {
        let args = json!({
            "operations": [
                {
                    "id": "dup",
                    "tool": "add",
                    "arguments": {"numbers": [1.0, 2.0]}
                },
                {
                    "id": "dup",
                    "tool": "multiply",
                    "arguments": {"numbers": [3.0, 4.0]}
                }
            ]
        });

        let result = execute(TOOL_BATCH, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Duplicate operation ID"));
    }

    #[test]
    fn test_batch_operations_max_size() {
        let mut operations = Vec::new();
        for i in 0..51 {
            operations.push(json!({
                "id": format!("op{}", i),
                "tool": "add",
                "arguments": {"numbers": [1.0, 2.0]}
            }));
        }

        let args = json!({
            "operations": operations
        });

        let result = execute(TOOL_BATCH, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("exceeds maximum"));
    }

    #[test]
    fn test_batch_operations_complex_scenario() {
        // Simulate a complex calculation scenario
        let args = json!({
            "operations": [
                {
                    "id": "data_sum",
                    "tool": "add",
                    "arguments": {"numbers": [100.0, 200.0, 300.0]}
                },
                {
                    "id": "data_mean",
                    "tool": "mean",
                    "arguments": {"numbers": [100.0, 200.0, 300.0]}
                },
                {
                    "id": "data_median",
                    "tool": "median",
                    "arguments": {"numbers": [100.0, 200.0, 300.0]}
                },
                {
                    "id": "area_calc",
                    "tool": "area_circle",
                    "arguments": {"radius": 5.0}
                },
                {
                    "id": "sqrt_calc",
                    "tool": "sqrt",
                    "arguments": {"number": 144.0}
                }
            ]
        });

        let result = execute(TOOL_BATCH, &args).unwrap();
        assert_eq!(result["summary"]["successful"], 5);
        assert_eq!(result["summary"]["failed"], 0);

        let results = result["results"].as_array().unwrap();
        assert_eq!(results.len(), 5);

        // All should be successful
        for r in results {
            assert_eq!(r["success"], true);
        }
    }
}
