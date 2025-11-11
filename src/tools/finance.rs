use crate::error::McpResult;
use serde_json::Value;
use crate::utils::args::{get_number, get_number_opt, result_json, result_value};

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "compound_interest",
            "description": "Calculate compound interest: A = P(1 + r/n)^(nt)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "principal": {"type": "number", "description": "Principal amount (P)"},
                    "rate": {"type": "number", "description": "Annual interest rate (as decimal, e.g., 0.05 for 5%)"},
                    "time": {"type": "number", "description": "Time in years (t)"},
                    "compounds_per_year": {"type": "number", "description": "Number of times compounded per year (n), default 1"}
                },
                "required": ["principal", "rate", "time"]
            }
        }),
        serde_json::json!({
            "name": "simple_interest",
            "description": "Calculate simple interest: I = P × r × t",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "principal": {"type": "number", "description": "Principal amount (P)"},
                    "rate": {"type": "number", "description": "Annual interest rate (as decimal)"},
                    "time": {"type": "number", "description": "Time in years (t)"}
                },
                "required": ["principal", "rate", "time"]
            }
        }),
        serde_json::json!({
            "name": "percentage",
            "description": "Calculate percentage: (part / whole) × 100 or find part/whole given percentage",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "part": {"type": "number", "description": "Part value (leave null if calculating)"},
                    "whole": {"type": "number", "description": "Whole value"},
                    "percent": {"type": "number", "description": "Percentage value (leave null if calculating)"}
                },
                "required": ["whole"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "compound_interest" => {
            let principal = get_number(arguments, "principal")?;
            let rate = get_number(arguments, "rate")?;
            let time = get_number(arguments, "time")?;
            let compounds_per_year = get_number_opt(arguments, "compounds_per_year");
            Ok(result_json(compound_interest(principal, rate, time, compounds_per_year)?))
        }
        "simple_interest" => {
            let principal = get_number(arguments, "principal")?;
            let rate = get_number(arguments, "rate")?;
            let time = get_number(arguments, "time")?;
            Ok(result_json(simple_interest(principal, rate, time)?))
        }
        "percentage" => {
            let part = get_number_opt(arguments, "part");
            let whole = get_number(arguments, "whole")?;
            let percent = get_number_opt(arguments, "percent");
            Ok(result_value(percentage(part, whole, percent)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!("Unknown finance tool: {}", name))),
    }
}

fn compound_interest(principal: f64, rate: f64, time: f64, compounds_per_year: Option<f64>) -> McpResult<f64> {
    let n = compounds_per_year.unwrap_or(1.0);
    Ok(principal * (1.0 + rate / n).powf(n * time))
}

fn simple_interest(principal: f64, rate: f64, time: f64) -> McpResult<f64> {
    Ok(principal * rate * time)
}

fn percentage(part: Option<f64>, whole: f64, percent: Option<f64>) -> McpResult<Value> {
    match (part, percent) {
        (Some(p), None) => Ok(serde_json::json!({ "percentage": (p / whole) * 100.0 })),
        (None, Some(perc)) => Ok(serde_json::json!({ "part": (perc / 100.0) * whole })),
        (Some(p), Some(perc)) => {
            let calculated_percent = (p / whole) * 100.0;
            Ok(serde_json::json!({
                "calculated_percentage": calculated_percent,
                "given_percentage": perc,
                "match": (calculated_percent - perc).abs() < 0.0001
            }))
        }
        (None, None) => Err(crate::error::McpError::validation_error("Must provide either 'part' or 'percent'")),
    }
}

