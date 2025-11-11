use crate::error::McpResult;
use crate::utils::args::{get_bool_opt, get_number, get_number_opt, result_json};
use serde_json::Value;

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "exponential_growth",
            "description": "Calculate exponential growth: A = P × e^(rt) or A = P × (1 + r)^t",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "initial": {"type": "number", "description": "Initial value (P)"},
                    "rate": {"type": "number", "description": "Growth rate (as decimal)"},
                    "time": {"type": "number", "description": "Time period"},
                    "continuous": {"type": "boolean", "description": "If true, use continuous compounding (e^rt), otherwise discrete (1+r)^t"}
                },
                "required": ["initial", "rate", "time"]
            }
        }),
        serde_json::json!({
            "name": "logarithm",
            "description": "Calculate logarithm: log_base(value) or natural log",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "value": {"type": "number", "description": "Value to take logarithm of"},
                    "base": {"type": "number", "description": "Base of logarithm (default: e for natural log, 10 for common log)"},
                    "natural": {"type": "boolean", "description": "If true, use natural logarithm (ln)"}
                },
                "required": ["value"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "exponential_growth" => {
            let initial = get_number(arguments, "initial")?;
            let rate = get_number(arguments, "rate")?;
            let time = get_number(arguments, "time")?;
            let continuous = get_bool_opt(arguments, "continuous");
            Ok(result_json(exponential_growth(
                initial, rate, time, continuous,
            )?))
        }
        "logarithm" => {
            let value = get_number(arguments, "value")?;
            let base = get_number_opt(arguments, "base");
            let natural = get_bool_opt(arguments, "natural");
            Ok(result_json(logarithm(value, base, natural)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!(
            "Unknown advanced tool: {}",
            name
        ))),
    }
}

fn exponential_growth(
    initial: f64,
    rate: f64,
    time: f64,
    continuous: Option<bool>,
) -> McpResult<f64> {
    if continuous.unwrap_or(false) {
        Ok(initial * (rate * time).exp())
    } else {
        Ok(initial * (1.0 + rate).powf(time))
    }
}

fn logarithm(value: f64, base: Option<f64>, natural: Option<bool>) -> McpResult<f64> {
    if value <= 0.0 {
        return Err(crate::error::McpError::validation_error(
            "Logarithm is undefined for non-positive values",
        ));
    }
    if natural.unwrap_or(false) {
        Ok(value.ln())
    } else if let Some(b) = base {
        if b <= 0.0 || b == 1.0 {
            return Err(crate::error::McpError::validation_error(
                "Invalid base for logarithm",
            ));
        }
        Ok(value.log(b))
    } else {
        Ok(value.log10())
    }
}
