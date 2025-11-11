use crate::config::Config;
use crate::error::{McpError, McpResult};
use crate::utils::validation::validate_array_size;
use std::time::{Duration, Instant};

/// Resource limits and constraints manager.
///
/// Provides centralized management of resource limits including array sizes,
/// timeouts, and other constraints to prevent resource exhaustion.
pub struct Limits {
    config: Config,
}

impl Limits {
    /// Create a new Limits instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Validate array size against limits
    pub fn check_array_size(&self, size: usize) -> McpResult<()> {
        validate_array_size(size, &self.config)
    }

    /// Check if operation should timeout (placeholder for future async implementation)
    pub fn check_timeout(&self, start: Instant, max_duration: Duration) -> McpResult<()> {
        if start.elapsed() > max_duration {
            return Err(McpError::resource_limit(format!(
                "Operation exceeded timeout of {:?}",
                max_duration
            )));
        }
        Ok(())
    }

    /// Get maximum array size
    pub fn max_array_size(&self) -> usize {
        self.config.max_array_size
    }

    /// Get maximum decimal places
    pub fn max_decimal_places(&self) -> i32 {
        self.config.max_decimal_places
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

