//! # Doccer Test Library
//!
//! This is a test library to demonstrate rustdoc JSON output parsing.

/// A simple structure to hold a name and age
pub struct Person {
    /// The person's name
    pub name: String,
    /// The person's age in years
    pub age: u32,
}

impl Person {
    /// Creates a new Person
    ///
    /// # Arguments
    ///
    /// * `name` - The person's name
    /// * `age` - The person's age
    ///
    /// # Examples
    ///
    /// ```
    /// let person = Person::new("Alice".to_string(), 30);
    /// ```
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    /// Gets the person's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the person's age
    pub fn age(&self) -> u32 {
        self.age
    }
}

/// Greets a person by name
///
/// # Arguments
///
/// * `name` - The name of the person to greet
///
/// # Returns
///
/// A greeting message
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// An enumeration of different colors
pub enum Color {
    /// Red color
    Red,
    /// Green color
    Green,
    /// Blue color
    Blue,
    /// Custom RGB color
    Rgb(u8, u8, u8),
}

/// A trait for things that can be colored
pub trait Colorable {
    /// Sets the color of the object
    fn set_color(&mut self, color: Color);

    /// Gets the current color of the object
    fn get_color(&self) -> &Color;
}
