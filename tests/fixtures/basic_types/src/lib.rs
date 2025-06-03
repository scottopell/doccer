//! Basic types fixture for testing doccer
//!
//! This crate contains simple Rust constructs to validate
//! basic parsing and rendering functionality.

/// A simple person struct
pub struct Person {
    /// The person's name
    pub name: String,
    /// The person's age in years
    pub age: u32,
}

impl Person {
    /// Creates a new person
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    /// Gets the person's name
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// Different types of vehicles
pub enum Vehicle {
    /// A car with number of doors
    Car(u8),
    /// A bicycle
    Bike,
    /// A truck with cargo capacity in tons
    Truck { capacity: f32 },
}

/// A simple constant
pub const MAX_USERS: usize = 1000;

/// Calculates the area of a rectangle
pub fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

/// A trait for things that can be named
pub trait Named {
    /// Returns the name
    fn name(&self) -> &str;
}

impl Named for Person {
    fn name(&self) -> &str {
        &self.name
    }
}
