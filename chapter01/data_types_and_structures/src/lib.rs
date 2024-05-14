/// Represents a User
pub struct User {
    /// The user's id
    pub id: u32,
    /// The user's name
    pub name: String,
    /// The user's email
    pub email: String,
    /// The user's active status
    pub active: bool,
}

impl User {
    /// Creates a new user
    pub fn new(id: u32, name: &str, email: &str) -> User {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active: true,
        }
    }

    /// Deactivates the user
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

/// Represents a Request
pub enum RequestStatus {
    Pending,
    Processing,
    Completed,
    Failed(String), // Variant with associated data
}

impl RequestStatus {
    pub fn message(&self) -> &str {
        match self {
            RequestStatus::Pending => "Request is pending",
            RequestStatus::Processing => "Request is being processed",
            RequestStatus::Completed => "Request completed successfully",
            RequestStatus::Failed(err) => err,
        }
    }
}

pub fn get_user_info() -> (String, String) {
    (String::from("Alice"), String::from("alice@example.com"))
}