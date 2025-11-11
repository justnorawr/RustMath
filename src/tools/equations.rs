use crate::error::McpResult;
use crate::utils::args::{get_number, get_number_opt, result_json, result_value};
use serde_json::Value;

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "quadratic_formula",
            "description": "Solve quadratic equation ax² + bx + c = 0 using the quadratic formula",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "Coefficient of x²"},
                    "b": {"type": "number", "description": "Coefficient of x"},
                    "c": {"type": "number", "description": "Constant term"}
                },
                "required": ["a", "b", "c"]
            }
        }),
        serde_json::json!({
            "name": "distance_formula",
            "description": "Calculate distance between two points (x1, y1) and (x2, y2)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "x1": {"type": "number", "description": "X coordinate of first point"},
                    "y1": {"type": "number", "description": "Y coordinate of first point"},
                    "x2": {"type": "number", "description": "X coordinate of second point"},
                    "y2": {"type": "number", "description": "Y coordinate of second point"}
                },
                "required": ["x1", "y1", "x2", "y2"]
            }
        }),
        serde_json::json!({
            "name": "pythagorean_theorem",
            "description": "Calculate the third side of a right triangle using Pythagorean theorem (a² + b² = c²)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "Length of side a"},
                    "b": {"type": "number", "description": "Length of side b"},
                    "c": {"type": "number", "description": "Length of hypotenuse (leave 0 to calculate)"}
                },
                "required": ["a", "b"]
            }
        }),
        serde_json::json!({
            "name": "slope",
            "description": "Calculate the slope of a line between two points: m = (y2 - y1) / (x2 - x1)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "x1": {"type": "number", "description": "X coordinate of first point"},
                    "y1": {"type": "number", "description": "Y coordinate of first point"},
                    "x2": {"type": "number", "description": "X coordinate of second point"},
                    "y2": {"type": "number", "description": "Y coordinate of second point"}
                },
                "required": ["x1", "y1", "x2", "y2"]
            }
        }),
        serde_json::json!({
            "name": "midpoint",
            "description": "Calculate the midpoint between two points",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "x1": {"type": "number", "description": "X coordinate of first point"},
                    "y1": {"type": "number", "description": "Y coordinate of first point"},
                    "x2": {"type": "number", "description": "X coordinate of second point"},
                    "y2": {"type": "number", "description": "Y coordinate of second point"}
                },
                "required": ["x1", "y1", "x2", "y2"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "quadratic_formula" => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            let c = get_number(arguments, "c")?;
            Ok(quadratic_formula(a, b, c)?)
        }
        "distance_formula" => {
            let x1 = get_number(arguments, "x1")?;
            let y1 = get_number(arguments, "y1")?;
            let x2 = get_number(arguments, "x2")?;
            let y2 = get_number(arguments, "y2")?;
            Ok(result_json(distance_formula(x1, y1, x2, y2)?))
        }
        "pythagorean_theorem" => {
            let a = get_number(arguments, "a")?;
            let b = get_number(arguments, "b")?;
            let c = get_number_opt(arguments, "c");
            Ok(result_json(pythagorean_theorem(a, b, c)?))
        }
        "slope" => {
            let x1 = get_number(arguments, "x1")?;
            let y1 = get_number(arguments, "y1")?;
            let x2 = get_number(arguments, "x2")?;
            let y2 = get_number(arguments, "y2")?;
            Ok(result_json(slope(x1, y1, x2, y2)?))
        }
        "midpoint" => {
            let x1 = get_number(arguments, "x1")?;
            let y1 = get_number(arguments, "y1")?;
            let x2 = get_number(arguments, "x2")?;
            let y2 = get_number(arguments, "y2")?;
            Ok(result_value(midpoint(x1, y1, x2, y2)?))
        }
        _ => Err(crate::error::McpError::tool_error(format!(
            "Unknown equations tool: {}",
            name
        ))),
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> McpResult<Value> {
    if a == 0.0 {
        return Err(crate::error::McpError::validation_error(
            "Coefficient 'a' cannot be zero for quadratic equation",
        ));
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        Ok(serde_json::json!({
            "roots": null,
            "discriminant": discriminant,
            "message": "No real roots (complex roots exist)"
        }))
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        Ok(serde_json::json!({
            "roots": [root, root],
            "discriminant": discriminant,
            "type": "repeated"
        }))
    } else {
        let sqrt_disc = discriminant.sqrt();
        let root1 = (-b + sqrt_disc) / (2.0 * a);
        let root2 = (-b - sqrt_disc) / (2.0 * a);
        Ok(serde_json::json!({
            "roots": [root1, root2],
            "discriminant": discriminant,
            "type": "distinct"
        }))
    }
}

fn distance_formula(x1: f64, y1: f64, x2: f64, y2: f64) -> McpResult<f64> {
    Ok(((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt())
}

fn pythagorean_theorem(a: f64, b: f64, c: Option<f64>) -> McpResult<f64> {
    if let Some(c_val) = c {
        if c_val == 0.0 {
            Ok((a * a + b * b).sqrt())
        } else if a == 0.0 {
            Ok((c_val * c_val - b * b).sqrt())
        } else if b == 0.0 {
            Ok((c_val * c_val - a * a).sqrt())
        } else {
            Err(crate::error::McpError::validation_error(
                "Cannot determine which side to calculate",
            ))
        }
    } else {
        Ok((a * a + b * b).sqrt())
    }
}

fn slope(x1: f64, y1: f64, x2: f64, y2: f64) -> McpResult<f64> {
    if (x2 - x1).abs() < 1e-10 {
        return Err(crate::error::McpError::validation_error(
            "Slope is undefined (vertical line)",
        ));
    }
    Ok((y2 - y1) / (x2 - x1))
}

fn midpoint(x1: f64, y1: f64, x2: f64, y2: f64) -> McpResult<Value> {
    Ok(serde_json::json!({
        "x": (x1 + x2) / 2.0,
        "y": (y1 + y2) / 2.0
    }))
}
