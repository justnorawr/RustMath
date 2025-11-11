use crate::error::{McpError, McpResult};
use serde_json::Value;

/// Extract a required number argument from JSON.
///
/// Validates that the value is a finite number (not NaN or Infinity).
///
/// # Arguments
///
/// * `arguments` - JSON object containing the arguments
/// * `key` - Key to extract from the arguments object
///
/// # Returns
///
/// The extracted number, or an error if the key is missing, not a number, or not finite.
///
/// # Example
///
/// ```rust
/// use rust_math_mcp::utils::args::get_number;
/// use serde_json::json;
///
/// let args = json!({ "radius": 5.0 });
/// let radius = get_number(&args, "radius").unwrap(); // 5.0
/// ```
pub fn get_number(arguments: &Value, key: &str) -> McpResult<f64> {
    let value = arguments[key]
        .as_f64()
        .ok_or_else(|| McpError::invalid_params(format!("Invalid argument: {} must be a number", key)))?;
    
    // Validate the number is finite
    if !value.is_finite() {
        return Err(McpError::validation_error(format!(
            "Invalid argument: {} must be a finite number",
            key
        )));
    }
    
    Ok(value)
}

/// Extract an optional number argument from JSON.
///
/// Returns `None` if the key is missing or the value is not a finite number.
///
/// # Arguments
///
/// * `arguments` - JSON object containing the arguments
/// * `key` - Key to extract from the arguments object
///
/// # Returns
///
/// `Some(number)` if the key exists and is a finite number, `None` otherwise.
pub fn get_number_opt(arguments: &Value, key: &str) -> Option<f64> {
    arguments[key].as_f64().filter(|v| v.is_finite())
}

/// Extract a required array of numbers from JSON with validation.
///
/// Validates:
/// - The value is an array
/// - Array size is within configured limits
/// - All elements are finite numbers
///
/// # Arguments
///
/// * `arguments` - JSON object containing the arguments
/// * `key` - Key to extract from the arguments object
///
/// # Returns
///
/// A vector of finite numbers, or an error if validation fails.
///
/// # Example
///
/// ```rust
/// use rust_math_mcp::utils::args::get_number_array;
/// use serde_json::json;
///
/// let args = json!({ "numbers": [1.0, 2.0, 3.0] });
/// let numbers = get_number_array(&args, "numbers").unwrap(); // vec![1.0, 2.0, 3.0]
/// ```
pub fn get_number_array(arguments: &Value, key: &str) -> McpResult<Vec<f64>> {
    use crate::config::Config;
    use crate::utils::validation::validate_array_size;
    
    let arr = arguments[key]
        .as_array()
        .ok_or_else(|| McpError::invalid_params(format!("Invalid arguments: {} must be an array", key)))?;
    
    // Validate array size
    let config = Config::new();
    validate_array_size(arr.len(), &config)?;
    
    let numbers: Vec<f64> = arr
        .iter()
        .filter_map(|v| v.as_f64())
        .collect();
    
    if numbers.len() != arr.len() {
        return Err(McpError::invalid_params(format!("Invalid arguments: {} must be an array of numbers", key)));
    }
    
    // Validate all numbers are finite
    for (idx, num) in numbers.iter().enumerate() {
        if !num.is_finite() {
            return Err(McpError::validation_error(format!(
                "Invalid argument: {}[{}] must be a finite number",
                key, idx
            )));
        }
    }
    
    Ok(numbers)
}

/// Extract an optional boolean argument from JSON
pub fn get_bool_opt(arguments: &Value, key: &str) -> Option<bool> {
    arguments[key].as_bool()
}

/// Helper to create a result JSON response
pub fn result_json(value: f64) -> Value {
    serde_json::json!({ "result": value })
}

/// Helper to create a result JSON response from a Value
pub fn result_value(value: Value) -> Value {
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_number() {
        let args = json!({ "a": 42.0 });
        assert_eq!(get_number(&args, "a").unwrap(), 42.0);
    }

    #[test]
    fn test_get_number_missing() {
        let args = json!({});
        assert!(get_number(&args, "a").is_err());
    }

    #[test]
    fn test_get_number_invalid() {
        let args = json!({ "a": "not a number" });
        assert!(get_number(&args, "a").is_err());
    }

    #[test]
    fn test_get_number_array() {
        let args = json!({ "numbers": [1.0, 2.0, 3.0] });
        let result = get_number_array(&args, "numbers").unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_get_number_array_empty() {
        let args = json!({ "numbers": [] });
        let result = get_number_array(&args, "numbers").unwrap();
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_get_number_array_invalid() {
        let args = json!({ "numbers": [1.0, "not a number", 3.0] });
        assert!(get_number_array(&args, "numbers").is_err());
    }

    #[test]
    fn test_get_number_opt() {
        let args = json!({ "a": 42.0 });
        assert_eq!(get_number_opt(&args, "a"), Some(42.0));
    }

    #[test]
    fn test_get_number_opt_missing() {
        let args = json!({});
        assert_eq!(get_number_opt(&args, "a"), None);
    }

    #[test]
    fn test_get_bool_opt() {
        let args = json!({ "flag": true });
        assert_eq!(get_bool_opt(&args, "flag"), Some(true));
    }

    #[test]
    fn test_get_bool_opt_missing() {
        let args = json!({});
        assert_eq!(get_bool_opt(&args, "flag"), None);
    }
}
