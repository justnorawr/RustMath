# RustMath MCP Server

A high-performance Model Context Protocol (MCP) server implementation in Rust for comprehensive mathematical operations.

## Overview

This is a production-ready MCP server that provides a comprehensive suite of mathematical tools organized into modular categories. The server implements the MCP protocol with proper error handling, input validation, and efficient tool dispatching.

## Features

- **MCP Protocol**: Full JSON-RPC 2.0 implementation with proper error handling
- **Tool Registry**: O(1) HashMap-based tool lookup for optimal performance
- **Input Validation**: Automatic validation of all inputs with configurable limits
- **Error Handling**: Structured error types with proper JSON-RPC error codes
- **Security Hardened**:
  - Rate limiting enabled by default (1000 req/s)
  - Content-Length limits (10MB max) prevent memory exhaustion
  - Checked arithmetic prevents integer overflow
  - Input sanitization prevents log injection
  - Mutex poison recovery for resilience

## Available Tools

### Basic Math Operations (11 tools)
- **add**: Add two or more numbers together
- **subtract**: Subtract numbers
- **multiply**: Multiply two or more numbers together
- **divide**: Divide two numbers
- **power**: Raise a number to a power
- **sqrt**: Calculate the square root of a number
- **abs**: Get the absolute value of a number
- **round**: Round a number to the nearest integer (or specified decimal places)
- **floor**: Round down to the nearest integer
- **ceil**: Round up to the nearest integer
- **modulo**: Calculate the remainder of division

### Algebraic Operations (3 tools)
- **gcd**: Calculate the greatest common divisor of two numbers
- **lcm**: Calculate the least common multiple of two numbers
- **factorial**: Calculate the factorial of a non-negative integer

### Statistical Operations (9 tools)
- **mean**: Calculate the arithmetic mean (average) of a list of numbers
- **median**: Calculate the median of a list of numbers
- **mode**: Find the mode (most frequently occurring value) of a list of numbers
- **std_dev**: Calculate the standard deviation (supports sample and population)
- **variance**: Calculate the variance (supports sample and population)
- **min**: Find the minimum value in a list of numbers
- **max**: Find the maximum value in a list of numbers
- **sum**: Calculate the sum of a list of numbers
- **product**: Calculate the product of a list of numbers

### Geometry (8 tools)
- **area_circle**: Calculate the area of a circle
- **area_rectangle**: Calculate the area of a rectangle
- **area_triangle**: Calculate the area of a triangle
- **area_trapezoid**: Calculate the area of a trapezoid
- **volume_sphere**: Calculate the volume of a sphere
- **volume_cylinder**: Calculate the volume of a cylinder
- **volume_cone**: Calculate the volume of a cone
- **volume_rectangular_prism**: Calculate the volume of a rectangular prism

### Equations (5 tools)
- **quadratic_formula**: Solve quadratic equation ax² + bx + c = 0
- **distance_formula**: Calculate distance between two points
- **pythagorean_theorem**: Calculate the third side of a right triangle
- **slope**: Calculate the slope of a line between two points
- **midpoint**: Calculate the midpoint between two points

### Trigonometry (10 tools)
- **sin, cos, tan**: Basic trigonometric functions (radians)
- **asin, acos, atan**: Inverse trigonometric functions
- **law_of_cosines**: Calculate side or angle using Law of Cosines
- **law_of_sines**: Calculate side or angle using Law of Sines
- **degrees_to_radians**: Convert degrees to radians
- **radians_to_degrees**: Convert radians to degrees

### Finance (3 tools)
- **compound_interest**: Calculate compound interest
- **simple_interest**: Calculate simple interest
- **percentage**: Calculate percentages or find parts/wholes

### Combinatorics (2 tools)
- **permutation**: Calculate permutations P(n, r)
- **combination**: Calculate combinations C(n, r)

### Advanced (2 tools)
- **exponential_growth**: Calculate exponential growth (continuous or discrete)
- **logarithm**: Calculate logarithms (natural, common, or custom base)

