//! Advanced error handling patterns demonstrating custom error types,
//! error chaining, Result operations, and complex error hierarchies.

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::num::ParseIntError;

/// A custom error type demonstrating basic Error trait implementation
#[derive(Debug)]
pub struct CustomError {
    pub message: String,
    pub code: i32,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom error [{}]: {}", self.code, self.message)
    }
}

impl Error for CustomError {}

/// An error type that wraps other errors, demonstrating error chaining
#[derive(Debug)]
pub enum ChainedError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(CustomError),
    Network { url: String, status: u16 },
}

impl fmt::Display for ChainedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainedError::Io(err) => write!(f, "IO error: {}", err),
            ChainedError::Parse(err) => write!(f, "Parse error: {}", err),
            ChainedError::Custom(err) => write!(f, "Custom error: {}", err),
            ChainedError::Network { url, status } => {
                write!(f, "Network error: {} returned status {}", url, status)
            }
        }
    }
}

impl Error for ChainedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChainedError::Io(err) => Some(err),
            ChainedError::Parse(err) => Some(err),
            ChainedError::Custom(err) => Some(err),
            ChainedError::Network { .. } => None,
        }
    }
}

/// Demonstrates From trait implementations for error conversion
impl From<io::Error> for ChainedError {
    fn from(err: io::Error) -> Self {
        ChainedError::Io(err)
    }
}

impl From<ParseIntError> for ChainedError {
    fn from(err: ParseIntError) -> Self {
        ChainedError::Parse(err)
    }
}

impl From<CustomError> for ChainedError {
    fn from(err: CustomError) -> Self {
        ChainedError::Custom(err)
    }
}

/// A function that demonstrates the ? operator with multiple error types
pub fn complex_operation(filename: &str, number_str: &str) -> Result<i32, ChainedError> {
    let _file = File::open(filename)?; // io::Error -> ChainedError
    let number: i32 = number_str.parse()?; // ParseIntError -> ChainedError
    
    if number < 0 {
        return Err(CustomError {
            message: "Number must be positive".to_string(),
            code: 400,
        })?; // CustomError -> ChainedError
    }
    
    Ok(number * 2)
}

/// Demonstrates Result chaining with map, and_then, or_else
pub fn result_chaining_example(input: &str) -> Result<String, ChainedError> {
    input
        .parse::<i32>()
        .map_err(ChainedError::from)
        .and_then(|num| {
            if num > 100 {
                Err(ChainedError::Custom(CustomError {
                    message: "Number too large".to_string(),
                    code: 413,
                }))
            } else {
                Ok(num)
            }
        })
        .map(|num| format!("Processed: {}", num))
        .or_else(|err| {
            match err {
                ChainedError::Parse(_) => Ok("Default value".to_string()),
                other => Err(other),
            }
        })
}

/// A more complex error hierarchy with associated types
pub trait ProcessingError: Error + Send + Sync + 'static {
    type Context;
    
    fn context(&self) -> &Self::Context;
    fn severity(&self) -> ErrorSeverity;
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// A context-aware error type
#[derive(Debug)]
pub struct ContextualError<T> {
    pub inner: Box<dyn Error + Send + Sync>,
    pub context: T,
    pub severity: ErrorSeverity,
}

impl<T> fmt::Display for ContextualError<T> 
where 
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Contextual error: {} (context: {:?})", self.inner, self.context)
    }
}

impl<T> Error for ContextualError<T> 
where 
    T: fmt::Debug + Send + Sync + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&**self.inner)
    }
}

impl<T> ProcessingError for ContextualError<T> 
where 
    T: fmt::Debug + Send + Sync + 'static,
{
    type Context = T;
    
    fn context(&self) -> &Self::Context {
        &self.context
    }
    
    fn severity(&self) -> ErrorSeverity {
        self.severity
    }
}

/// A trait for error recovery strategies
pub trait ErrorRecovery<E> {
    type Output;
    
    fn recover(self, error: E) -> Self::Output;
}

/// Demonstrates error recovery patterns
pub struct RetryStrategy {
    pub max_attempts: u32,
    pub delay_ms: u64,
}

impl ErrorRecovery<io::Error> for RetryStrategy {
    type Output = Result<(), io::Error>;
    
    fn recover(self, error: io::Error) -> Self::Output {
        match error.kind() {
            io::ErrorKind::TimedOut | io::ErrorKind::Interrupted => {
                // In a real implementation, this would retry
                Ok(())
            }
            _ => Err(error),
        }
    }
}

/// Helper function for creating contextual errors
pub fn with_context<T, E>(
    result: Result<T, E>,
    context: String,
    severity: ErrorSeverity,
) -> Result<T, ContextualError<String>>
where
    E: Error + Send + Sync + 'static,
{
    result.map_err(|err| ContextualError {
        inner: Box::new(err),
        context,
        severity,
    })
}

/// Demonstrates error aggregation patterns
pub fn aggregate_errors(operations: Vec<fn() -> Result<i32, ChainedError>>) -> Result<Vec<i32>, Vec<ChainedError>> {
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for op in operations {
        match op() {
            Ok(result) => results.push(result),
            Err(err) => errors.push(err),
        }
    }
    
    if errors.is_empty() {
        Ok(results)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_error() {
        let err = CustomError {
            message: "Test error".to_string(),
            code: 500,
        };
        assert_eq!(err.to_string(), "Custom error [500]: Test error");
    }
    
    #[test]
    fn test_error_chaining() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let chained = ChainedError::from(io_err);
        
        assert!(chained.source().is_some());
        assert!(chained.to_string().contains("IO error"));
    }
    
    #[test]
    fn test_result_chaining() {
        let result = result_chaining_example("50");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Processed: 50");
        
        let result = result_chaining_example("not_a_number");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Default value");
    }
}