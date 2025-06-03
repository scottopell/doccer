//! Modules fixture for testing doccer
//!
//! This crate contains nested modules with different visibility
//! patterns to validate hierarchical structure parsing.

/// Public utilities module
pub mod utils {
    /// A public utility function
    pub fn helper() -> String {
        "helper".to_string()
    }

    /// A crate-local utility
    pub(crate) fn internal_helper() -> i32 {
        42
    }

    /// Private function (not exported)
    fn private_helper() {
        // This should not appear in docs
    }

    /// Nested utilities module
    pub mod nested {
        /// A deeply nested function
        pub fn deep_function() -> bool {
            true
        }

        /// Crate-visible nested function
        pub(crate) fn crate_function() {
            // Available to the crate
        }

        /// Super-visible function
        pub(super) fn parent_function() {
            // Available to parent module
        }
    }

    /// A private nested module
    mod private_nested {
        pub fn hidden_function() {
            // This module is private, so this won't be visible
        }
    }
}

/// Network-related functionality
pub mod network {
    /// A connection struct
    pub struct Connection {
        /// The host address
        pub host: String,
        /// The port number (crate-visible)
        pub(crate) port: u16,
        /// Private connection state
        state: ConnectionState,
    }

    /// Connection state enumeration
    enum ConnectionState {
        Connected,
        Disconnected,
        Connecting,
    }

    impl Connection {
        /// Creates a new connection
        pub fn new(host: String, port: u16) -> Self {
            Self {
                host,
                port,
                state: ConnectionState::Disconnected,
            }
        }

        /// Gets the host
        pub fn host(&self) -> &str {
            &self.host
        }

        /// Gets the port (crate-visible)
        pub(crate) fn port(&self) -> u16 {
            self.port
        }

        /// Private connection method
        fn connect_internal(&mut self) {
            self.state = ConnectionState::Connecting;
        }
    }

    /// Protocol submodule
    pub mod protocol {
        /// HTTP-specific functionality
        pub mod http {
            /// HTTP methods
            pub enum Method {
                Get,
                Post,
                Put,
                Delete,
            }

            /// HTTP request structure
            pub struct Request {
                /// The HTTP method
                pub method: Method,
                /// The request path
                pub path: String,
            }
        }

        /// TCP-specific functionality
        pub mod tcp {
            /// TCP socket options
            pub struct Options {
                /// Keep-alive setting
                pub keep_alive: bool,
                /// Timeout in seconds
                pub(super) timeout: u32,
            }
        }
    }
}

/// Database module (crate-visible)
pub(crate) mod database {
    /// Database connection
    pub struct DbConnection {
        /// Connection string
        pub url: String,
    }

    /// Database operations
    pub mod ops {
        /// Query function
        pub fn query(sql: &str) -> Vec<String> {
            vec![sql.to_string()]
        }
    }
}
