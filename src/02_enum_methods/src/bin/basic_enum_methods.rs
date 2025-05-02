//! Basic example of enum methods and traits

/// A simple enum representing a shape
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Square { side: f64 },
}

impl Shape {
    /// Calculate the area of the shape
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Square { side } => side * side,
        }
    }

    /// Check if the shape is a circle
    fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle { .. })
    }

    /// Get the name of the shape
    fn name(&self) -> &'static str {
        match self {
            Shape::Circle { .. } => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Square { .. } => "Square",
        }
    }
}

/// A trait for shapes that can be described
trait Describable {
    fn describe(&self) -> String;
}

/// Implement the Describable trait for Shape
impl Describable for Shape {
    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("A circle with radius {radius}"),
            Shape::Rectangle { width, height } => {
                format!("A rectangle with width {width} and height {height}")
            }
            Shape::Square { side } => format!("A square with side length {side}"),
        }
    }
}

fn main() {
    // Create some shapes
    let circle = Shape::Circle { radius: 5.0 };
    let rectangle = Shape::Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let square = Shape::Square { side: 7.0 };

    // Use enum methods
    println!("{} has area {}", circle.name(), circle.area());
    println!("{} has area {}", rectangle.name(), rectangle.area());
    println!("{} has area {}", square.name(), square.area());

    // Check if a shape is a circle
    println!("Is circle a circle? {}", circle.is_circle());
    println!("Is rectangle a circle? {}", rectangle.is_circle());

    // Use the Describable trait
    println!("Description: {}", circle.describe());
    println!("Description: {}", rectangle.describe());
    println!("Description: {}", square.describe());
}
