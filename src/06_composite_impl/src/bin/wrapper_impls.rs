//! Examples demonstrating implementing traits for composite types

use std::boxed::Box;
use std::sync::Arc;

/// A trait for types that can be converted to a string representation
trait Stringify {
    fn to_string(&self) -> String;
}

/// A simple struct that we'll wrap
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

/// Implement Stringify for Person
impl Stringify for Person {
    fn to_string(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

/// Implement Stringify for Arc<Person>
/// This allows us to call `to_string()` on Arc<Person>
impl Stringify for Arc<Person> {
    fn to_string(&self) -> String {
        // We can access the inner Person through the Arc
        format!("(Arc) {}", self.as_ref().to_string())
    }
}

/// Implement Stringify for Box<Person>
/// This allows us to call `to_string()` on Box<Person>
impl Stringify for Box<Person> {
    fn to_string(&self) -> String {
        // We can access the inner Person through the Box
        format!("(Box) {}", self.as_ref().to_string())
    }
}

/// A custom wrapper type
struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    fn new(inner: T) -> Self {
        Wrapper { inner }
    }
}

/// Implement Stringify for Wrapper<T> where T: Stringify
/// This is a generic implementation for any type that implements Stringify
impl<T: Stringify> Stringify for Wrapper<T> {
    fn to_string(&self) -> String {
        format!("(Wrapper) {}", self.inner.to_string())
    }
}

/// A type alias for Arc<Person>
type PersonArc = Arc<Person>;

fn main() {
    // Create a Person
    let person = Person::new("Alice".to_string(), 30);
    println!("Person: {}", person.to_string());

    // Create an Arc<Person>
    let person_arc = Arc::new(Person::new("Bob".to_string(), 25));
    println!("Arc<Person>: {}", person_arc.to_string());

    // Create a Box<Person>
    let person_box = Box::new(Person::new("Charlie".to_string(), 35));
    println!("Box<Person>: {}", person_box.to_string());

    // Create a Wrapper<Person>
    let person_wrapper = Wrapper::new(Person::new("David".to_string(), 40));
    println!("Wrapper<Person>: {}", person_wrapper.to_string());

    // Using the type alias PersonArc (which is the same as Arc<Person>)
    let person_arc_alias: PersonArc = Arc::new(Person::new("Eve".to_string(), 45));
    println!("PersonArc: {}", person_arc_alias.to_string());

    // We can also wrap other types that implement Stringify
    let wrapper_arc = Wrapper::new(Arc::new(Person::new("Frank".to_string(), 50)));
    println!("Wrapper<Arc<Person>>: {}", wrapper_arc.to_string());
}
