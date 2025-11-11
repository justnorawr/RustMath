use crate::error::McpResult;
use crate::utils::args::{get_number, get_number_array, get_number_opt, result_json};
use serde_json::Value;

// Tool name constants
pub const TOOL_ADD: &str = "add";
pub const TOOL_SUBTRACT: &str = "subtract";
pub const TOOL_MULTIPLY: &str = "multiply";
pub const TOOL_DIVIDE: &str = "divide";
pub const TOOL_POWER: &str = "power";
pub const TOOL_SQRT: &str = "sqrt";
pub const TOOL_ABS: &str = "abs";
pub const TOOL_ROUND: &str = "round";
pub const TOOL_FLOOR: &str = "floor";
pub const TOOL_CEIL: &str = "ceil";
pub const TOOL_MODULO: &str = "modulo";

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": TOOL_ADD,
            "description": "Add two or more numbers together",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers to add"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": TOOL_SUBTRACT,
            "description": "Subtract numbers. Subtracts all subsequent numbers from the first.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "First number"},
                    "b": {"type": "number", "description": "Number to subtract"}
                },
                "required": ["a", "b"]
            }
        }),
        serde_json::json!({
            "name": TOOL_MULTIPLY,
            "description": "Multiply two or more numbers together",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "numbers": {
                        "type": "array",
                        "items": {"type": "number"},
                        "description": "Array of numbers to multiply"
                    }
                },
                "required": ["numbers"]
            }
        }),
        serde_json::json!({
            "name": TOOL_DIVIDE,
            "description": "Divide two numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "Dividend"},
                    "b": {"type": "number", "description": "Divisor"}
                },
                "required": ["a", "b"]
            }
        }),
        serde_json::json!({
            "name": TOOL_POWER,
            "description": "Raise a number to a power",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "base": {"type": "number", "description": "Base number"},
                    "exponent": {"type": "number", "description": "Exponent"}
                },
                "required": ["base", "exponent"]
            }
        }),
        serde_json::json!({
            "name": TOOL_SQRT,
            "description": "Calculate the square root of a number",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "number": {"type": "number", "description": "Number to take square root of"}
                },
                "required": ["number"]
            }
        }),
        serde_json::json!({
            "name": TOOL_ABS,
            "description": "Get the absolute value of a number",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "number": {"type": "number", "description": "Number"}
                },
                "required": ["number"]
            }
        }),
        serde_json::json!({
            "name": TOOL_ROUND,
            "description": "Round a number to the nearest integer",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "number": {"type": "number", "description": "Number to round"},
                    "decimals": {"type": "number", "description": "Number of decimal places (default: 0)"}
                },
                "required": ["number"]
            }
        }),
        serde_json::json!({
            "name": TOOL_FLOOR,
            "description": "Round down to the nearest integer",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "number": {"type": "number", "description": "Number"}
                },
                "required": ["number"]
            }
        }),
        serde_json::json!({
            "name": TOOL_CEIL,
            "description": "Round up to the nearest integer",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "number": {"type": "number", "description": "Number"}
                },
                "required": ["number"]
            }
        }),
        serde_json::json!({
            "name": TOOL_MODULO,
            "description": "Calculate the remainder of division",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "Dividend"},
                    "b": {"type": "number", "description": "Divisor"}
                },
                "required": ["a", "b"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        TOOL_ADD => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(add(numbers)?))
        }
        TOOL_SUBTRACT => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            Ok(result_json(subtract(a, b)?))
        }
        TOOL_MULTIPLY => {
            let numbers = get_number_array(arguments, "numbers")?;
            Ok(result_json(multiply(numbers)?))
        }
        TOOL_DIVIDE => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            Ok(result_json(divide(a, b)?))
        }
        TOOL_POWER => {
            let base = get_number(arguments, "base")?;
            let exponent = get_number(arguments, "exponent")?;
            Ok(result_json(power(base, exponent)?))
        }
        TOOL_SQRT => {
            let number = get_number(arguments, "number")?;
            Ok(result_json(sqrt(number)?))
        }
        TOOL_ABS => {
            let number = get_number(arguments, "number")?;
            Ok(result_json(abs(number)?))
        }
        TOOL_ROUND => {
            let number = get_number(arguments, "number")?;
            let decimals = get_number_opt(arguments, "decimals");
            Ok(result_json(round(number, decimals)?))
        }
        TOOL_FLOOR => {
            let number = get_number(arguments, "number")?;
            Ok(result_json(floor(number)?))
        }
        TOOL_CEIL => {
            let number = get_number(arguments, "number")?;
            Ok(result_json(ceil(number)?))
        }
        TOOL_MODULO => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            Ok(result_json(modulo(a, b)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!(
            "Unknown basic math tool: {}",
            name
        ))),
    }
}

// Implementation functions
fn add(numbers: Vec<f64>) -> McpResult<f64> {
    Ok(numbers.iter().sum())
}

fn subtract(a: f64, b: f64) -> McpResult<f64> {
    Ok(a - b)
}

fn multiply(numbers: Vec<f64>) -> McpResult<f64> {
    Ok(numbers.iter().product())
}

fn divide(a: f64, b: f64) -> McpResult<f64> {
    if b == 0.0 {
        return Err(crate::error::McpError::validation_error("Division by zero"));
    }
    Ok(a / b)
}

fn power(base: f64, exponent: f64) -> McpResult<f64> {
    Ok(base.powf(exponent))
}

fn sqrt(number: f64) -> McpResult<f64> {
    if number < 0.0 {
        return Err(crate::error::McpError::validation_error(
            "Cannot take square root of negative number",
        ));
    }
    Ok(number.sqrt())
}

fn abs(number: f64) -> McpResult<f64> {
    Ok(number.abs())
}

fn round(number: f64, decimals: Option<f64>) -> McpResult<f64> {
    let places = decimals.unwrap_or(0.0) as i32;
    let multiplier = 10.0_f64.powi(places);
    Ok((number * multiplier).round() / multiplier)
}

fn floor(number: f64) -> McpResult<f64> {
    Ok(number.floor())
}

fn ceil(number: f64) -> McpResult<f64> {
    Ok(number.ceil())
}

fn modulo(a: f64, b: f64) -> McpResult<f64> {
    if b == 0.0 {
        return Err(crate::error::McpError::validation_error("Modulo by zero"));
    }
    Ok(a % b)
}
