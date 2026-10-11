from __future__ import annotations

from scratchplate_app.application.port_in import GreetUserUseCase
from scratchplate_app.application.port_out import WelcomeTemplatePort
from scratchplate_app.domain import MissingRoleError, UserIdentity, WelcomeMessage


class GreetUserService(GreetUserUseCase):
    def __init__(self, welcome_templates: WelcomeTemplatePort) -> None:
        self._welcome_templates = welcome_templates

    def greet(self, user: UserIdentity) -> WelcomeMessage:
        role = user.primary_role()
        if role is None:
            raise MissingRoleError(user.username)
        return WelcomeMessage.from_template(self._welcome_templates.template_for(role), user, role)
