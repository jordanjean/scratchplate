use std::collections::BTreeSet;

use super::Role;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserIdentity {
    username: String,
    roles: BTreeSet<Role>,
}

impl UserIdentity {
    pub fn new(username: impl Into<String>, roles: impl IntoIterator<Item = Role>) -> Result<Self, &'static str> {
        let username = username.into();
        if username.trim().is_empty() {
            return Err("username must not be blank");
        }

        Ok(Self {
            username,
            roles: roles.into_iter().collect(),
        })
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn roles(&self) -> &BTreeSet<Role> {
        &self.roles
    }

    pub fn primary_role(&self) -> Option<Role> {
        self.roles.iter().next().copied()
    }

    pub fn has_supported_role(&self) -> bool {
        self.primary_role().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::UserIdentity;
    use crate::domain::Role;

    #[test]
    fn rejects_blank_username() {
        assert!(UserIdentity::new("", []).is_err());
        assert!(UserIdentity::new(" ", []).is_err());
    }

    #[test]
    fn primary_role_is_most_privileged() {
        let user = UserIdentity::new("alice", [Role::User, Role::Admin]).unwrap();
        assert_eq!(user.primary_role(), Some(Role::Admin));
        assert!(user.has_supported_role());
    }

    #[test]
    fn supports_users_without_roles() {
        let user = UserIdentity::new("guest", []).unwrap();
        assert_eq!(user.primary_role(), None);
        assert!(!user.has_supported_role());
    }
}
