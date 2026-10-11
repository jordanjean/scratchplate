from __future__ import annotations

from dataclasses import dataclass

from .role import Role
from .user_identity import UserIdentity


@dataclass(frozen=True, slots=True)
class WelcomeMessage:
    USERNAME_PLACEHOLDER = "{username}"

    username: str
    role: Role
    text: str

    @classmethod
    def from_template(cls, template: str, user: UserIdentity, role: Role) -> "WelcomeMessage":
        return cls(
            username=user.username,
            role=role,
            text=template.replace(cls.USERNAME_PLACEHOLDER, user.username),
        )
