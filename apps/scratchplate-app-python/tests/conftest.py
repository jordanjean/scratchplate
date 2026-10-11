from __future__ import annotations

import pytest
from fastapi.testclient import TestClient

from scratchplate_app.infrastructure.app_factory import build_apps
from scratchplate_app.infrastructure.security import StaticSigningKeyProvider, TokenVerifier
from scratchplate_app.infrastructure.settings import Settings
from tests.support import TEST_ISSUER, TEST_SECRET


@pytest.fixture()
def settings() -> Settings:
    return Settings(
        server_port=8080,
        management_server_port=8081,
        jwt_issuer_uri=TEST_ISSUER,
        jwt_jwk_set_uri="http://jwks.example.test/certs",
    )


@pytest.fixture()
def token_verifier(settings: Settings) -> TokenVerifier:
    return TokenVerifier(issuer=settings.jwt_issuer_uri, signing_key_provider=StaticSigningKeyProvider(TEST_SECRET))


@pytest.fixture()
def main_client(settings: Settings, token_verifier: TokenVerifier) -> TestClient:
    _, app, _ = build_apps(settings, token_verifier)
    with TestClient(app) as client:
        yield client


@pytest.fixture()
def management_client(settings: Settings, token_verifier: TokenVerifier) -> TestClient:
    _, _, management_app = build_apps(settings, token_verifier)
    assert management_app is not None
    with TestClient(management_app) as client:
        yield client
