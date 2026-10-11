from __future__ import annotations

from typing import Protocol

from scratchplate_app.domain import Role


class WelcomeTemplatePort(Protocol):
    def template_for(self, role: Role) -> str: ...
