use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Simple token bucket rate limiter.
///
/// This implementation uses a token bucket algorithm to limit the rate of operations.
/// Tokens are added at a fixed rate, and each operation consumes one token.
///
/// # Example
///
/// ```rust
/// use rust_math_mcp::utils::rate_limiter::RateLimiter;
/// use std::time::Duration;
///
/// let limiter = RateLimiter::new(10, Duration::from_secs(1)); // 10 requests per second
/// assert!(limiter.check_rate_limit()); // First request succeeds
/// ```
pub struct RateLimiter {
    state: Arc<Mutex<RateLimiterState>>,
}

struct RateLimiterState {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - Maximum number of tokens (burst capacity)
    /// * `refill_interval` - Time interval for refilling all tokens
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_math_mcp::utils::rate_limiter::RateLimiter;
    /// use std::time::Duration;
    ///
    /// // Allow 100 requests per second with burst of 100
    /// let limiter = RateLimiter::new(100, Duration::from_secs(1));
    /// ```
    pub fn new(max_tokens: usize, refill_interval: Duration) -> Self {
        let refill_rate = max_tokens as f64 / refill_interval.as_secs_f64();

        Self {
            state: Arc::new(Mutex::new(RateLimiterState {
                tokens: max_tokens as f64,
                max_tokens: max_tokens as f64,
                refill_rate,
                last_refill: Instant::now(),
            })),
        }
    }

    /// Check if an operation is allowed under the rate limit.
    ///
    /// Returns `true` if the operation should be allowed (token available),
    /// `false` if the rate limit has been exceeded.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rust_math_mcp::utils::rate_limiter::RateLimiter;
    /// use std::time::Duration;
    ///
    /// let limiter = RateLimiter::new(2, Duration::from_secs(1));
    ///
    /// assert!(limiter.check_rate_limit()); // 1st request: OK
    /// assert!(limiter.check_rate_limit()); // 2nd request: OK
    /// assert!(!limiter.check_rate_limit()); // 3rd request: rate limited
    /// ```
    pub fn check_rate_limit(&self) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| {
            // Recover from poisoned mutex by taking ownership of the inner data
            // This prevents cascading failures if a thread panics while holding the lock
            poisoned.into_inner()
        });

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_refill).as_secs_f64();
        let tokens_to_add = elapsed * state.refill_rate;

        state.tokens = (state.tokens + tokens_to_add).min(state.max_tokens);
        state.last_refill = now;

        // Check if we have a token available
        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Get the current number of available tokens.
    ///
    /// Useful for monitoring and debugging.
    pub fn available_tokens(&self) -> f64 {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_refill).as_secs_f64();
        let tokens_to_add = elapsed * state.refill_rate;

        state.tokens = (state.tokens + tokens_to_add).min(state.max_tokens);
        state.last_refill = now;

        state.tokens
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new(2, Duration::from_secs(1));

        assert!(limiter.check_rate_limit()); // 1st request: OK
        assert!(limiter.check_rate_limit()); // 2nd request: OK
        assert!(!limiter.check_rate_limit()); // 3rd request: rate limited
    }

    #[test]
    fn test_rate_limiter_refill() {
        let limiter = RateLimiter::new(1, Duration::from_millis(100));

        assert!(limiter.check_rate_limit()); // 1st request: OK
        assert!(!limiter.check_rate_limit()); // 2nd request: rate limited

        // Wait for refill
        thread::sleep(Duration::from_millis(150));

        assert!(limiter.check_rate_limit()); // Should be OK after refill
    }

    #[test]
    fn test_rate_limiter_available_tokens() {
        let limiter = RateLimiter::new(5, Duration::from_secs(1));

        assert_eq!(limiter.available_tokens().floor(), 5.0);

        limiter.check_rate_limit();
        assert_eq!(limiter.available_tokens().floor(), 4.0);

        limiter.check_rate_limit();
        limiter.check_rate_limit();
        assert_eq!(limiter.available_tokens().floor(), 2.0);
    }
}
