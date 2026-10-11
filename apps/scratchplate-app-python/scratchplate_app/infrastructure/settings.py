from __future__ import annotations

import os
from dataclasses import dataclass

DEFAULT_SERVER_PORT = 8080
DEFAULT_JWT_ISSUER_URI = "http://localhost:32000/realms/scratchplate"
DEFAULT_JWT_JWK_SET_URI = "http://localhost:32000/realms/scratchplate/protocol/openid-connect/certs"
DEFAULT_WELCOME_ADMIN_TEMPLATE = (
    "Welcome {username}! You are signed in as an administrator: you have full access to the platform."
)
DEFAULT_WELCOME_USER_TEMPLATE = "Welcome {username}! You are signed in as a regular user."


@dataclass(frozen=True, slots=True)
class Settings:
    server_port: int = DEFAULT_SERVER_PORT
    management_server_port: int = DEFAULT_SERVER_PORT
    jwt_issuer_uri: str = DEFAULT_JWT_ISSUER_URI
    jwt_jwk_set_uri: str = DEFAULT_JWT_JWK_SET_URI
    welcome_admin_template: str = DEFAULT_WELCOME_ADMIN_TEMPLATE
    welcome_user_template: str = DEFAULT_WELCOME_USER_TEMPLATE

    @classmethod
    def from_env(cls) -> "Settings":
        server_port = int(os.getenv("SERVER_PORT", str(DEFAULT_SERVER_PORT)))
        management_server_port = int(os.getenv("MANAGEMENT_SERVER_PORT", str(server_port)))
        return cls(
            server_port=server_port,
            management_server_port=management_server_port,
            jwt_issuer_uri=os.getenv("JWT_ISSUER_URI", DEFAULT_JWT_ISSUER_URI),
            jwt_jwk_set_uri=os.getenv("JWT_JWK_SET_URI", DEFAULT_JWT_JWK_SET_URI),
            welcome_admin_template=os.getenv(
                "SCRATCHPLATE_WELCOME_ADMIN_TEMPLATE", DEFAULT_WELCOME_ADMIN_TEMPLATE
            ),
            welcome_user_template=os.getenv("SCRATCHPLATE_WELCOME_USER_TEMPLATE", DEFAULT_WELCOME_USER_TEMPLATE),
        )
