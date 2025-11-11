/// JSON-RPC protocol version
pub const JSON_RPC_VERSION: &str = "2.0";

/// MCP protocol methods
pub mod methods {
    pub const INITIALIZE: &str = "initialize";
    pub const TOOLS_LIST: &str = "tools/list";
    pub const TOOLS_CALL: &str = "tools/call";
}

/// JSON-RPC error codes
pub mod error_codes {
    /// Parse error
    pub const PARSE_ERROR: i32 = -32700;
    /// Invalid Request
    pub const INVALID_REQUEST: i32 = -32600;
    /// Method not found
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid params
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal error
    pub const INTERNAL_ERROR: i32 = -32603;
    
    /// Server error range: -32000 to -32099
    /// Tool execution error
    pub const TOOL_ERROR: i32 = -32000;
    /// Validation error
    pub const VALIDATION_ERROR: i32 = -32001;
    /// Resource limit error
    pub const RESOURCE_LIMIT: i32 = -32002;
}

/// Server configuration constants
pub mod server {
    pub const DEFAULT_NAME: &str = "rust-math-mcp";
    pub const DEFAULT_VERSION: &str = "0.1.0";
}

