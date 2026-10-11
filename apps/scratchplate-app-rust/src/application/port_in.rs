use crate::domain::{MissingRoleError, UserIdentity, WelcomeMessage};

pub trait GreetUserUseCase: Send + Sync {
    fn greet(&self, user: &UserIdentity) -> Result<WelcomeMessage, MissingRoleError>;
}
