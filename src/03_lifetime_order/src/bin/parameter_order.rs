//! Examples demonstrating the flexibility of parameter ordering in Rust

/// A struct with a reference and a generic type parameter
/// The order of the lifetime and type parameters doesn't matter
#[allow(dead_code)]
struct Container<'a, T> {
    value: &'a T,
}

/// Same struct with different parameter order
/// This is functionally identical to the above
#[allow(dead_code)]
struct ContainerAlt<'a, T> {
    value: &'a T,
}

/// A function with lifetime and type parameters
/// The order of the parameters doesn't affect functionality
fn process<'a, T>(_container: &'a Container<'a, T>) {
    // Implementation
}

/// Same function with different parameter order
fn process_alt<'a, T>(_container: &'a ContainerAlt<'a, T>) {
    // Implementation
}

/// A function with multiple lifetime parameters
/// The order of the lifetime parameters doesn't matter
fn combine<'a, 'b, T>(a: &'a T, b: &'b T) -> (&'a T, &'b T) {
    (a, b)
}

/// Same function with different lifetime parameter order
fn combine_alt<'b, 'a, T>(a: &'a T, b: &'b T) -> (&'a T, &'b T) {
    (a, b)
}

/// A function with const generics
/// The order of const and type parameters doesn't matter
fn create_array<T, const N: usize>() -> [T; N]
where
    T: Default + Copy,
{
    [T::default(); N]
}

/// Same function with different parameter order
fn create_array_alt<const N: usize, T>() -> [T; N]
where
    T: Default + Copy,
{
    [T::default(); N]
}

fn main() {
    // Create some containers
    let value = 42;
    let container = Container { value: &value };
    let container_alt = ContainerAlt { value: &value };

    // Process the containers
    process(&container);
    process_alt(&container_alt);

    // Combine values with different lifetime parameter orders
    let a = 1;
    let b = 2;
    let (a_ref, b_ref) = combine(&a, &b);
    let (a_ref_alt, b_ref_alt) = combine_alt(&a, &b);

    println!("a_ref: {a_ref}, b_ref: {b_ref}");
    println!("a_ref_alt: {a_ref_alt}, b_ref_alt: {b_ref_alt}");

    // Create arrays with different parameter orders
    let array1: [i32; 3] = create_array();
    let array2: [i32; 3] = create_array_alt();

    println!("array1: {array1:?}");
    println!("array2: {array2:?}");
}
