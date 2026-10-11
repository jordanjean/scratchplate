from __future__ import annotations

from datetime import UTC, datetime, timedelta
from typing import Any

import jwt

TEST_SECRET = "not-a-private-key"
TEST_ISSUER = "http://issuer.example.test/realms/scratchplate"


def make_token(
    subject: str,
    issuer: str = TEST_ISSUER,
    roles: list[str] | None = None,
    preferred_username: str | None = None,
    expires_delta: timedelta = timedelta(minutes=5),
    secret: str = TEST_SECRET,
) -> str:
    now = datetime.now(UTC)
    payload: dict[str, Any] = {
        "sub": subject,
        "iss": issuer,
        "exp": now + expires_delta,
        "realm_access": {"roles": roles or []},
    }
    if preferred_username is not None:
        payload["preferred_username"] = preferred_username
    return jwt.encode(payload, secret, algorithm="HS256", headers={"kid": "test-kid"})
