use crate::config::Config;
use crate::error::{McpError, McpResult};

/// Validate array size against configuration limits.
///
/// # Arguments
///
/// * `size` - The array size to validate
/// * `config` - Server configuration containing limits
///
/// # Returns
///
/// `Ok(())` if size is within limits, error otherwise.
pub fn validate_array_size(size: usize, config: &Config) -> McpResult<()> {
    if size > config.max_array_size {
        return Err(McpError::resource_limit(format!(
            "Array size {} exceeds maximum allowed size of {}",
            size, config.max_array_size
        )));
    }
    Ok(())
}

/// Validate a number is finite (not NaN or Infinity).
///
/// # Arguments
///
/// * `value` - The number to validate
/// * `name` - Name of the value for error messages
pub fn validate_finite(value: f64, name: &str) -> McpResult<()> {
    if !value.is_finite() {
        return Err(McpError::validation_error(format!(
            "{} must be a finite number, got: {}",
            name, value
        )));
    }
    Ok(())
}

/// Validate decimal places for rounding operations
pub fn validate_decimal_places(places: i32, config: &Config) -> McpResult<()> {
    if places < 0 {
        return Err(McpError::validation_error(
            "Decimal places must be non-negative",
        ));
    }
    if places > config.max_decimal_places {
        return Err(McpError::validation_error(format!(
            "Decimal places {} exceeds maximum of {}",
            places, config.max_decimal_places
        )));
    }
    Ok(())
}

/// Validate that a number is positive
pub fn validate_positive(value: f64, name: &str) -> McpResult<()> {
    if value <= 0.0 {
        return Err(McpError::validation_error(format!(
            "{} must be positive, got: {}",
            name, value
        )));
    }
    Ok(())
}

/// Validate that a number is non-negative
pub fn validate_non_negative(value: f64, name: &str) -> McpResult<()> {
    if value < 0.0 {
        return Err(McpError::validation_error(format!(
            "{} must be non-negative, got: {}",
            name, value
        )));
    }
    Ok(())
}

/// Validate integer value (within reasonable range)
pub fn validate_integer(value: f64, name: &str) -> McpResult<i64> {
    if value.fract() != 0.0 {
        return Err(McpError::validation_error(format!(
            "{} must be an integer, got: {}",
            name, value
        )));
    }

    // Check if value is within i64 range
    if value < i64::MIN as f64 || value > i64::MAX as f64 {
        return Err(McpError::validation_error(format!(
            "{} is out of range for integer operations",
            name
        )));
    }

    Ok(value as i64)
}