### Batch Operations (1 tool)
- **batch_operations**: Execute multiple math operations in a single call
  - Batch up to 50 operations at once
  - Each operation has a unique ID for result matching
  - Operations execute independently (failures don't stop others)
  - Returns summary with success/failure counts

**Total: 54 mathematical tools**

## Requirements

- Rust 1.70+ (edition 2021)
- Cargo

## Quick Start with Claude Desktop

### 1. Build the Server

```bash
cargo build --release
```

The binary will be created at `target/release/rust-math-mcp`.

### 2. Configure Claude Desktop

Add the server to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "rust-math": {
      "command": "/absolute/path/to/rust-math-mcp/target/release/rust-math-mcp"
    }
  }
}
```

**Important**: Replace `/absolute/path/to/rust-math-mcp` with the actual absolute path to your project directory.

Example:
```json
{
  "mcpServers": {
    "rust-math": {
      "command": "/Users/yourname/projects/rust-math-mcp/target/release/rust-math-mcp"
    }
  }
}
```

### Advanced Configuration

You can customize the server behavior by passing environment variables:

```json
{
  "mcpServers": {
    "rust-math": {
      "command": "/Users/yourname/projects/rust-math-mcp/target/release/rust-math-mcp",
      "env": {
        "RUST_LOG": "rust_math_mcp=debug",
        "MCP_MAX_REQUESTS_PER_SECOND": "500",
        "MCP_MAX_ARRAY_SIZE": "5000"
      }
    }
  }
}
```

Available environment variables:
- `RUST_LOG`: Logging level (`error`, `warn`, `info`, `debug`, `trace`)
- `MCP_MAX_REQUESTS_PER_SECOND`: Rate limit (default: 1000)
- `MCP_MAX_ARRAY_SIZE`: Max array size for inputs (default: 10000)
- `MCP_MAX_DECIMAL_PLACES`: Max decimal precision (default: 15)
- `MCP_ENABLE_RATE_LIMIT`: Enable/disable rate limiting (default: true)

### 3. Restart Claude Desktop

Completely quit Claude Desktop (⌘Q on macOS) and reopen it. The rust-math MCP server will automatically start when Claude Desktop launches.

### 4. Verify Installation

In Claude Desktop, you can verify the server is running by asking:
> "What math tools do you have available?"

Claude should respond with a list of the 54 available mathematical tools.

## Usage Examples

### Basic Calculations

```
You: Calculate 15% of 250
Claude: [Uses percentage tool] → 37.5

You: What's the square root of 144?
Claude: [Uses sqrt tool] → 12

You: Find the GCD of 48 and 18
Claude: [Uses gcd tool] → 6
```

### Statistical Analysis

```
You: Calculate the mean, median, and standard deviation of [23, 45, 67, 12, 89, 34, 56]
Claude: [Uses mean, median, std_dev tools]
- Mean: 46.57
- Median: 45
- Standard Deviation: 25.89
```

### Batch Operations

Perform multiple calculations in a single call for better performance:

```
You: I need to calculate the following:
1. Sum of 10, 20, 30
2. Mean of 100, 200, 300
3. Square root of 144
4. Area of a circle with radius 5

Claude: [Uses batch_operations tool]
Results:
1. Sum: 60
2. Mean: 200
3. Square root: 12
4. Circle area: 78.54
```

### Complex Problems

```
You: Solve the quadratic equation: 2x² - 7x + 3 = 0
Claude: [Uses quadratic_formula tool]
The solutions are x = 3 and x = 0.5

