from __future__ import annotations

import json
import time
import urllib.error
import urllib.request
from dataclasses import dataclass, field
from threading import Lock
from typing import Any, Protocol

import jwt
from jwt import InvalidTokenError as PyJwtInvalidTokenError
from jwt.algorithms import RSAAlgorithm


class InvalidTokenError(RuntimeError):
    pass


class SigningKeyProvider(Protocol):
    algorithms: tuple[str, ...]

    def get_key(self, headers: dict[str, Any]) -> Any: ...


@dataclass(slots=True)
class StaticSigningKeyProvider(SigningKeyProvider):
    key: str
    algorithms: tuple[str, ...] = ("HS256",)

    def get_key(self, headers: dict[str, Any]) -> Any:
        _ = headers
        return self.key


@dataclass(slots=True)
class JwksSigningKeyProvider(SigningKeyProvider):
    jwk_set_uri: str
    refresh_interval_seconds: float = 30.0
    algorithms: tuple[str, ...] = ("RS256",)
    _lock: Lock = field(default_factory=Lock, init=False, repr=False)
    _keys_by_kid: dict[str, Any] = field(default_factory=dict, init=False, repr=False)
    _last_refresh_monotonic: float = field(default=0.0, init=False, repr=False)

    def get_key(self, headers: dict[str, Any]) -> Any:
        kid = headers.get("kid")
        if not kid or not isinstance(kid, str):
            raise InvalidTokenError("Missing key id")

        with self._lock:
            key = self._keys_by_kid.get(kid)
            if key is not None:
                return key
            if self._should_refresh_locked():
                self._refresh_locked()
                key = self._keys_by_kid.get(kid)
            if key is None:
                raise InvalidTokenError("Unknown key id")
            return key

    def _should_refresh_locked(self) -> bool:
        return (not self._keys_by_kid) or (
            time.monotonic() - self._last_refresh_monotonic >= self.refresh_interval_seconds
        )

    def _refresh_locked(self) -> None:
        try:
            with urllib.request.urlopen(self.jwk_set_uri, timeout=5) as response:
                document = json.load(response)
        except (OSError, urllib.error.URLError, json.JSONDecodeError) as exc:
            raise InvalidTokenError("Unable to refresh JWKS") from exc
        keys = document.get("keys")
        if not isinstance(keys, list):
            raise InvalidTokenError("Invalid JWKS document")

        refreshed: dict[str, Any] = {}
        for jwk in keys:
            if not isinstance(jwk, dict):
                continue
            kid = jwk.get("kid")
            if not kid or not isinstance(kid, str):
                continue
            refreshed[kid] = RSAAlgorithm.from_jwk(json.dumps(jwk))

        self._keys_by_kid = refreshed
        self._last_refresh_monotonic = time.monotonic()


@dataclass(frozen=True, slots=True)
class TokenVerifier:
    issuer: str
    signing_key_provider: SigningKeyProvider

    def verify(self, token: str) -> dict[str, Any]:
        try:
            headers = jwt.get_unverified_header(token)
            key = self.signing_key_provider.get_key(headers)
            claims = jwt.decode(
                token,
                key=key,
                algorithms=list(self.signing_key_provider.algorithms),
                issuer=self.issuer,
                options={"verify_aud": False, "require": ["exp", "iss"]},
            )
        except (PyJwtInvalidTokenError, InvalidTokenError, ValueError) as exc:
            raise InvalidTokenError(str(exc)) from exc
        if not isinstance(claims, dict):
            raise InvalidTokenError("Invalid claims")
        return claims
