from __future__ import annotations

from enum import Enum


class Role(str, Enum):
    ADMIN = "ADMIN"
    USER = "USER"

    @classmethod
    def from_name(cls, name: str | None) -> "Role | None":
        if name is None:
            return None
        normalized = name.strip().upper()
        if not normalized:
            return None
        try:
            return cls[normalized]
        except KeyError:
            return None
