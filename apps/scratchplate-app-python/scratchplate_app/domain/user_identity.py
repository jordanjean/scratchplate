from __future__ import annotations

from dataclasses import dataclass, field

from .role import Role


@dataclass(frozen=True, slots=True)
class UserIdentity:
    username: str
    roles: frozenset[Role] = field(default_factory=frozenset)

    def __post_init__(self) -> None:
        if not self.username or self.username.isspace():
            raise ValueError("username must not be blank")
        object.__setattr__(self, "roles", frozenset(self.roles))

    def primary_role(self) -> Role | None:
        if Role.ADMIN in self.roles:
            return Role.ADMIN
        if Role.USER in self.roles:
            return Role.USER
        return None

    def has_role(self, role: Role) -> bool:
        return role in self.roles
