//! Complex fixture for testing doccer
//!
//! This crate combines all Rust features to provide a comprehensive
//! test of doccer's parsing and rendering capabilities.
//!
//! # Examples
//!
//! ```rust
//! use complex::storage::*;
//! let mut storage = Storage::new();
//! storage.insert("key", "value");
//! ```

use std::collections::HashMap;
use std::fmt::Debug;

/// A macro for creating formatted messages
///
/// # Examples
///
/// ```
/// let msg = format_message!("Hello", "World");
/// assert_eq!(msg, "Hello: World");
/// ```
#[macro_export]
macro_rules! format_message {
    ($prefix:expr, $content:expr) => {
        format!("{}: {}", $prefix, $content)
    };
}

/// Storage and data management
pub mod storage {
    use super::*;

    /// A generic storage container with complex constraints
    pub struct Storage<K, V>
    where
        K: Clone + Debug + PartialEq + Eq + std::hash::Hash,
        V: Clone + Debug,
    {
        /// Internal data storage
        data: HashMap<K, V>,
        /// Maximum capacity
        pub capacity: usize,
        /// Current version
        version: u64,
    }

    impl<K, V> Storage<K, V>
    where
        K: Clone + Debug + PartialEq + Eq + std::hash::Hash,
        V: Clone + Debug,
    {
        /// Creates a new storage with default capacity
        ///
        /// # Examples
        ///
        /// ```
        /// let storage: Storage<String, i32> = Storage::new();
        /// ```
        pub fn new() -> Self {
            Self::with_capacity(100)
        }

        /// Creates a new storage with specified capacity
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                data: HashMap::new(),
                capacity,
                version: 0,
            }
        }

        /// Inserts a key-value pair
        pub fn insert(&mut self, key: K, value: V) -> Option<V> {
            if self.data.len() >= self.capacity {
                return None;
            }
            self.version += 1;
            self.data.insert(key, value)
        }

        /// Gets a value by key
        pub fn get(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }
    }

    /// A trait for cacheable items
    pub trait Cacheable<K>
    where
        K: Clone,
    {
        /// The cache key type
        type Key: Clone + Debug + std::hash::Hash + Eq;

        /// Gets the cache key for this item
        fn cache_key(&self) -> Self::Key;

        /// Validates if the item can be cached
        fn can_cache(&self) -> bool {
            true
        }
    }

    /// Cache implementation with lifetime parameters
    pub struct Cache<'a, T: Cacheable<String>> {
        /// Reference to the original data
        pub data: &'a [T],
        /// Cache storage
        cache: HashMap<T::Key, &'a T>,
    }

    impl<'a, T: Cacheable<String>> Cache<'a, T> {
        /// Creates a new cache
        pub fn new(data: &'a [T]) -> Self {
            let mut cache = HashMap::new();
            for item in data {
                if item.can_cache() {
                    cache.insert(item.cache_key(), item);
                }
            }
            Self { data, cache }
        }
    }

    /// Specialized cache for strings
    pub mod string_cache {
        use super::*;

        /// A string cache with complex operations
        pub struct StringCache {
            data: HashMap<String, CachedString>,
        }

        /// A cached string with metadata
        #[derive(Clone, Debug)]
        pub struct CachedString {
            /// The string content
            pub content: String,
            /// Access count
            pub(crate) access_count: u32,
            /// Whether it's compressed
            compressed: bool,
        }

        impl StringCache {
            /// Creates a new string cache
            pub fn new() -> Self {
                Self {
                    data: HashMap::new(),
                }
            }

            /// Adds a string to the cache
            pub fn add(&mut self, key: String, content: String) {
                let cached = CachedString {
                    content,
                    access_count: 0,
                    compressed: false,
                };
                self.data.insert(key, cached);
            }
        }
    }
}

/// Network operations and protocols
pub mod network {
    /// Protocol definitions
    pub mod protocol {
        /// A generic protocol handler
        pub trait Protocol<Req, Resp> {
            /// The error type for this protocol
            type Error: std::error::Error;

            /// Processes a request
            fn handle(&mut self, request: Req) -> Result<Resp, Self::Error>;
        }

        /// HTTP protocol implementation
        pub struct Http;

        /// HTTP request
        pub struct HttpRequest {
            /// Request method
            pub method: String,
            /// Request path
            pub path: String,
            /// Request headers
            pub headers: std::collections::HashMap<String, String>,
        }

        /// HTTP response
        pub struct HttpResponse {
            /// Response status code
            pub status: u16,
            /// Response body
            pub body: String,
        }

        /// HTTP error
        #[derive(Debug)]
        pub struct HttpError {
            /// Error message
            pub message: String,
        }

        impl std::fmt::Display for HttpError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "HTTP Error: {}", self.message)
            }
        }

        impl std::error::Error for HttpError {}

        impl Protocol<HttpRequest, HttpResponse> for Http {
            type Error = HttpError;

            fn handle(&mut self, request: HttpRequest) -> Result<HttpResponse, Self::Error> {
                Ok(HttpResponse {
                    status: 200,
                    body: format!("Handled {} {}", request.method, request.path),
                })
            }
        }
    }
}

/// Mathematical operations and utilities
pub mod math {
    /// A point in 2D space
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Point<T: Copy> {
        /// X coordinate
        pub x: T,
        /// Y coordinate
        pub y: T,
    }

    impl<T: Copy + std::ops::Add<Output = T>> Point<T> {
        /// Creates a new point
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }

        /// Adds two points together
        pub fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    /// Constants for mathematical operations
    pub mod constants {
        /// Mathematical constant π
        pub const PI: f64 = 3.14159265359;
        /// Mathematical constant e
        pub const E: f64 = 2.71828182846;
        /// Golden ratio
        pub const PHI: f64 = 1.61803398875;
    }
}

pub use math::Point;
pub use network::protocol::{Http, Protocol};
/// Re-exports for convenience
pub use storage::Storage;
