//! # Trait Objects & Dynamic Dispatch Test Fixture
//!
//! This fixture demonstrates various trait object patterns in Rust,
//! including dyn Trait usage, object safety, and dynamic dispatch.

use std::fmt::Display;

/// Object-safe trait for dynamic dispatch
///
/// This trait is designed to be object-safe, allowing it to be used
/// as a trait object with `dyn Draw`.
pub trait Draw {
    /// Draw the object to some output
    fn draw(&self) -> String;
    
    /// Get the name of the drawable object
    fn name(&self) -> &str {
        "drawable"
    }
}

/// Another object-safe trait for composition
///
/// Demonstrates multiple trait objects in the same codebase.
pub trait Clickable {
    /// Handle click events
    fn on_click(&mut self);
    
    /// Check if the object is clickable
    fn is_clickable(&self) -> bool {
        true
    }
}

/// Trait that extends another trait
///
/// Shows trait inheritance and how it works with trait objects.
pub trait Interactive: Draw + Clickable {
    /// Handle focus events
    fn on_focus(&mut self);
}

/// Simple struct implementing Draw
///
/// Basic implementation to demonstrate trait objects.
pub struct Circle {
    pub radius: f64,
}

impl Draw for Circle {
    fn draw(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
    
    fn name(&self) -> &str {
        "circle"
    }
}

impl Clickable for Circle {
    fn on_click(&mut self) {
        println!("Circle clicked!");
    }
}

impl Interactive for Circle {
    fn on_focus(&mut self) {
        println!("Circle focused!");
    }
}

/// Another struct implementing Draw
///
/// Shows multiple implementations of the same trait.
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) -> String {
        format!("Rectangle {}x{}", self.width, self.height)
    }
    
    fn name(&self) -> &str {
        "rectangle"
    }
}

impl Clickable for Rectangle {
    fn on_click(&mut self) {
        println!("Rectangle clicked!");
    }
}

/// Function taking a trait object by reference
///
/// Demonstrates `&dyn Trait` usage in function parameters.
pub fn draw_shape(shape: &dyn Draw) -> String {
    format!("Drawing: {}", shape.draw())
}

/// Function taking a mutable trait object by reference
///
/// Shows mutable borrowing of trait objects.
pub fn click_shape(shape: &mut dyn Clickable) {
    if shape.is_clickable() {
        shape.on_click();
    }
}

/// Function taking a boxed trait object
///
/// Demonstrates `Box<dyn Trait>` ownership patterns.
pub fn consume_shape(shape: Box<dyn Draw>) -> String {
    format!("Consumed: {}", shape.draw())
}

/// Function returning a boxed trait object
///
/// Shows how to return trait objects from functions.
pub fn create_circle(radius: f64) -> Box<dyn Draw> {
    Box::new(Circle { radius })
}

/// Function returning different trait objects
///
/// Demonstrates dynamic dispatch with conditional returns.
pub fn create_shape(shape_type: &str, size: f64) -> Box<dyn Draw> {
    match shape_type {
        "circle" => Box::new(Circle { radius: size }),
        "rectangle" => Box::new(Rectangle { width: size, height: size }),
        _ => Box::new(Circle { radius: 1.0 }),
    }
}

/// Struct containing a trait object
///
/// Shows how to store trait objects in structs.
pub struct Canvas {
    pub shapes: Vec<Box<dyn Draw>>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }
    
    pub fn add_shape(&mut self, shape: Box<dyn Draw>) {
        self.shapes.push(shape);
    }
    
    pub fn draw_all(&self) -> Vec<String> {
        self.shapes.iter().map(|shape| shape.draw()).collect()
    }
}

/// Generic function with trait object conversion
///
/// Demonstrates converting from generic types to trait objects.
pub fn as_drawable<T: Draw + 'static>(item: T) -> Box<dyn Draw> {
    Box::new(item)
}

/// Trait that combines multiple traits
///
/// Shows how to combine multiple traits for trait objects.
pub trait DisplayableDrawable: Draw + Display + Send {}

/// Blanket implementation for any type that implements the required traits
impl<T> DisplayableDrawable for T where T: Draw + Display + Send {}

/// Function with complex trait object bounds
///
/// Demonstrates trait objects with Send + Sync bounds.
pub fn process_drawable(drawable: Box<dyn Draw + Send + Sync>) -> String {
    format!("Processing: {}", drawable.draw())
}

/// Non-object-safe trait (for contrast)
///
/// This trait cannot be used as a trait object due to generic methods.
pub trait NonObjectSafe {
    /// Generic method makes this trait non-object-safe
    fn generic_method<T>(&self, value: T) -> T;
    
    /// Associated function makes this trait non-object-safe
    fn associated_function() -> Self;
}

/// Trait with associated types
///
/// Shows trait objects with associated types.
pub trait Producer {
    type Item;
    
    fn produce(&self) -> Self::Item;
}

/// Concrete implementation of Producer
pub struct StringProducer {
    pub prefix: String,
}

impl Producer for StringProducer {
    type Item = String;
    
    fn produce(&self) -> Self::Item {
        format!("{}: produced", self.prefix)
    }
}

/// Function working with trait objects with associated types
///
/// Note: This is tricky because you can't directly use `dyn Producer`
/// without specifying the associated type.
pub fn use_string_producer(producer: &dyn Producer<Item = String>) -> String {
    producer.produce()
}

/// Trait object with lifetime parameters
///
/// Shows how trait objects interact with lifetimes.
pub trait Borrowing<'a> {
    fn borrow_str(&self) -> &'a str;
}

/// Function taking trait object with lifetime
///
/// Demonstrates lifetime parameters in trait objects.
pub fn use_borrowing<'a>(borrower: &dyn Borrowing<'a>) -> &'a str {
    borrower.borrow_str()
}

/// Higher-ranked trait bounds with trait objects
///
/// Shows `for<'a>` syntax with trait objects.
pub fn use_higher_ranked_trait_object(
    f: Box<dyn for<'a> Fn(&'a str) -> &'a str>
) -> Box<dyn for<'a> Fn(&'a str) -> &'a str> {
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_objects() {
        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle { width: 10.0, height: 20.0 };
        
        // Test trait object references
        assert_eq!(draw_shape(&circle), "Drawing: Circle with radius 5");
        assert_eq!(draw_shape(&rectangle), "Drawing: Rectangle 10x20");
        
        // Test boxed trait objects
        let boxed_circle = create_circle(3.0);
        assert_eq!(consume_shape(boxed_circle), "Consumed: Circle with radius 3");
        
        // Test canvas with multiple shapes
        let mut canvas = Canvas::new();
        canvas.add_shape(Box::new(Circle { radius: 1.0 }));
        canvas.add_shape(Box::new(Rectangle { width: 2.0, height: 3.0 }));
        
        let drawings = canvas.draw_all();
        assert_eq!(drawings.len(), 2);
        assert!(drawings[0].contains("Circle"));
        assert!(drawings[1].contains("Rectangle"));
    }
}