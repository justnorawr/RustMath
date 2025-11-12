use rmcp::{
    ServerHandler,
    model::{CallToolResult, CallToolRequestParam, ErrorCode, ErrorData, Implementation, ListToolsResult, PaginatedRequestParam, ProtocolVersion, ServerCapabilities, ServerInfo, Tool},
    service::{RequestContext, RoleServer},
};
use std::borrow::Cow;

/// MathService implements the ServerHandler for rmcp
/// This bridges the existing tool implementations with the rmcp SDK
#[derive(Clone, Default)]
pub struct MathService;

impl MathService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build tool list from existing implementations
    pub fn _list_tools(&self) -> Vec<Tool> {
        // TODO: Wire up existing tool definitions from src/tools/
        // For now, returning empty to let compilation succeed
        vec![]
    }

    /// Execute a tool by name
    pub async fn _call_tool(
        &self,
        tool_name: &str,
        tool_input: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<CallToolResult, ErrorData> {
        // TODO: Wire up existing tool execution from src/tools/
        // This bridges to the custom tool implementations
        let _ = tool_input;
        Err(ErrorData {
            code: ErrorCode(-32601),
            message: Cow::Owned(format!("Tool not implemented: {}", tool_name)),
            data: None,
        })
    }
}

/// Implement ServerHandler for rmcp integration
impl ServerHandler for MathService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "rust-math-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: Some("Rust Math MCP Server".to_string()),
                website_url: Some("https://github.com/justnorawr/RustMath".to_string()),
                icons: Some(vec![]),
            },
            instructions: Some(
                "A comprehensive mathematical operations server providing basic arithmetic, \
                 algebra, statistics, geometry, trigonometry, finance, and advanced mathematical \
                 functions."
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _params: Option<PaginatedRequestParam>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult {
            tools: self._list_tools(),
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        params: CallToolRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        self._call_tool(&params.name, params.arguments).await
    }
}
