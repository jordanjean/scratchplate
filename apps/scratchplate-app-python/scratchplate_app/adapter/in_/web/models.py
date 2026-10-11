from __future__ import annotations

from pydantic import BaseModel, ConfigDict

from scratchplate_app.domain import WelcomeMessage


class WelcomeResponse(BaseModel):
    model_config = ConfigDict(frozen=True)

    message: str
    username: str
    role: str

    @classmethod
    def from_domain(cls, welcome: WelcomeMessage) -> "WelcomeResponse":
        return cls(message=welcome.text, username=welcome.username, role=welcome.role.value.lower())
