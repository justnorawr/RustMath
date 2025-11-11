use proptest::prelude::*;
use rust_math_mcp::tools::{DefaultToolRegistry, ToolRegistry};
use serde_json::json;

proptest! {
    #[test]
    fn test_add_commutative(a in -1000.0f64..1000.0, b in -1000.0f64..1000.0) {
        let registry = DefaultToolRegistry;
        
        let args1 = json!({ "numbers": [a, b] });
        let args2 = json!({ "numbers": [b, a] });
        
        let result1 = registry.execute_tool("add", &args1).unwrap();
        let result2 = registry.execute_tool("add", &args2).unwrap();
        
        let sum1 = result1["result"].as_f64().unwrap();
        let sum2 = result2["result"].as_f64().unwrap();
        prop_assert_eq!(sum1, sum2);
    }

    #[test]
    fn test_multiply_zero(a in -1000.0f64..1000.0) {
        let registry = DefaultToolRegistry;
        let args = json!({ "numbers": [a, 0.0] });
        
        let result = registry.execute_tool("multiply", &args).unwrap();
        prop_assert_eq!(result["result"].as_f64().unwrap(), 0.0);
    }

    #[test]
    fn test_abs_always_positive(a in -1000.0f64..1000.0) {
        let registry = DefaultToolRegistry;
        let args = json!({ "number": a });
        
        let result = registry.execute_tool("abs", &args).unwrap();
        prop_assert!(result["result"].as_f64().unwrap() >= 0.0);
    }

    #[test]
    fn test_sqrt_squared(a in 0.0f64..100.0) {
        let registry = DefaultToolRegistry;
        let squared = a * a;
        
        let args = json!({ "number": squared });
        let result = registry.execute_tool("sqrt", &args).unwrap();
        
        let sqrt_result = result["result"].as_f64().unwrap();
        prop_assert!((sqrt_result - a).abs() < 1e-10);
    }
}

