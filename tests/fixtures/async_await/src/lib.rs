//! # Async/Await Test Fixture
//!
//! This fixture demonstrates various async/await patterns in Rust,
//! including async functions, trait methods, and complex Future types.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Simple async function that returns a future
///
/// This demonstrates the basic async fn syntax and how it appears
/// in documentation.
pub async fn simple_async_function() -> Result<String, Box<dyn std::error::Error>> {
    Ok("Hello, async world!".to_string())
}

/// Async function with parameters and complex return type
///
/// Shows how async functions with multiple parameters and complex
/// return types are documented.
pub async fn complex_async_function(
    input: &str,
    timeout: u64,
) -> Result<Vec<String>, std::io::Error> {
    tokio::time::sleep(tokio::time::Duration::from_millis(timeout)).await;
    Ok(vec![input.to_string()])
}

/// Async function returning a boxed future
///
/// This pattern is common when you need to return different future types
/// from the same function.
pub fn boxed_future_function() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async { 42 })
}

/// Async trait with various method types
///
/// Demonstrates async trait methods and their documentation.
pub trait AsyncTrait {
    /// Async method with default implementation
    async fn async_method(&self) -> String {
        "default implementation".to_string()
    }

    /// Async method without default implementation
    async fn required_async_method(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Method returning a boxed future
    fn future_method(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>>;
}

/// Struct implementing async trait
///
/// Shows how async trait implementations are documented.
pub struct AsyncStruct {
    pub value: i32,
}

impl AsyncTrait for AsyncStruct {
    async fn required_async_method(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.value > 0 {
            Ok(())
        } else {
            Err("Value must be positive".into())
        }
    }

    fn future_method(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async move { self.value > 10 })
    }
}

/// Custom Future implementation
///
/// Demonstrates manual Future implementation and how it's documented.
pub struct CustomFuture {
    completed: bool,
}

impl CustomFuture {
    pub fn new() -> Self {
        Self { completed: false }
    }
}

impl Future for CustomFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready("Future completed".to_string())
        } else {
            self.completed = true;
            Poll::Pending
        }
    }
}

/// Async function with Send + Sync bounds
///
/// Shows how async functions with trait bounds are documented.
pub async fn bounded_async_function<T>() -> T
where
    T: Send + Sync + Default,
{
    T::default()
}

/// Async closure type alias
///
/// Demonstrates complex async closure types in documentation.
pub type AsyncClosure<T> = Box<dyn Fn() -> Pin<Box<dyn Future<Output = T> + Send>> + Send + Sync>;

/// Function that takes an async closure
///
/// Shows how functions accepting async closures are documented.
pub async fn use_async_closure<T>(closure: AsyncClosure<T>) -> T {
    closure().await
}

/// Async generator-like function
///
/// Demonstrates async functions that yield multiple values over time.
pub async fn async_generator(count: usize) -> Vec<i32> {
    let mut results = Vec::new();
    for i in 0..count {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        results.push(i as i32);
    }
    results
}

/// Stream-like async iterator
///
/// Shows how async stream patterns are documented.
pub struct AsyncIterator {
    current: usize,
    max: usize,
}

impl AsyncIterator {
    pub fn new(max: usize) -> Self {
        Self { current: 0, max }
    }

    pub async fn next(&mut self) -> Option<usize> {
        if self.current < self.max {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_async_function() {
        let result = simple_async_function().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_custom_future() {
        let future = CustomFuture::new();
        let result = future.await;
        assert_eq!(result, "Future completed");
    }
}