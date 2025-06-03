//! Generics fixture for testing doccer
//!
//! This crate contains generic types, lifetimes, and constraints
//! to validate advanced parsing functionality.

use std::fmt::Display;

/// A generic container that holds a value
pub struct Container<T> {
    /// The contained value
    pub value: T,
}

impl<T> Container<T> {
    /// Creates a new container
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Gets a reference to the contained value
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Consumes the container and returns the value
    pub fn into_inner(self) -> T {
        self.value
    }
}

/// A generic pair of values
pub struct Pair<T, U> {
    /// The first value
    pub first: T,
    /// The second value
    pub second: U,
}

/// A trait for types that can be compared
pub trait Comparable<T> {
    /// Compare this value with another
    fn compare(&self, other: &T) -> std::cmp::Ordering;
}

/// A generic result type with constraints
pub struct Result<T, E>
where
    T: Clone,
    E: Display,
{
    value: Option<T>,
    error: Option<E>,
}

impl<T, E> Result<T, E>
where
    T: Clone,
    E: Display,
{
    /// Creates a successful result
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            error: None,
        }
    }

    /// Creates an error result
    pub fn err(error: E) -> Self {
        Self {
            value: None,
            error: Some(error),
        }
    }
}

/// A function with lifetime parameters
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// A struct with lifetime parameters
pub struct Reference<'a> {
    /// A reference to some data
    pub data: &'a str,
}

impl<'a> Reference<'a> {
    /// Creates a new reference
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }
}

/// Associated types example
pub trait Iterator {
    /// The type of items yielded by the iterator
    type Item;

    /// Get the next item
    fn next(&mut self) -> Option<Self::Item>;
}

/// Generic associated constants
pub trait Constants<T> {
    /// A default value
    const DEFAULT: T;
    /// Maximum value
    const MAX: T;
}
