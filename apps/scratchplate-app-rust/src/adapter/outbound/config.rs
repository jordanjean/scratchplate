use crate::application::port_out::WelcomeTemplatePort;
use crate::domain::Role;

pub struct ConfigWelcomeTemplateAdapter {
    admin_template: String,
    user_template: String,
}

impl ConfigWelcomeTemplateAdapter {
    pub fn new(admin_template: impl Into<String>, user_template: impl Into<String>) -> Self {
        Self {
            admin_template: admin_template.into(),
            user_template: user_template.into(),
        }
    }
}

impl WelcomeTemplatePort for ConfigWelcomeTemplateAdapter {
    fn template_for(&self, role: Role) -> &str {
        match role {
            Role::Admin => &self.admin_template,
            Role::User => &self.user_template,
        }
    }
}
