use super::{Role, UserIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WelcomeMessage {
    username: String,
    role: Role,
    text: String,
}

impl WelcomeMessage {
    pub const USERNAME_PLACEHOLDER: &str = "{username}";

    pub fn from_template(template: &str, user: &UserIdentity, role: Role) -> Self {
        Self {
            username: user.username().to_owned(),
            role,
            text: template.replace(Self::USERNAME_PLACEHOLDER, user.username()),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
