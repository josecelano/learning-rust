// Example 7: Generic Traits
// This example demonstrates the difference between generic traits with associated types
// and generic methods, and when to use each approach.

// Generic trait with associated type
trait Container {
    type Item; // Associated type
    fn get(&self) -> &Self::Item;
    fn set(&mut self, item: Self::Item);
}

// Implementation for a simple container
struct Box<T> {
    value: T,
}

impl<T> Container for Box<T> {
    type Item = T; // Specify the associated type

    fn get(&self) -> &Self::Item {
        &self.value
    }

    fn set(&mut self, item: Self::Item) {
        self.value = item;
    }
}

// Generic trait with generic methods
trait Storage<T> {
    fn store(&mut self, item: T);
    fn retrieve(&self) -> &T;
}

// Implementation for a simple storage
struct Shelf<T> {
    item: T,
}

impl<T> Storage<T> for Shelf<T> {
    fn store(&mut self, item: T) {
        self.item = item;
    }

    fn retrieve(&self) -> &T {
        &self.item
    }
}

#[allow(unused_variables)]
fn main() {
    // Using Container with associated type
    let mut box_container = Box { value: 42 };
    println!("Box value: {}", box_container.get());
    box_container.set(100);
    println!("New box value: {}", box_container.get());

    // Using Storage with generic methods
    let mut shelf = Shelf { item: "book" };
    println!("Shelf item: {}", shelf.retrieve());
    shelf.store("magazine");
    println!("New shelf item: {}", shelf.retrieve());

    // Demonstrate type safety
    let number_box: Box<i32> = Box { value: 42 };
    // This would not compile because we can't change the type of the associated type
    // number_box.set("string");  // Error: expected integer, found string

    let string_shelf: Shelf<String> = Shelf {
        item: String::from("book"),
    };
    // This would not compile because the type is fixed at implementation
    // string_shelf.store(42);  // Error: expected String, found integer
}

// Key differences between the two approaches:
//
// 1. Associated Types (Container trait):
//    - Type is specified once when implementing the trait
//    - Cannot change the type after implementation
//    - More restrictive but clearer type relationships
//    - Better when the type is fundamental to the trait's purpose
//
// 2. Generic Methods (Storage trait):
//    - Type is specified when using the trait
//    - Can implement the trait multiple times with different types
//    - More flexible but potentially more complex
//    - Better when the type is just a parameter to the trait's methods
