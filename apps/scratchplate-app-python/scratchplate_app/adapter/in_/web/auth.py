from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from fastapi import Header
from fastapi.responses import Response

from scratchplate_app.domain import Role, UserIdentity
from scratchplate_app.infrastructure.security import InvalidTokenError, TokenVerifier

WWW_AUTHENTICATE_HEADER = {"WWW-Authenticate": "Bearer"}


@dataclass(frozen=True, slots=True)
class AuthenticatedRequest:
    user: UserIdentity
    claims: dict[str, Any]


def _unauthorized() -> Response:
    return Response(status_code=401, headers=WWW_AUTHENTICATE_HEADER)


def _forbidden() -> Response:
    return Response(status_code=403)


def _parse_bearer_token(authorization: str | None) -> str:
    if authorization is None:
        raise InvalidTokenError("Missing bearer token")
    scheme, _, value = authorization.partition(" ")
    if scheme.lower() != "bearer" or not value.strip():
        raise InvalidTokenError("Missing bearer token")
    return value.strip()


def _username_from_claims(claims: dict[str, Any]) -> str:
    preferred_username = str(claims.get("preferred_username") or "").strip()
    if preferred_username:
        return preferred_username
    subject = str(claims.get("sub") or "").strip()
    if subject:
        return subject
    raise InvalidTokenError("Missing subject")


def extract_roles(claims: dict[str, Any]) -> frozenset[Role]:
    realm_access = claims.get("realm_access")
    if not isinstance(realm_access, dict):
        return frozenset()
    roles = realm_access.get("roles")
    if not isinstance(roles, list):
        return frozenset()
    resolved_roles = {role for raw_role in roles if isinstance(raw_role, str) if (role := Role.from_name(raw_role))}
    return frozenset(resolved_roles)


def authenticate_request(token_verifier: TokenVerifier, authorization: str | None) -> AuthenticatedRequest:
    token = _parse_bearer_token(authorization)
    claims = token_verifier.verify(token)
    user = UserIdentity(username=_username_from_claims(claims), roles=extract_roles(claims))
    return AuthenticatedRequest(user=user, claims=claims)


async def require_authenticated_request(
    authorization: str | None = Header(default=None),
    token_verifier: TokenVerifier | None = None,
) -> AuthenticatedRequest:
    assert token_verifier is not None
    return authenticate_request(token_verifier, authorization)


async def require_authorized_user(
    authorization: str | None = Header(default=None),
    token_verifier: TokenVerifier | None = None,
) -> UserIdentity:
    assert token_verifier is not None
    authenticated = authenticate_request(token_verifier, authorization)
    if not authenticated.user.roles:
        raise PermissionError("Authenticated caller lacks a supported role")
    return authenticated.user
