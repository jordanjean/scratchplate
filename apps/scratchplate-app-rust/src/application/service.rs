use std::sync::Arc;

use crate::application::port_in::GreetUserUseCase;
use crate::application::port_out::WelcomeTemplatePort;
use crate::domain::{MissingRoleError, UserIdentity, WelcomeMessage};

pub struct GreetUserService {
    welcome_templates: Arc<dyn WelcomeTemplatePort>,
}

impl GreetUserService {
    pub fn new(welcome_templates: Arc<dyn WelcomeTemplatePort>) -> Self {
        Self { welcome_templates }
    }
}

impl GreetUserUseCase for GreetUserService {
    fn greet(&self, user: &UserIdentity) -> Result<WelcomeMessage, MissingRoleError> {
        let role = user
            .primary_role()
            .ok_or_else(|| MissingRoleError::new(user.username()))?;

        Ok(WelcomeMessage::from_template(
            self.welcome_templates.template_for(role),
            user,
            role,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::GreetUserService;
    use crate::application::port_in::GreetUserUseCase;
    use crate::application::port_out::WelcomeTemplatePort;
    use crate::domain::{Role, UserIdentity};

    struct TestTemplates;

    impl WelcomeTemplatePort for TestTemplates {
        fn template_for(&self, role: Role) -> &str {
            match role {
                Role::Admin => "Hello admin {username}",
                Role::User => "Hello user {username}",
            }
        }
    }

    #[test]
    fn greets_admin() {
        let service = GreetUserService::new(Arc::new(TestTemplates));
        let message = service
            .greet(&UserIdentity::new("alice", [Role::Admin]).unwrap())
            .unwrap();

        assert_eq!(message.role(), Role::Admin);
        assert_eq!(message.text(), "Hello admin alice");
    }

    #[test]
    fn greets_user() {
        let service = GreetUserService::new(Arc::new(TestTemplates));
        let message = service
            .greet(&UserIdentity::new("bob", [Role::User]).unwrap())
            .unwrap();

        assert_eq!(message.role(), Role::User);
        assert_eq!(message.text(), "Hello user bob");
    }

    #[test]
    fn most_privileged_role_wins() {
        let service = GreetUserService::new(Arc::new(TestTemplates));
        let message = service
            .greet(&UserIdentity::new("carol", [Role::User, Role::Admin]).unwrap())
            .unwrap();

        assert_eq!(message.role(), Role::Admin);
    }

    #[test]
    fn rejects_users_without_roles() {
        let service = GreetUserService::new(Arc::new(TestTemplates));
        let error = service
            .greet(&UserIdentity::new("dave", []).unwrap())
            .unwrap_err();

        assert_eq!(error.to_string(), "User 'dave' does not hold any supported role");
    }
}