You: Calculate compound interest on $10,000 at 5% annual rate for 10 years, compounded monthly
Claude: [Uses compound_interest tool]
Final amount: $16,470.09
Interest earned: $6,470.09
```

## Running Manually

The MCP server communicates via stdin/stdout using JSON-RPC 2.0:

```bash
cargo run
```

Set log level with environment variable:
```bash
RUST_LOG=rust_math_mcp=debug cargo run
```

## Configuration

The server can be configured via environment variables:

- `MCP_SERVER_NAME`: Server name (default: "rust-math-mcp")
- `MCP_SERVER_VERSION`: Server version (default: "0.1.0")
- `MCP_MAX_ARRAY_SIZE`: Maximum array size for tool inputs (default: 10000)
- `MCP_MAX_DECIMAL_PLACES`: Maximum decimal places for rounding (default: 15)
- `MCP_ENABLE_RATE_LIMIT`: Enable rate limiting (default: true)
- `MCP_MAX_REQUESTS_PER_SECOND`: Maximum requests per second when rate limiting enabled (default: 1000)
- `RUST_LOG`: Logging level (default: "rust_math_mcp=info")

### Rate Limiting

Rate limiting is **enabled by default** for security. To disable or adjust:

```bash
# Disable rate limiting (not recommended for production)
MCP_ENABLE_RATE_LIMIT=false cargo run

# Adjust rate limit (default: 1000 req/s)
MCP_MAX_REQUESTS_PER_SECOND=100 cargo run
```

Rate limiting uses a token bucket algorithm to prevent DoS attacks while allowing bursts of legitimate traffic.

## Troubleshooting

### Server Not Showing in Claude Desktop

1. **Check configuration file path**: Ensure the config file exists at the correct location for your OS
2. **Verify binary path**: The `command` must be an absolute path, not relative
3. **Check binary exists**: Run `ls -la /path/to/rust-math-mcp` to verify the file exists
4. **Restart Claude Desktop**: Completely quit (⌘Q on macOS) and reopen

### Server Errors in Claude Desktop

Check the logs for detailed error messages:
- **macOS**: `~/Library/Logs/Claude/mcp-server-rust-math.log`
- **Windows**: `%APPDATA%\Claude\logs\mcp-server-rust-math.log`
- **Linux**: `~/.local/state/Claude/logs/mcp-server-rust-math.log`

### Common Issues

**"Server disconnected" or timeout errors**:
- Make sure you've built the latest version: `cargo build --release`
- Copy the new binary to your configured location
- Restart Claude Desktop

**"Command not found"**:
- Check that the path in your config is absolute (starts with `/` on Unix or `C:\` on Windows)
- Verify the binary has execute permissions: `chmod +x target/release/rust-math-mcp`

**Tools not appearing**:
- Wait a few seconds after Claude Desktop starts for the server to initialize
- Ask Claude "What tools do you have?" to trigger tool discovery
- Check the logs for initialization errors

### Security Features

- **Memory Protection**: Content-Length capped at 10MB to prevent memory exhaustion attacks
- **Overflow Protection**: Checked arithmetic on all combinatorial operations (factorial, permutation, combination)
- **Input Sanitization**: Error messages sanitized to prevent log injection
- **Resilience**: Mutex poison recovery ensures cascading failures don't occur

## Development

```bash
# Run in debug mode
cargo run

# Run all tests
cargo test

# Run with verbose output
RUST_LOG=rust_math_mcp=debug cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Project Structure

```
src/
├── main.rs              # Entry point and main loop
├── lib.rs               # Library root
├── error.rs             # Custom error types
├── config.rs            # Configuration management
├── protocol/            # MCP protocol implementation
│   ├── mod.rs          # Protocol types and handlers
│   ├── parser.rs       # Message parsing
│   └── constants.rs    # Protocol constants
├── tools/               # Tool implementations
│   ├── mod.rs          # Tool registry
│   ├── registry.rs     # HashMap-based tool registry
│   ├── traits.rs       # ToolRegistry trait
│   └── [category].rs   # Tool modules by category
└── lib/                 # Shared utilities
    ├── args.rs         # Argument parsing
    ├── validation.rs   # Input validation
    └── limits.rs        # Resource limits
```

## License

MIT License - see LICENSE file for details.
