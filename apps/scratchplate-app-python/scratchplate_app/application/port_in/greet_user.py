from __future__ import annotations

from typing import Protocol

from scratchplate_app.domain import UserIdentity, WelcomeMessage


class GreetUserUseCase(Protocol):
    def greet(self, user: UserIdentity) -> WelcomeMessage: ...
