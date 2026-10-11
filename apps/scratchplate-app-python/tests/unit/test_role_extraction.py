from __future__ import annotations

from scratchplate_app.adapter.in_.web.auth import extract_roles
from scratchplate_app.domain import Role


def test_extract_roles_filters_unknown_and_is_case_insensitive() -> None:
    claims = {"realm_access": {"roles": ["user", "ADMIN", "offline_access"]}}

    assert extract_roles(claims) == frozenset({Role.USER, Role.ADMIN})


def test_extract_roles_handles_missing_realm_access() -> None:
    assert extract_roles({}) == frozenset()
