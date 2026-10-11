use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingRoleError {
    username: String,
}

impl MissingRoleError {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

impl Display for MissingRoleError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "User '{}' does not hold any supported role", self.username)
    }
}

impl Error for MissingRoleError {}
