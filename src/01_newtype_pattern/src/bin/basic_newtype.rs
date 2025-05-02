//! Basic example of the newtype pattern

/// A newtype wrapper around String to represent a user ID
/// This provides type safety and prevents mixing with other string types
struct UserId(String);

impl UserId {
    /// Create a new `UserId`
    fn new(id: String) -> Self {
        UserId(id)
    }

    /// Get the underlying string value
    fn as_str(&self) -> &str {
        &self.0
    }
}

/// A newtype wrapper around String to represent an email address
struct EmailAddress(String);

impl EmailAddress {
    /// Create a new `EmailAddress` with basic validation
    fn new(email: String) -> Option<Self> {
        if email.contains('@') {
            Some(EmailAddress(email))
        } else {
            None
        }
    }

    /// Get the underlying string value
    fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() {
    // Create a UserId
    let user_id = UserId::new("user123".to_string());
    println!("User ID: {}", user_id.as_str());

    // Create an EmailAddress
    if let Some(email) = EmailAddress::new("user@example.com".to_string()) {
        println!("Email: {}", email.as_str());
    } else {
        println!("Invalid email address");
    }

    // This would not compile because UserId and EmailAddress are different types
    // let invalid: EmailAddress = user_id;

    // We can implement conversion if needed
    // impl From<UserId> for EmailAddress { ... }
}
