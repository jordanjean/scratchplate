from __future__ import annotations

import pytest

from scratchplate_app.application.service import GreetUserService
from scratchplate_app.domain import MissingRoleError, Role, UserIdentity, WelcomeMessage


class StubTemplates:
    def template_for(self, role: Role) -> str:
        return {
            Role.ADMIN: "Hello admin {username}",
            Role.USER: "Hello user {username}",
        }[role]


def test_role_names_are_case_insensitive() -> None:
    assert Role.from_name("admin") is Role.ADMIN
    assert Role.from_name(" User ") is Role.USER
    assert Role.from_name("offline_access") is None
    assert Role.from_name(None) is None


def test_user_identity_rejects_blank_username() -> None:
    with pytest.raises(ValueError):
        UserIdentity(" ", frozenset())


def test_primary_role_prefers_admin() -> None:
    identity = UserIdentity("carol", frozenset({Role.USER, Role.ADMIN}))

    assert identity.primary_role() is Role.ADMIN


def test_welcome_message_uses_plain_replace() -> None:
    user = UserIdentity("alice", frozenset({Role.ADMIN}))

    message = WelcomeMessage.from_template("Welcome {username}! {username}", user, Role.ADMIN)

    assert message.text == "Welcome alice! alice"


def test_greet_user_service_returns_admin_message() -> None:
    service = GreetUserService(StubTemplates())

    message = service.greet(UserIdentity("alice", frozenset({Role.ADMIN})))

    assert message.role is Role.ADMIN
    assert message.text == "Hello admin alice"


def test_greet_user_service_raises_when_role_is_missing() -> None:
    service = GreetUserService(StubTemplates())

    with pytest.raises(MissingRoleError):
        service.greet(UserIdentity("guest", frozenset()))
