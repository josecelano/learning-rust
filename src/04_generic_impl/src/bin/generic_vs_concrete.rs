//! Examples demonstrating the difference between generic and concrete implementations

/// A generic container that can hold any type
struct Container<T> {
    value: T,
}

/// A concrete container that can only hold strings
struct StringContainer {
    value: String,
}

/// Implementation for the generic container
/// This applies to Container<T> for any type T
#[allow(dead_code)]
impl<T> Container<T> {
    /// Create a new container with the given value
    fn new(value: T) -> Self {
        Container { value }
    }

    /// Get a reference to the value
    fn get(&self) -> &T {
        &self.value
    }

    /// Get the length of the value
    fn len(&self) -> usize
    where
        T: AsRef<str>,
    {
        self.value.as_ref().len()
    }
}

/// Implementation specific to Container<String>
/// This only applies to Container<String>, not to Container<T> for other types
#[allow(dead_code)]
impl Container<String> {
    /// Get the length of the string
    fn string_length(&self) -> usize {
        self.value.len()
    }
}

/// Generic implementation for Container<T> with a generic method
/// This requires impl<T> because the method itself is generic
#[allow(dead_code)]
impl<T> Container<T> {
    /// Convert the container to a different type
    /// This method is generic over U, so we need impl<T>
    fn convert<U>(self) -> Container<U>
    where
        U: From<T>,
    {
        Container {
            value: U::from(self.value),
        }
    }
}

/// Implementation for the concrete string container
impl StringContainer {
    /// Create a new string container
    fn new(value: String) -> Self {
        StringContainer { value }
    }

    /// Get a reference to the string
    fn get_value(&self) -> &String {
        &self.value
    }

    /// Get the length of the string
    fn string_length(&self) -> usize {
        self.value.len()
    }

    /// Convert to a different string type
    /// This is a concrete method, not generic
    fn to_uppercase(&self) -> String {
        self.value.to_uppercase()
    }
}

fn main() {
    // Using the generic container
    let number_container = Container::new(42);
    println!("Number container value: {}", number_container.get());

    // Using the generic container with a string
    let string_container = Container::new("hello".to_string());
    println!("String container value: {}", string_container.get());
    println!("String container length: {}", string_container.len());

    // Converting between types using ToString trait
    let number_container = Container::new(42);
    let string_container: Container<String> = Container::new(number_container.get().to_string());
    println!("Converted to string: {}", string_container.get());

    // Using the concrete string container
    let concrete_container = StringContainer::new("world".to_string());
    println!(
        "Concrete container value: {}",
        concrete_container.get_value()
    );
    println!(
        "Concrete container length: {}",
        concrete_container.string_length()
    );
    println!(
        "Concrete container uppercase: {}",
        concrete_container.to_uppercase()
    );
}
