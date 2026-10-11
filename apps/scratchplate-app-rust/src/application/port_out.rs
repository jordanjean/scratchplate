use crate::domain::Role;

pub trait WelcomeTemplatePort: Send + Sync {
    fn template_for(&self, role: Role) -> &str;
}
