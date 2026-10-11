from __future__ import annotations

from scratchplate_app.application.port_out import WelcomeTemplatePort
from scratchplate_app.domain import Role
from scratchplate_app.infrastructure.settings import Settings


class ConfigurationWelcomeTemplateAdapter(WelcomeTemplatePort):
    def __init__(self, settings: Settings) -> None:
        self._settings = settings

    def template_for(self, role: Role) -> str:
        if role is Role.ADMIN:
            return self._settings.welcome_admin_template
        return self._settings.welcome_user_template
