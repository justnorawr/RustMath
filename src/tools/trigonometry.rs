use crate::error::McpResult;
use serde_json::Value;
use crate::utils::args::{get_number, get_number_opt, result_json, result_value};

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "sin",
            "description": "Calculate sine of an angle (in radians)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "angle": {"type": "number", "description": "Angle in radians"}
                },
                "required": ["angle"]
            }
        }),
        serde_json::json!({
            "name": "cos",
            "description": "Calculate cosine of an angle (in radians)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "angle": {"type": "number", "description": "Angle in radians"}
                },
                "required": ["angle"]
            }
        }),
        serde_json::json!({
            "name": "tan",
            "description": "Calculate tangent of an angle (in radians)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "angle": {"type": "number", "description": "Angle in radians"}
                },
                "required": ["angle"]
            }
        }),
        serde_json::json!({
            "name": "asin",
            "description": "Calculate arcsine (inverse sine) in radians",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "value": {"type": "number", "description": "Value between -1 and 1"}
                },
                "required": ["value"]
            }
        }),
        serde_json::json!({
            "name": "acos",
            "description": "Calculate arccosine (inverse cosine) in radians",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "value": {"type": "number", "description": "Value between -1 and 1"}
                },
                "required": ["value"]
            }
        }),
        serde_json::json!({
            "name": "atan",
            "description": "Calculate arctangent (inverse tangent) in radians",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "value": {"type": "number", "description": "Value"}
                },
                "required": ["value"]
            }
        }),
        serde_json::json!({
            "name": "law_of_cosines",
            "description": "Calculate side or angle using Law of Cosines: c² = a² + b² - 2ab cos(C)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "Side a"},
                    "b": {"type": "number", "description": "Side b"},
                    "c": {"type": "number", "description": "Side c (leave 0 to calculate)"},
                    "angle_c": {"type": "number", "description": "Angle C in radians (required if calculating c)"}
                },
                "required": ["a", "b"]
            }
        }),
        serde_json::json!({
            "name": "law_of_sines",
            "description": "Calculate side or angle using Law of Sines: a/sin(A) = b/sin(B) = c/sin(C)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "side_a": {"type": "number", "description": "Side a (leave 0 if calculating)"},
                    "angle_a": {"type": "number", "description": "Angle A in radians"},
                    "side_b": {"type": "number", "description": "Side b (leave 0 if calculating)"},
                    "angle_b": {"type": "number", "description": "Angle B in radians"}
                },
                "required": ["angle_a", "angle_b"]
            }
        }),
        serde_json::json!({
            "name": "degrees_to_radians",
            "description": "Convert degrees to radians",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "degrees": {"type": "number", "description": "Angle in degrees"}
                },
                "required": ["degrees"]
            }
        }),
        serde_json::json!({
            "name": "radians_to_degrees",
            "description": "Convert radians to degrees",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "radians": {"type": "number", "description": "Angle in radians"}
                },
                "required": ["radians"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "sin" => {
            let angle = get_number(arguments, "angle")?;
            Ok(result_json(sin(angle)?))
        }
        "cos" => {
            let angle = get_number(arguments, "angle")?;
            Ok(result_json(cos(angle)?))
        }
        "tan" => {
            let angle = get_number(arguments, "angle")?;
            Ok(result_json(tan(angle)?))
        }
        "asin" => {
            let value = get_number(arguments, "value")?;
            Ok(result_json(asin(value)?))
        }
        "acos" => {
            let value = get_number(arguments, "value")?;
            Ok(result_json(acos(value)?))
        }
        "atan" => {
            let value = get_number(arguments, "value")?;
            Ok(result_json(atan(value)?))
        }
        "law_of_cosines" => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            let c = get_number_opt(arguments, "c");
            let angle_c = get_number_opt(arguments, "angle_c");
            Ok(result_value(law_of_cosines(a, b, c, angle_c)?))
        }
        "law_of_sines" => {
            let side_a = get_number_opt(arguments, "side_a");
            let angle_a = get_number(arguments, "angle_a")?;
            let side_b = get_number_opt(arguments, "side_b");
            let angle_b = get_number(arguments, "angle_b")?;
            Ok(result_value(law_of_sines(side_a, angle_a, side_b, angle_b)?))
        }
        "degrees_to_radians" => {
            let degrees = get_number(arguments, "degrees")?;
            Ok(result_json(degrees_to_radians(degrees)?))
        }
        "radians_to_degrees" => {
            let radians = get_number(arguments, "radians")?;
            Ok(result_json(radians_to_degrees(radians)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!("Unknown trigonometry tool: {}", name))),
    }
}

fn sin(angle: f64) -> McpResult<f64> {
    Ok(angle.sin())
}

fn cos(angle: f64) -> McpResult<f64> {
    Ok(angle.cos())
}

fn tan(angle: f64) -> McpResult<f64> {
    Ok(angle.tan())
}

fn asin(value: f64) -> McpResult<f64> {
    if !(-1.0..=1.0).contains(&value) {
        return Err(crate::error::McpError::validation_error("Value must be between -1 and 1 for arcsine"));
    }
    Ok(value.asin())
}

fn acos(value: f64) -> McpResult<f64> {
    if !(-1.0..=1.0).contains(&value) {
        return Err(crate::error::McpError::validation_error("Value must be between -1 and 1 for arccosine"));
    }
    Ok(value.acos())
}

fn atan(value: f64) -> McpResult<f64> {
    Ok(value.atan())
}

fn law_of_cosines(a: f64, b: f64, c: Option<f64>, angle_c: Option<f64>) -> McpResult<Value> {
    if let Some(c_val) = c {
        if c_val == 0.0 {
            if let Some(angle) = angle_c {
                let c_calc = (a * a + b * b - 2.0 * a * b * angle.cos()).sqrt();
                Ok(serde_json::json!({ "side_c": c_calc }))
            } else {
                Err(crate::error::McpError::validation_error("Angle C is required to calculate side c"))
            }
        } else {
            let cos_c = (a * a + b * b - c_val * c_val) / (2.0 * a * b);
            if cos_c.abs() > 1.0 {
                return Err(crate::error::McpError::validation_error("Invalid triangle: sides do not satisfy triangle inequality"));
            }
            Ok(serde_json::json!({ "angle_c": cos_c.acos() }))
        }
    } else {
        Err(crate::error::McpError::validation_error("Must provide side c or set it to 0 to calculate"))
    }
}

fn law_of_sines(side_a: Option<f64>, angle_a: f64, side_b: Option<f64>, angle_b: f64) -> McpResult<Value> {
    match (side_a, side_b) {
        (Some(a), None) => {
            let b = a * angle_b.sin() / angle_a.sin();
            Ok(serde_json::json!({ "side_b": b }))
        }
        (None, Some(b)) => {
            let a = b * angle_a.sin() / angle_b.sin();
            Ok(serde_json::json!({ "side_a": a }))
        }
        (Some(a), Some(b)) => {
            let ratio_a = a / angle_a.sin();
            let ratio_b = b / angle_b.sin();
            Ok(serde_json::json!({
                "ratio_a": ratio_a,
                "ratio_b": ratio_b,
                "match": (ratio_a - ratio_b).abs() < 1e-10
            }))
        }
        (None, None) => Err(crate::error::McpError::validation_error("Must provide at least one side")),
    }
}

fn degrees_to_radians(degrees: f64) -> McpResult<f64> {
    Ok(degrees * std::f64::consts::PI / 180.0)
}

fn radians_to_degrees(radians: f64) -> McpResult<f64> {
    Ok(radians * 180.0 / std::f64::consts::PI)
}

