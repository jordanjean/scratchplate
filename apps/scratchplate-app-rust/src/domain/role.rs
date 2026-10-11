#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn from_name(name: &str) -> Option<Self> {
        let normalized = name.trim();
        if normalized.is_empty() {
            return None;
        }

        if normalized.eq_ignore_ascii_case("admin") {
            Some(Self::Admin)
        } else if normalized.eq_ignore_ascii_case("user") {
            Some(Self::User)
        } else {
            None
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::User => "user",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Role;

    #[test]
    fn resolves_role_names_case_insensitively() {
        assert_eq!(Role::from_name("admin"), Some(Role::Admin));
        assert_eq!(Role::from_name(" User "), Some(Role::User));
        assert_eq!(Role::from_name("offline_access"), None);
        assert_eq!(Role::from_name(""), None);
    }
}
