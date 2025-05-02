//! Example of using the newtype pattern for configuration option flags

/// A newtype wrapper around bool to represent a verbose logging flag
/// This provides semantic meaning and prevents mixing with other boolean flags
#[derive(Debug, Clone, Copy)]
struct VerboseLogging(bool);

impl VerboseLogging {
    /// Create a new `VerboseLogging` flag
    fn new(enabled: bool) -> Self {
        VerboseLogging(enabled)
    }

    /// Get the underlying boolean value
    fn is_enabled(self) -> bool {
        self.0
    }
}

/// A newtype wrapper around bool to represent a debug mode flag
#[derive(Debug, Clone, Copy)]
struct DebugMode(bool);

impl DebugMode {
    /// Create a new `DebugMode` flag
    fn new(enabled: bool) -> Self {
        DebugMode(enabled)
    }

    /// Get the underlying boolean value
    fn is_enabled(self) -> bool {
        self.0
    }
}

/// A newtype wrapper around bool to represent a cache enabled flag
#[derive(Debug, Clone, Copy)]
struct CacheEnabled(bool);

impl CacheEnabled {
    /// Create a new `CacheEnabled` flag
    fn new(enabled: bool) -> Self {
        CacheEnabled(enabled)
    }

    /// Get the underlying boolean value
    fn is_enabled(self) -> bool {
        self.0
    }
}

/// Example function that takes configuration flags as parameters
fn process_data(
    data: &str,
    verbose: VerboseLogging,
    debug: DebugMode,
    cache: CacheEnabled,
) -> String {
    if verbose.is_enabled() {
        println!("Processing data: {data}");
    }

    if debug.is_enabled() {
        println!("Debug mode enabled, showing additional information");
    }

    if cache.is_enabled() {
        println!("Caching results for future use");
    }

    // Process the data and return a result
    format!("Processed: {data}")
}

fn main() {
    // Create configuration flags
    let verbose = VerboseLogging::new(true);
    let debug = DebugMode::new(false);
    let cache = CacheEnabled::new(true);

    // Call the function with the flags
    // Notice how the parameter names make the code more readable
    let result = process_data("example data", verbose, debug, cache);

    println!("Result: {result}");

    // Example of how this improves readability compared to using raw booleans
    // Without newtype pattern:
    // process_data("example data", true, false, true);
    // With newtype pattern:
    // process_data("example data", VerboseLogging::new(true), DebugMode::new(false), CacheEnabled::new(true));

    // This would not compile because VerboseLogging and DebugMode are different types
    // let invalid: DebugMode = verbose;
}
