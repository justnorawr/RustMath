use crate::error::McpResult;
use crate::utils::args::{get_number, result_json};
use serde_json::Value;

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "gcd",
            "description": "Calculate the greatest common divisor of two numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "First number"},
                    "b": {"type": "number", "description": "Second number"}
                },
                "required": ["a", "b"]
            }
        }),
        serde_json::json!({
            "name": "lcm",
            "description": "Calculate the least common multiple of two numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "First number"},
                    "b": {"type": "number", "description": "Second number"}
                },
                "required": ["a", "b"]
            }
        }),
        serde_json::json!({
            "name": "factorial",
            "description": "Calculate the factorial of a non-negative integer",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "n": {"type": "number", "description": "Non-negative integer"}
                },
                "required": ["n"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "gcd" => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            Ok(result_json(gcd(a, b)?))
        }
        "lcm" => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            Ok(result_json(lcm(a, b)?))
        }
        "factorial" => {
            let n = get_number(arguments, "n")?;
            Ok(result_json(factorial(n)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!(
            "Unknown algebra tool: {}",
            name
        ))),
    }
}

fn gcd(a: f64, b: f64) -> McpResult<f64> {
    let mut a = a.abs() as i64;
    let mut b = b.abs() as i64;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    Ok(a as f64)
}

fn lcm(a: f64, b: f64) -> McpResult<f64> {
    use crate::utils::validation::validate_integer;

    let a_int = validate_integer(a, "a")?;
    let b_int = validate_integer(b, "b")?;

    let a_abs = a_int.abs();
    let b_abs = b_int.abs();

    if a_abs == 0 || b_abs == 0 {
        return Ok(0.0);
    }

    // Calculate GCD first
    let gcd_val = gcd(a, b)? as i64;

    // Use checked arithmetic to prevent overflow: lcm(a,b) = (a / gcd) * b
    let result = (a_abs / gcd_val).checked_mul(b_abs).ok_or_else(|| {
        crate::error::McpError::validation_error("LCM calculation would overflow")
    })?;

    Ok(result as f64)
}

fn factorial(n: f64) -> McpResult<f64> {
    use crate::utils::validation::validate_integer;

    let n_int = validate_integer(n, "n")?;

    if n_int < 0 {
        return Err(crate::error::McpError::validation_error(
            "Factorial is not defined for negative numbers",
        ));
    }

    // Limit factorial to prevent overflow (20! < u64::MAX, but 21! overflows)
    if n_int > 170 {
        return Err(crate::error::McpError::validation_error(
            "Factorial overflow: n must be <= 170 to prevent overflow",
        ));
    }

    // Use checked multiplication to detect overflow
    let mut result = 1u64;
    for i in 1..=n_int as u64 {
        result = result.checked_mul(i).ok_or_else(|| {
            crate::error::McpError::validation_error(format!("Factorial overflow at i={}", i))
        })?;
    }

    Ok(result as f64)
}
