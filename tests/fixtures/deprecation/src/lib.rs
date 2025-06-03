//! Deprecation fixture for testing doccer
//!
//! This crate contains deprecated items to validate
//! deprecation notice rendering functionality.

/// A simple struct with deprecated field
pub struct Config {
    /// This field is used for the API key
    pub api_key: String,
    
    /// This field is no longer used
    #[deprecated(since = "1.2.0", note = "Use `timeout_ms` instead")]
    pub timeout: u32,
    
    /// Timeout in milliseconds
    pub timeout_ms: u32,
}

impl Config {
    /// Creates a new config
    pub fn new(api_key: String, timeout_ms: u32) -> Self {
        Self {
            api_key,
            timeout: timeout_ms / 1000, // For backward compatibility
            timeout_ms,
        }
    }
    
    /// Old method for setting timeout in seconds
    #[deprecated(since = "1.1.0", note = "Use `set_timeout_ms` instead")]
    pub fn set_timeout(&mut self, seconds: u32) {
        self.timeout = seconds;
        self.timeout_ms = seconds * 1000;
    }
    
    /// Sets the timeout in milliseconds
    pub fn set_timeout_ms(&mut self, ms: u32) {
        self.timeout_ms = ms;
        self.timeout = ms / 1000;
    }
}

/// A deprecated enum that should be replaced
#[deprecated(since = "1.3.0", note = "Use `HttpStatus` enum instead")]
pub enum Status {
    /// Everything is fine
    Ok,
    /// Something went wrong
    Error,
}

/// HTTP status codes
pub enum HttpStatus {
    /// 200 OK
    Ok,
    /// 400 Bad Request
    BadRequest,
    /// 404 Not Found
    NotFound,
    /// 500 Internal Server Error
    InternalError,
}

/// A trait for handling deprecation
pub trait Handler {
    /// Process a request
    fn process(&self) -> Result<(), String>;
    
    /// Old way of handling errors
    #[deprecated(since = "1.2.5")]
    fn handle_error(&self, error: &str);
}

/// Implementation of Handler
pub struct DefaultHandler;

impl Handler for DefaultHandler {
    fn process(&self) -> Result<(), String> {
        Ok(())
    }
    
    #[deprecated(since = "1.2.5")]
    fn handle_error(&self, _error: &str) {
        // Deprecated implementation
    }
}

/// A function that is no longer recommended
#[deprecated(since = "1.0.0", note = "Use `new_connect` instead")]
pub fn connect(host: &str, port: u16) -> bool {
    // Old connection logic
    !host.is_empty() && port > 0
}

/// New connection function
pub fn new_connect(host: &str, port: u16) -> Result<(), String> {
    if host.is_empty() {
        return Err("Empty hostname".to_string());
    }
    if port == 0 {
        return Err("Invalid port".to_string());
    }
    Ok(())
}