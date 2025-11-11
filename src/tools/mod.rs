pub mod advanced;
pub mod algebra;
pub mod basic_math;
pub mod combinatorics;
pub mod equations;
pub mod finance;
pub mod geometry;
pub mod registry;
pub mod statistics;
pub mod traits;
pub mod trigonometry;

// Re-export for convenience
pub use registry::{execute_tool, get_all_tools, DefaultToolRegistry};
pub use traits::ToolRegistry;

