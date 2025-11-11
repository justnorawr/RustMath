use crate::error::{McpError, McpResult};
use crate::tools::traits::ToolRegistry;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use super::{
    advanced, algebra, basic_math, batch, combinatorics, equations, finance, geometry, statistics,
    trigonometry,
};

/// Tool executor function type
type ToolExecutor = fn(&str, &Value) -> McpResult<Value>;

/// Static tool registry with HashMap for O(1) lookup
static TOOL_REGISTRY: Lazy<HashMap<&'static str, ToolExecutor>> = Lazy::new(|| {
    let mut registry = HashMap::new();

    // Register basic math tools using const strings from modules
    registry.insert(basic_math::TOOL_ADD, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_SUBTRACT, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_MULTIPLY, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_DIVIDE, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_POWER, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_SQRT, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_ABS, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_ROUND, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_FLOOR, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_CEIL, basic_math::execute as ToolExecutor);
    registry.insert(basic_math::TOOL_MODULO, basic_math::execute as ToolExecutor);

    // Register batch operations tool
    registry.insert(batch::TOOL_BATCH, batch::execute as ToolExecutor);

    // Register other tool categories (they still use the old approach temporarily)
    register_tools_legacy(&mut registry, algebra::get_tool_definitions(), algebra::execute);
    register_tools_legacy(&mut registry, statistics::get_tool_definitions(), statistics::execute);
    register_tools_legacy(&mut registry, geometry::get_tool_definitions(), geometry::execute);
    register_tools_legacy(&mut registry, equations::get_tool_definitions(), equations::execute);
    register_tools_legacy(&mut registry, trigonometry::get_tool_definitions(), trigonometry::execute);
    register_tools_legacy(&mut registry, finance::get_tool_definitions(), finance::execute);
    register_tools_legacy(&mut registry, combinatorics::get_tool_definitions(), combinatorics::execute);
    register_tools_legacy(&mut registry, advanced::get_tool_definitions(), advanced::execute);

    registry
});

/// Static tool definitions cache - now using Arc to avoid cloning
static TOOL_DEFINITIONS: Lazy<Arc<Value>> = Lazy::new(|| {
    let mut all_tools = Vec::new();

    all_tools.extend(basic_math::get_tool_definitions());
    all_tools.extend(batch::get_tool_definitions());
    all_tools.extend(algebra::get_tool_definitions());
    all_tools.extend(statistics::get_tool_definitions());
    all_tools.extend(geometry::get_tool_definitions());
    all_tools.extend(equations::get_tool_definitions());
    all_tools.extend(trigonometry::get_tool_definitions());
    all_tools.extend(finance::get_tool_definitions());
    all_tools.extend(combinatorics::get_tool_definitions());
    all_tools.extend(advanced::get_tool_definitions());

    Arc::new(serde_json::json!(all_tools))
});

/// Legacy registration for modules not yet converted to const strings
/// TODO: Remove this once all modules use const strings
fn register_tools_legacy(
    registry: &mut HashMap<&'static str, ToolExecutor>,
    definitions: Vec<Value>,
    executor: ToolExecutor,
) {
    for tool_def in definitions {
        if let Some(name) = tool_def.get("name").and_then(|n| n.as_str()) {
            // Temporarily leak memory for legacy tools - will be removed
            let name_static = Box::leak(name.to_string().into_boxed_str());
            registry.insert(name_static, executor);
        }
    }
}

/// Default tool registry implementation.
///
/// Uses a static HashMap for O(1) tool lookup and cached tool definitions.
/// This is the recommended registry for production use.
pub struct DefaultToolRegistry;

impl ToolRegistry for DefaultToolRegistry {
    fn get_all_tools(&self) -> Value {
        // Clone the Arc (cheap ref count increment) and dereference to get Value
        (**TOOL_DEFINITIONS).clone()
    }

    fn execute_tool(&self, name: &str, arguments: &Value) -> McpResult<Value> {
        TOOL_REGISTRY
            .get(name)
            .ok_or_else(|| McpError::tool_error(format!("Unknown tool: {}", name)))
            .and_then(|executor| executor(name, arguments))
    }
}

/// Get all tool definitions (cached, returns Arc-wrapped Value for efficiency)
pub fn get_all_tools_arc() -> Arc<Value> {
    Arc::clone(&TOOL_DEFINITIONS)
}

/// Get all tool definitions (clones from Arc)
pub fn get_all_tools() -> Value {
    (**TOOL_DEFINITIONS).clone()
}

/// Execute a tool (O(1) lookup)
pub fn execute_tool(name: &str, arguments: &Value) -> McpResult<Value> {
    DefaultToolRegistry.execute_tool(name, arguments)
}

