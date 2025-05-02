//! Examples demonstrating non-primitive constants in Rust

/// A simple point struct
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// A simple color enum
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

/// A constant struct
const ORIGIN: Point = Point { x: 0, y: 0 };

/// A constant enum variant
const DEFAULT_COLOR: Color = Color::Blue;

/// A constant array
const COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

/// A const function to create a point
const fn create_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

/// A constant created using a const function
const POINT_ONE: Point = create_point(1, 1);

/// A struct with a const generic parameter
struct Grid<const N: usize> {
    data: [[i32; N]; N],
}

impl<const N: usize> Grid<N> {
    /// Create a new grid with all zeros
    const fn new() -> Self {
        Grid { data: [[0; N]; N] }
    }
}

/// A constant grid
const GRID_3X3: Grid<3> = Grid::new();

fn main() {
    // Using the constant struct
    println!("Origin: {ORIGIN:?}");

    // Using the constant enum
    println!("Default color: {DEFAULT_COLOR:?}");

    // Using the constant array
    println!("Colors: {COLORS:?}");

    // Using the constant created with a const function
    println!("Point one: {POINT_ONE:?}");

    // Using the constant grid
    println!("Grid 3x3: {:?}", GRID_3X3.data);

    // Creating a new grid at runtime
    let grid_2x2 = Grid::<2>::new();
    println!("Grid 2x2: {:?}", grid_2x2.data);

    // Using constants in expressions
    let distance = (POINT_ONE.x - ORIGIN.x).pow(2) + (POINT_ONE.y - ORIGIN.y).pow(2);
    println!("Distance from origin to point one: {distance}");
}
