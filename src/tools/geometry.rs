use crate::error::McpResult;
use crate::utils::args::{get_number, result_json};
use serde_json::Value;

pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "area_circle",
            "description": "Calculate the area of a circle",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "radius": {"type": "number", "description": "Radius of the circle"}
                },
                "required": ["radius"]
            }
        }),
        serde_json::json!({
            "name": "area_rectangle",
            "description": "Calculate the area of a rectangle",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "length": {"type": "number", "description": "Length of rectangle"},
                    "width": {"type": "number", "description": "Width of rectangle"}
                },
                "required": ["length", "width"]
            }
        }),
        serde_json::json!({
            "name": "area_triangle",
            "description": "Calculate the area of a triangle",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "base": {"type": "number", "description": "Base of triangle"},
                    "height": {"type": "number", "description": "Height of triangle"}
                },
                "required": ["base", "height"]
            }
        }),
        serde_json::json!({
            "name": "area_trapezoid",
            "description": "Calculate the area of a trapezoid",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "base1": {"type": "number", "description": "First base length"},
                    "base2": {"type": "number", "description": "Second base length"},
                    "height": {"type": "number", "description": "Height of trapezoid"}
                },
                "required": ["base1", "base2", "height"]
            }
        }),
        serde_json::json!({
            "name": "volume_sphere",
            "description": "Calculate the volume of a sphere",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "radius": {"type": "number", "description": "Radius of sphere"}
                },
                "required": ["radius"]
            }
        }),
        serde_json::json!({
            "name": "volume_cylinder",
            "description": "Calculate the volume of a cylinder",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "radius": {"type": "number", "description": "Radius of base"},
                    "height": {"type": "number", "description": "Height of cylinder"}
                },
                "required": ["radius", "height"]
            }
        }),
        serde_json::json!({
            "name": "volume_cone",
            "description": "Calculate the volume of a cone",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "radius": {"type": "number", "description": "Radius of base"},
                    "height": {"type": "number", "description": "Height of cone"}
                },
                "required": ["radius", "height"]
            }
        }),
        serde_json::json!({
            "name": "volume_rectangular_prism",
            "description": "Calculate the volume of a rectangular prism (box)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "length": {"type": "number", "description": "Length"},
                    "width": {"type": "number", "description": "Width"},
                    "height": {"type": "number", "description": "Height"}
                },
                "required": ["length", "width", "height"]
            }
        }),
    ]
}

pub fn execute(name: &str, arguments: &Value) -> McpResult<Value> {
    match name {
        "area_circle" => {
            let radius = get_number(arguments, "radius")?;
            Ok(result_json(area_circle(radius)?))
        }
        "area_rectangle" => {
            let length = get_number(arguments, "length")?;
            let width = get_number(arguments, "width")?;
            Ok(result_json(area_rectangle(length, width)?))
        }
        "area_triangle" => {
            let base = get_number(arguments, "base")?;
            let height = get_number(arguments, "height")?;
            Ok(result_json(area_triangle(base, height)?))
        }
        "area_trapezoid" => {
            let base1 = get_number(arguments, "base1")?;
            let base2 = get_number(arguments, "base2")?;
            let height = get_number(arguments, "height")?;
            Ok(result_json(area_trapezoid(base1, base2, height)?))
        }
        "volume_sphere" => {
            let radius = get_number(arguments, "radius")?;
            Ok(result_json(volume_sphere(radius)?))
        }
        "volume_cylinder" => {
            let radius = get_number(arguments, "radius")?;
            let height = get_number(arguments, "height")?;
            Ok(result_json(volume_cylinder(radius, height)?))
        }
        "volume_cone" => {
            let radius = get_number(arguments, "radius")?;
            let height = get_number(arguments, "height")?;
            Ok(result_json(volume_cone(radius, height)?))
        }
        "volume_rectangular_prism" => {
            let length = get_number(arguments, "length")?;
            let width = get_number(arguments, "width")?;
            let height = get_number(arguments, "height")?;
            Ok(result_json(volume_rectangular_prism(
                length, width, height,
            )?))
        }
        _ => Err(crate::error::McpError::tool_error(format!(
            "Unknown geometry tool: {}",
            name
        ))),
    }
}

fn area_circle(radius: f64) -> McpResult<f64> {
    Ok(std::f64::consts::PI * radius * radius)
}

fn area_rectangle(length: f64, width: f64) -> McpResult<f64> {
    Ok(length * width)
}

fn area_triangle(base: f64, height: f64) -> McpResult<f64> {
    Ok(0.5 * base * height)
}

fn area_trapezoid(base1: f64, base2: f64, height: f64) -> McpResult<f64> {
    Ok(0.5 * (base1 + base2) * height)
}

fn volume_sphere(radius: f64) -> McpResult<f64> {
    Ok((4.0 / 3.0) * std::f64::consts::PI * radius.powi(3))
}

fn volume_cylinder(radius: f64, height: f64) -> McpResult<f64> {
    Ok(std::f64::consts::PI * radius * radius * height)
}

fn volume_cone(radius: f64, height: f64) -> McpResult<f64> {
    Ok((1.0 / 3.0) * std::f64::consts::PI * radius * radius * height)
}

fn volume_rectangular_prism(length: f64, width: f64, height: f64) -> McpResult<f64> {
    Ok(length * width * height)
}
