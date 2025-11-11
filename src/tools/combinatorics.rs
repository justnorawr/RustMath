use crate::error::McpResult;
use serde_json::Value;
use crate::utils::args::{get_number, result_json};

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "permutation",
            "description": "Calculate permutations: P(n, r) = n! / (n - r)!",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "n": {"type": "number", "description": "Total number of items"},
                    "r": {"type": "number", "description": "Number of items to arrange"}
                },
                "required": ["n", "r"]
            }
        }),
        serde_json::json!({
            "name": "combination",
            "description": "Calculate combinations: C(n, r) = n! / (r! × (n - r)!)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "n": {"type": "number", "description": "Total number of items"},
                    "r": {"type": "number", "description": "Number of items to choose"}
                },
                "required": ["n", "r"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "permutation" => {
            let n = get_number(arguments, "n")?;
            let r = get_number(arguments, "r")?;
            Ok(result_json(permutation(n, r)?))
        }
        "combination" => {
            let n = get_number(arguments, "n")?;
            let r = get_number(arguments, "r")?;
            Ok(result_json(combination(n, r)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!("Unknown combinatorics tool: {}", name))),
    }
}

fn permutation(n: f64, r: f64) -> McpResult<f64> {
    use crate::utils::validation::validate_integer;

    let n_int = validate_integer(n, "n")?;
    let r_int = validate_integer(r, "r")?;

    if n_int < 0 || r_int < 0 {
        return Err(crate::error::McpError::validation_error(
            "Permutation: n and r must be non-negative"
        ));
    }

    if r_int > n_int {
        return Err(crate::error::McpError::validation_error(
            "Permutation: r must be <= n"
        ));
    }

    // Limit to prevent overflow (conservative limit)
    if n_int > 170 {
        return Err(crate::error::McpError::validation_error(
            "Permutation overflow: n must be <= 170"
        ));
    }

    // Use checked arithmetic
    let mut result = 1u64;
    for i in 0..r_int {
        let factor = (n_int - i) as u64;
        result = result.checked_mul(factor)
            .ok_or_else(|| crate::error::McpError::validation_error(
                format!("Permutation overflow at factor {}", factor)
            ))?;
    }

    Ok(result as f64)
}

fn combination(n: f64, r: f64) -> McpResult<f64> {
    use crate::utils::validation::validate_integer;

    let n_int = validate_integer(n, "n")?;
    let r_int = validate_integer(r, "r")?;

    if n_int < 0 || r_int < 0 {
        return Err(crate::error::McpError::validation_error(
            "Combination: n and r must be non-negative"
        ));
    }

    if r_int > n_int {
        return Err(crate::error::McpError::validation_error(
            "Combination: r must be <= n"
        ));
    }

    // Limit to prevent overflow
    if n_int > 170 {
        return Err(crate::error::McpError::validation_error(
            "Combination overflow: n must be <= 170"
        ));
    }

    // Use smaller of r and n-r for efficiency
    let r_opt = if r_int > n_int / 2 { n_int - r_int } else { r_int };

    // Calculate using checked arithmetic to prevent overflow
    // C(n,r) = (n * (n-1) * ... * (n-r+1)) / (r * (r-1) * ... * 1)
    let mut result = 1u64;
    for i in 0..r_opt {
        result = result.checked_mul((n_int - i) as u64)
            .ok_or_else(|| crate::error::McpError::validation_error(
                "Combination overflow during multiplication"
            ))?;
        result = result.checked_div((i + 1) as u64)
            .ok_or_else(|| crate::error::McpError::validation_error(
                "Combination: division error"
            ))?;
    }

    Ok(result as f64)
}

