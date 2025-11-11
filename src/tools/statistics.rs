use crate::error::McpResult;
use crate::utils::args::{get_bool_opt, get_number_array, result_json, result_value};
use serde_json::Value;
use std::collections::HashMap;

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "mean",
            "description": "Calculate the arithmetic mean (average) of a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "median",
            "description": "Calculate the median of a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "mode",
            "description": "Find the mode (most frequently occurring value) of a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "std_dev",
            "description": "Calculate the standard deviation of a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    },
                    "sample": {
                        "type": "boolean",
                        "description": "If true, calculate sample standard deviation (n-1), otherwise population (n)"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "variance",
            "description": "Calculate the variance of a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    },
                    "sample": {
                        "type": "boolean",
                        "description": "If true, calculate sample variance (n-1), otherwise population (n)"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "min",
            "description": "Find the minimum value in a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "max",
            "description": "Find the maximum value in a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "sum",
            "description": "Calculate the sum of a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": "product",
            "description": "Calculate the product of a list of numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers"
                    }
                },
                "required": ["numbers"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "mean" => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(mean(numbers)?))
        }
        "median" => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(median(numbers)?))
        }
        "mode" => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_value(mode(numbers)?))
        }
        "variance" => {
            let numbers = get_number_array(arguments, "numbers")?;
            let sample = get_bool_opt(arguments, "sample");
            Ok(result_json(variance(numbers, sample)?))
        }
        "std_dev" => {
            let numbers = get_number_array(arguments, "numbers")?;
            let sample = get_bool_opt(arguments, "sample");
            Ok(result_json(std_dev(numbers, sample)?))
        }
        "min" => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(min(numbers)?))
        }
        "max" => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(max(numbers)?))
        }
        "sum" => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(sum(numbers)?))
        }
        "product" => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(product(numbers)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!(
            "Unknown statistics tool: {}",
            name
        ))),
    }
}

fn mean(numbers: Vec<f64>) -> McpResult<f64> {
    if numbers.is_empty() {
        return Err(crate::error::McpError::validation_error(
            "Cannot calculate mean of empty array",
        ));
    }
    Ok(numbers.iter().sum::<f64>() / numbers.len() as f64)
}

fn median(numbers: Vec<f64>) -> McpResult<f64> {
    if numbers.is_empty() {
        return Err(crate::error::McpError::validation_error(
            "Cannot calculate median of empty array",
        ));
    }
    let mut sorted = numbers;
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted.len();
    if len.is_multiple_of(2) {
        Ok((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
    } else {
        Ok(sorted[len / 2])
    }
}

fn mode(numbers: Vec<f64>) -> McpResult<Value> {
    if numbers.is_empty() {
        return Err(crate::error::McpError::validation_error(
            "Cannot calculate mode of empty array",
        ));
    }
    let mut frequency: HashMap<String, usize> = HashMap::new();
    for num in &numbers {
        let key = format!("{:.10}", num);
        *frequency.entry(key).or_insert(0) += 1;
    }
    let max_freq = frequency.values().max().copied().unwrap_or(0);
    let modes: Vec<f64> = frequency
        .iter()
        .filter(|(_, &freq)| freq == max_freq)
        .filter_map(|(key, _)| key.parse::<f64>().ok())
        .collect();

    if modes.len() == numbers.len() {
        Ok(serde_json::json!({"mode": null, "message": "No mode - all values are unique"}))
    } else {
        Ok(serde_json::json!({"mode": modes, "frequency": max_freq}))
    }
}

fn variance(numbers: Vec<f64>, sample: Option<bool>) -> McpResult<f64> {
    if numbers.is_empty() {
        return Err(crate::error::McpError::validation_error(
            "Cannot calculate variance of empty array",
        ));
    }
    let mean_val = mean(numbers.clone())?;
    let n = numbers.len() as f64;
    let divisor = if sample.unwrap_or(false) && n > 1.0 {
        n - 1.0
    } else {
        n
    };
    let sum_squared_diff: f64 = numbers.iter().map(|x| (x - mean_val).powi(2)).sum();
    Ok(sum_squared_diff / divisor)
}

fn std_dev(numbers: Vec<f64>, sample: Option<bool>) -> McpResult<f64> {
    Ok(variance(numbers, sample)?.sqrt())
}

fn min(numbers: Vec<f64>) -> McpResult<f64> {
    numbers
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .ok_or_else(|| crate::error::McpError::validation_error("Cannot find min of empty array"))
}

fn max(numbers: Vec<f64>) -> McpResult<f64> {
    numbers
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .ok_or_else(|| crate::error::McpError::validation_error("Cannot find max of empty array"))
}

fn sum(numbers: Vec<f64>) -> McpResult<f64> {
    Ok(numbers.iter().sum())
}

fn product(numbers: Vec<f64>) -> McpResult<f64> {
    Ok(numbers.iter().product())
}
